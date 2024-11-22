use super::{ model::{ ColoredVertex, Face, Material, Mesh, Vertex, VertexMath }, Entity, Lens };
use crate::{ Drawable, DrawableData, DrawBuffer };
use std::{error::Error, usize};

static VIRTUAL_CAMERA_ENTITY_NAME:&str = "VIRTUAL_CAMERA";
pub static X_AXIS:usize = 0; // The index of the 3D axis that is mapped to the 2D image's X axis.
pub static Y_AXIS:usize = 2; // The index of the 3D axis that is mapped to the 2D image's Y axis.
pub static D_AXIS:usize = 1; // The index of the 3D axis that is mapped to the 2D image's depth.

// Obviously not the final structure, but allows me to draw meshes for now.
pub struct Scene {
	data_set:DrawableData,
	entities:Vec<Entity>,
	lens:Box<dyn Lens>,
	camera:Entity
}
impl Scene {

	/* CONSTRUCTOR METHODS */

	/// Create a new Scene.
	pub fn new(width:usize, height:usize, lens:&dyn Lens, entities:Vec<Entity>) -> Scene {
		Scene {
			data_set: DrawableData::new::<usize>(vec![
				("width", width),
				("height", height)
			], vec![]),
			entities,
			lens: lens.boxify(),
			camera: Entity::new(VIRTUAL_CAMERA_ENTITY_NAME, Mesh::empty())
		}
	}



	/* ENTITY METHODS */

	/// Get a reference to all entities.
	pub fn entities(&self) -> &Vec<Entity> {
		&self.entities
	}

	/// Get a mutable reference to all entities. Assumes a modification will take place, so updates the datasets cache.
	pub fn entities_mut(&mut self) -> &mut Vec<Entity> {
		self.data_set_mut();
		&mut self.entities
	}

	/// Get a specific entity by name.
	pub fn entity_by_name(&self, name:&str) -> Option<&Entity> {
		for entity in &self.entities {
			if entity.get_name() == name {
				return Some(entity);
			}
			if let Some(child) = entity.child_by_name(name) {
				return Some(child);
			}
		}
		None
	}

	/// Get a specific mutable entity by name.
	pub fn entity_by_name_mut(&mut self, name:&str) -> Option<&mut Entity> {
		for entity in &mut self.entities {
			if entity.get_name() == name {
				return Some(entity);
			}
			if let Some(child) = entity.child_by_name_mut(name) {
				return Some(child);
			}
		}
		None
	}

	/// Add a new entity.
	pub fn add_entity(&mut self, entity:Entity) {
		self.entities.push(entity);
	}

	/// Remove an entity based on its name.
	pub fn remove_entity_by_name(&mut self, name:&str) {
		for entity_index in (0..self.entities.len()).rev() {
			self.entities[entity_index].remove_child_by_name(name);
			if self.entities[entity_index].get_name() == name {
				self.entities.remove(entity_index);
			}
		}
	}

	/// Get a mutable reference to the virtual camera.
	pub fn camera(&mut self) -> &mut Entity {
		&mut self.camera
	}
	


	/* DRAWING METHODS */

	/// Bump a list of vertices closer to the camera.
	fn bumped(vertices:&[Vertex], bump:f32) -> Vec<Vertex> {
		vertices.iter().map(|vertex| {
			let mut vertex:Vertex = *vertex;
			vertex[D_AXIS] -= bump;
			vertex
		}).collect::<Vec<Vertex>>()
	}

