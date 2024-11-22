/*
	As I decided to keep all parameters for vertices borrowed, I decided to do the same here for consistency.
*/


use super::{ materials::{ MultiMaterial, SimpleColorMaterial }, Face, Material, Vertex, VertexMath };
use std::error::Error;

pub struct Mesh {
	vertices:Vec<Vertex>,
	faces:Vec<Face>,
	materials:MultiMaterial
}
impl Mesh {

	/* CONSTRUCTOR METHODS */

	/// Create a new mesh from raw vertex and face data.
	pub fn raw(vertices:Vec<Vertex>, face_data:Vec<[usize; 3]>) -> Mesh {
		Mesh {
			vertices,
			faces: face_data.iter().map(|face_data| Face::new(*face_data)).collect::<Vec<Face>>(),
			materials: MultiMaterial::new(vec![([0, face_data.len()], &SimpleColorMaterial::default())])
		}
	}

	/// Create an empty mesh.
	pub fn empty() -> Mesh {
		Self::raw(Vec::new(), Vec::new())
	}

	/// Create a new mesh from an obj file/string.
	pub fn from_obj(obj_contents:&str) -> Result<Mesh, Box<dyn Error>> {
		use std::fs::{ metadata, read_to_string };
		
		let obj_contents:String = obj_contents.replace('\r', "");

		// Try to read from file if exist.
		if obj_contents.to_lowercase().contains(".obj") &&  metadata(&obj_contents).is_ok() {
			if let Ok(file_contents) = read_to_string(&obj_contents) {
				if let Ok(mesh) = Mesh::from_obj(&file_contents) {
					return Ok(mesh);
				}
			}
		}

		// Try to create from string.
		let mut object_count_offset:usize = 0;
		let mut vertices:Vec<Vertex> = Vec::new();


		let mut faces:Vec<[usize; 3]> = Vec::new();
		let relevant_lines:Vec<&str> = obj_contents.split('\n').filter(|line| !line.starts_with('#')).collect::<Vec<&str>>();
		for line in &relevant_lines {
			let params:Vec<&str> = line.split(' ').collect::<Vec<&str>>();

			// Object.
			if params[0] == "o" {
				object_count_offset = vertices.len(); // All face data will point to the index of the vertex in thea active object, adding this value will point it to the index of the vertex of the file.
			}
			
			// Face.
			if params[0] == "f" {
				if params.len() == 4 {
					let face_data:Vec<usize> = (0..3_usize).map(|index| params[index + 1].split('/').map(|val_str| val_str.parse::<usize>().expect("Could not create an integer value from face material source.") - 1).collect::<Vec<usize>>()[0]).collect::<Vec<usize>>();
					faces.push([face_data[0] - object_count_offset, face_data[1] - object_count_offset, face_data[2] - object_count_offset]);
				} else {
					return Err(format!("ERROR: Could not create a face from a set of {} coordinates. Please triangulate your model.", params.len() - 1).into());
				}
			}
			
			// Vertex.
			if params[0] == "v" {
				vertices.push(Self::floats_from_obj_params(&params, 3, "vertex").try_into().unwrap());
			}
		}

		Ok(Mesh::raw(vertices, faces))
	}

	/// Extract a number of floats from a string in a OBJ file.
	fn floats_from_obj_params(params:&[&str], param_count:usize, error_source_name:&str) -> Vec<f32> {
		let mut output:Vec<f32> = Vec::new();
		for index in 0..param_count {
			if params.len() > param_count {
				if let Ok(value) = params[index + 1].parse::<f32>() {
					output.push(value);
				} else {
					println!("ERROR: Could not create a float value from {} in {} source. Skipping value.", params[index + 1], error_source_name);
					output.push(0.0);
				}
			} else {
				println!("ERROR: Could not create {} float values from {} parameters in {} source. Skipping value.", param_count, params.len(), error_source_name);
				output.push(0.0);
			}
		}
		output
	}



	/* BUILDER METHODS */

	/// Combine this mesh with another one.
	pub fn combine_with(&mut self, addition:&Mesh) {
		let vertex_shift:usize = self.vertices.len();
		let face_shift:usize = self.faces.len();

		// Adapt faces.
		let shifted_faces:Vec<Face> = addition.faces().iter().map(|face| face.vertex_indexes()).map(|indexes| Face::new([indexes[0] + vertex_shift, indexes[1] + vertex_shift, indexes[2] + vertex_shift])).collect::<Vec<Face>>();

		// Combine materials.
		for material in addition.materials.materials() {
			let face_range:[usize; 2] = material.face_range();
			self.materials.add([face_range[0] + face_shift, face_range[1] + face_shift], material.material());
		}

		// Add faces and vertices to current.
		self.vertices.extend_from_slice(&addition.vertices);
		self.faces.extend_from_slice(&shifted_faces);
	}