	/// List all coordinates in a line between 2 vertices.
	fn coords_in_line(start:&Vertex, end:&Vertex, step_size:f32) -> Vec<Vertex> {

		// Calculate modifier data.
		let distance_x:f32 = start[X_AXIS].max(end[X_AXIS]) - start[X_AXIS].min(end[X_AXIS]);
		let distance_y:f32 = start[Y_AXIS].max(end[Y_AXIS]) - start[Y_AXIS].min(end[Y_AXIS]);
		let distance_d:f32 = start[D_AXIS].max(end[D_AXIS]) - start[D_AXIS].min(end[D_AXIS]);
		let line_distance:f32 = (distance_x.abs().powi(2) + distance_y.abs().powi(2)).sqrt(); // Do not take the 'depth' into account for the distance of the line.
		if line_distance == f32::INFINITY || line_distance == 0.0 { return Vec::new(); }
		let steps:f32 = (line_distance / step_size).abs();
		let modification:Vertex = [end[0] - start[0], end[1] - start[1], end[2] - start[2]];
		let modification_step:Vertex = [modification[0] / steps, modification[1] / steps, modification[2] / steps];
		
		// If the start or end vertices have a D_AXIS less than 0.0, find a new coordinate to line to.
		if start[D_AXIS] < 0.0 {
			let cutoff_part:f32 = 1.0 / distance_d * start[D_AXIS].abs();
			let mut new_start:[f32; 3] = [0.0; 3];
			new_start[X_AXIS] = start[X_AXIS] + modification[X_AXIS] * cutoff_part;
			new_start[Y_AXIS] = start[Y_AXIS] + modification[Y_AXIS] * cutoff_part;
			new_start[D_AXIS] = 0.0;
			return Self::coords_in_line(&new_start, end, step_size);
		}
		if end[D_AXIS] < 0.0 {
			let cutoff_part:f32 = 1.0 / distance_d * end[D_AXIS].abs();
			let mut new_end:[f32; 3] = [0.0; 3];
			new_end[X_AXIS] = end[X_AXIS] - modification[X_AXIS] * cutoff_part;
			new_end[Y_AXIS] = end[Y_AXIS] - modification[Y_AXIS] * cutoff_part;
			new_end[D_AXIS] = 0.0;
			return Self::coords_in_line(start, &new_end, step_size);
		}

		// Loop through coordinates.
		let mut vertex:Vertex = *start;
		let mut vertices:Vec<Vertex> = vec![*start];
		for _ in 0..steps.floor() as usize {
			vertex.displace(&modification_step);
			vertices.push(vertex);
		}

		// Return list.
		vertices
	}

	/// Draw a pixel on the 3D buffer.
	fn coords_in_face(edges:&Vec<Vec<Vertex>>, bounding_rect:[f32; 4]) -> Vec<Vertex> {

		// Find for each y coordinate the smallest and largest x value in the edges.
		let mut range_per_row:Vec<[Vertex; 2]> = vec![[[f32::MAX; 3], [f32::MIN; 3]]; (bounding_rect[3] - bounding_rect[1] + 1.0).ceil() as usize];
		for edge in edges {
			let rounded_edge:Vec<Vertex> = edge.iter().map(|vertex| [vertex[0].round(), vertex[1].round(), vertex[2].round()]).collect::<Vec<[f32; 3]>>();
			for vertex in &rounded_edge {
				let row:usize = (vertex[Y_AXIS] - bounding_rect[1]).floor() as usize;
				if row < range_per_row.len() {
					if vertex[X_AXIS] < range_per_row[row][0][X_AXIS] { range_per_row[row][0] = *vertex; }
					if vertex[X_AXIS] > range_per_row[row][1][X_AXIS] { range_per_row[row][1] = *vertex; }
				}
			}
		}

		// For each y coordinate, create a line from the outer 2 x coordinates found earlier to create each layer of the face.
		range_per_row.iter().flat_map(|range| Self::coords_in_line(&range[0], &range[1], 1.0)).collect::<Vec<Vertex>>()
	}

	/// Create a flat bounding rect for a list of faces. Returns 3D vertices, but only checks X_AXIS and Y_AXIS axes for bounding.
	fn create_bounding_rect(vertices:&Vec<&Vertex>) -> [f32; 4] {
		let mut bounding_rect:[f32; 4] = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
		for vertex in vertices {
			if vertex[X_AXIS] < bounding_rect[0] { bounding_rect[0] = vertex[X_AXIS]; }
			if vertex[Y_AXIS] < bounding_rect[1] { bounding_rect[1] = vertex[Y_AXIS]; }
			if vertex[X_AXIS] > bounding_rect[2] { bounding_rect[2] = vertex[X_AXIS]; }
			if vertex[Y_AXIS] > bounding_rect[3] { bounding_rect[3] = vertex[Y_AXIS]; }
		}
		bounding_rect
	}