	/// Return a clone combined with another mesh with another one.
	pub fn combined_with(&self, addition:&Mesh) -> Mesh {
		let mut clone:Mesh = self.clone();
		clone.combine_with(addition);
		clone
	}



	/* MATERIAL METHODS */

	/// Set the material for the entire the mesh.
	pub fn set_material(&mut self, material:&dyn Material) {
		self.materials = MultiMaterial::new(vec![([0, self.faces.len()], material)])
	}

	/// Return a clone of self with multiple material.
	pub fn with_materials(&self, materials:Vec<([usize; 2], &dyn Material)>) -> Mesh {
		let mut clone:Mesh = self.clone();
		clone.materials = MultiMaterial::new(materials);
		clone
	}

	/// Return a clone of self with a material.
	pub fn with_material(&self, material:&dyn Material) -> Mesh {
		self.with_materials(vec![([0, self.faces.len()], material)])
	}

	/// Add a new material.
	pub fn add_material(&mut self, vertex_range:[usize; 2], material:&dyn Material) {
		self.materials.add(vertex_range, material)
	}

	/// Add multiple new materials.
	pub fn add_materials(&mut self, materials:Vec<(usize, usize, &dyn Material)>) {
		for material in &materials {
			self.add_material([material.0, material.1], material.2);
		}
	}



	/* PROPERTY GETTERS */

	/// Get the vertices of the mesh.
	pub fn vertices(&self) -> &Vec<Vertex> {
		&self.vertices
	}

	/// Get the faces of the mesh.
	pub fn faces(&self) -> &Vec<Face> {
		&self.faces
	}

	/// Get the materials of the mesh.
	pub fn materials(&self) -> &MultiMaterial {
		&self.materials
	}



	/* MOTION METHODS */

	/// Return a displaced version of the vertex.
	pub fn displaced(&self, offset:&Vertex) -> Mesh {
		let mut new_mesh:Mesh = self.clone();
		new_mesh.displace(offset);
		new_mesh
	}

	/// Displace this vertex.
	pub fn displace(&mut self, offset:&Vertex) {
		self.vertices.iter_mut().for_each(|vert| vert.displace(offset));
	}

	/// Return a rotated version of the current vertex. Rotation is in degrees for use-friendliness.
	pub fn rotated(&self, rotation:&[f32; 3], anchor_pos:&Option<[f32; 3]>) -> Mesh {
		let mut new_mesh:Mesh = self.clone();
		new_mesh.rotate(rotation, anchor_pos);
		new_mesh
	}

	/// Rotate this vertex in-place. Rotation is in degrees for use-friendliness.
	pub fn rotate(&mut self, rotation:&[f32; 3], anchor_pos:&Option<[f32; 3]>) {
		let rotation_dataset:(f32, [f32; 3]) = [0.0; 3].create_rotation_dataset(&[rotation[0].to_radians(), rotation[1].to_radians(), rotation[2].to_radians()]);
		self.vertices.iter_mut().for_each(|vertex| vertex.rotate_using_dataset(&rotation_dataset, anchor_pos));
	}

	/// Return a euler rotated version of the current vertex. Rotation is in degrees for use-friendliness.
	pub fn euler_rotated(&self, rotation:&[f32; 3], anchor_pos:&Option<[f32; 3]>) -> Mesh {
		let mut new_mesh:Mesh = self.clone();
		new_mesh.euler_rotate(rotation, anchor_pos);
		new_mesh
	}

	/// Euler rotate this vertex in-place. Rotation is in degrees for use-friendliness.
	pub fn euler_rotate(&mut self, rotation:&[f32; 3], anchor_pos:&Option<[f32; 3]>) {
		use std::f32::consts::PI;

		let scale:f32 = 1.0 / 180.0 * PI;
		let rotation_radians:[f32; 3] = [rotation[0] * scale, rotation[1] * scale, rotation[2] * scale];
		self.vertices.iter_mut().for_each(|vert| vert.euler_rotate(&rotation_radians, anchor_pos));
	}

	/// Return a scaled version of the vertex.
	pub fn scaled(&self, scale:&[f32; 3]) -> Mesh {
		let mut new_mesh:Mesh = self.clone();
		new_mesh.scale(scale);
		new_mesh
	}

	/// Scale the mesh.
	pub fn scale(&mut self, scale:&[f32; 3]) {
		self.vertices.iter_mut().for_each(|vert: &mut [f32; 3]| vert.scale(scale));
	}
}

impl Clone for Mesh {
	fn clone(&self) -> Mesh {
		Mesh {
			vertices: self.vertices.clone(),
			faces: self.faces.clone(),
			materials: self.materials.clone()
		}
	}
}