	/// Create a buffer from the colored vertices list.
	fn create_buffer(colored_vertices:&Vec<ColoredVertex>, scene_size:&[usize; 2]) -> DrawBuffer {
		let scene_size_f32:[f32; 2] = [scene_size[0] as f32, scene_size[1] as f32];
		
		let mut buffer:Vec<u32> = vec![0; scene_size[0] * scene_size[1]];
		for colored_vertex in colored_vertices {

			// Only draw vertices that fit on screen.
			let location:&Vertex = &colored_vertex.vertex;
			if location[X_AXIS] >= 0.0 && location[Y_AXIS] >= 0.0 && location[X_AXIS] < scene_size_f32[0] && location[Y_AXIS] < scene_size_f32[1] - 1.0 {

				// Draw the vertex if the spot is not taken.
				let index:usize = (scene_size[1] - (location[Y_AXIS] as usize + 1)) * scene_size[0] + location[X_AXIS] as usize;
				if buffer[index] == 0 {
					buffer[index] = colored_vertex.color;
				}
			}
		}
		DrawBuffer::new(buffer, scene_size[0], scene_size[1])
	}
}
impl Drawable for Scene {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("tridimensional_scene")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Scene {
			data_set: self.data_set().clone(),
			entities: self.entities.to_vec(),
			lens: (*self.lens).boxify(),
			camera: self.camera.clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[0, 0, self.data_set().get_setting_value_or::<usize>("width", 0), self.data_set().get_setting_value_or::<usize>("height", 0)]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {

		// Drawing settings.
		let bump_face:f32 = 0.0; // Vertices should be drawn on top of edges, which should be drawn on top of faces, so bump them up towards the camera. TODO: Find better way, fixed bump is not reliable far away and close
		let bump_edge:f32 = 1.0;
		let bump_vert:f32 = 2.0;

		// Scene size settings.
		let scene_position:[usize; 4] = self.position();
		let scene_size:[usize; 2] = [scene_position[2], scene_position[3]];
		let min_render_distance:f32 = 20.0;

		// Get the offset created by the position and rotation of the camera.
		let camera_position:Vertex = self.camera.get_position().negative();
		let camera_rotation_offset:[f32; 3] = self.camera.get_rotation().negative();

		// Combine all vertices and meshes.
		let mut scene_mesh:Mesh = Mesh::empty();
		for entity in &self.entities {
			scene_mesh.combine_with(&entity.processed_mesh());
		}

		// Displace scene mesh to camera.
		scene_mesh.displace(&camera_position);
		scene_mesh.rotate(&camera_rotation_offset, &None);

		// Adapt scene vertices to lens perspective.
		let mut scene_vertices:Vec<Vertex> = scene_mesh.vertices().to_vec();
		scene_vertices = self.lens.warp_vertices(scene_vertices);

		// Move center of camera to center of window. Do this after mapping scene to window coordinates to simplify the formula there by keeping [0.0; 3] the center.
		let mut scene_center_offset:[f32; 3] = [0.0; 3];
		scene_center_offset[X_AXIS] = scene_size[0] as f32 * 0.5;
		scene_center_offset[Y_AXIS] = scene_size[1] as f32 * 0.5;
		scene_vertices.iter_mut().for_each(|vertex| vertex.displace(&scene_center_offset));


		// Find vertices for each edge and face.
		let mut colored_scene_vertices:Vec<ColoredVertex> = Vec::new();
		let scene_faces:&Vec<Face> = scene_mesh.faces();
		let scene_materials:&dyn Material = scene_mesh.materials();
		for (face_index, face) in scene_faces.iter().enumerate() {
			let face_vertices:Vec<&Vertex> = face.vertex_indexes().iter().map(|vertex_index| &scene_vertices[*vertex_index]).collect::<Vec<&Vertex>>();

			// Calculate all vertices required to draw the edges, but validate the bounding rect before adding them to the color list..
			let edges:Vec<Vec<Vertex>> = (0..face_vertices.len()).map(|edge_index| Self::coords_in_line(face_vertices[edge_index], face_vertices[(edge_index + 1) % face_vertices.len()], 1.0)).collect::<Vec<Vec<Vertex>>>();
			
			// Calculate the bounding rectangle of the face on the drawing plane.
			let bounding_rect:[f32; 4] = Self::create_bounding_rect(&face_vertices);
			
			// Skip if the bounding rect falls out of the scene rendering rect.
			if bounding_rect[0] > scene_size[0] as f32 || bounding_rect[1] > scene_size[1] as f32 || bounding_rect[2] < 0.0 || bounding_rect[3] < 0.0 || face_vertices.iter().filter(|vertex| vertex[D_AXIS] > 0.0).count() == 0 {
				continue;
			}

			// Add the vertices to the colored vertices.
			let bumped_vertices:Vec<Vertex> = Self::bumped(&face_vertices.iter().map(|vertex| **vertex).collect::<Vec<Vertex>>(), bump_vert);
			let colored_vertices:Vec<ColoredVertex> = bumped_vertices.iter().map(|vertex| scene_materials.color_vertex(face_index, *vertex)).collect::<Vec<ColoredVertex>>();
			colored_scene_vertices.extend_from_slice(&colored_vertices);
			
			// Add the edge to the colored vertices.
			let bumped_edges:Vec<Vertex> = Self::bumped(&edges.iter().flatten().copied().collect::<Vec<Vertex>>(), bump_edge);
			let colored_edges:Vec<ColoredVertex> = scene_materials.color_edge(face_index, bumped_edges);
			colored_scene_vertices.extend_from_slice(&colored_edges);

			// Add the face to the colored vertices.
			let face_vertices:Vec<Vertex> = Self::coords_in_face(&edges, bounding_rect);
			let bumped_faces:Vec<Vertex> = Self::bumped(&face_vertices, bump_face);
			let colored_face_vertices:Vec<ColoredVertex> = scene_materials.color_face(face_index, bumped_faces);
			colored_scene_vertices.extend_from_slice(&colored_face_vertices);
		}

		// Remove vertices behind the camera.
		colored_scene_vertices = colored_scene_vertices.iter().filter(|colored_vertex| colored_vertex.vertex[D_AXIS] > min_render_distance).cloned().collect::<Vec<ColoredVertex>>();

		// Round all axes that will be used to determine their position in the buffer.
		colored_scene_vertices.iter_mut().for_each(|colored_vertex| {
			colored_vertex.vertex[X_AXIS] = colored_vertex.vertex[X_AXIS].round();
			colored_vertex.vertex[Y_AXIS] = colored_vertex.vertex[Y_AXIS].round();
		});

		// Sort vertices from near to far away to minimize writing operations.
		colored_scene_vertices.sort_by(|a, b| a.vertex[D_AXIS].partial_cmp(&b.vertex[D_AXIS]).unwrap());

		// Create the buffer.
		Self::create_buffer(&colored_scene_vertices, &scene_size)
	}

	

	/* DATA SET METHODS */

	/// Get a reference to the instances Data object.
	fn data_set(&self) -> &DrawableData {
		&self.data_set
	}
	
	/// Get a mutable reference to the instances Data object.
	fn data_set_mut(&mut self) -> &mut DrawableData {
		&mut self.data_set
	}



	/* SPECIFIC SCENARIO METHODS */

	/// If the drawable is a 3D scene, fetch it.
	fn as_scene(&mut self) -> Result<&mut super::Scene, Box<dyn Error>> {
		Ok(self as &mut Scene)
	}
}