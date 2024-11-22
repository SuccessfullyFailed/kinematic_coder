use super::{ StaticMesh, StaticMeshLink };
use crate::tridimensional::model::{ Mesh, Vertex, VertexMath };
use std::error::Error;

#[derive(Clone)]
pub struct Entity {
	name:String,
	mesh_link:StaticMeshLink,
	position:[f32; 3],
	rotation:[f32; 3],
	euler_rotation:[f32; 3], // Can sometimes be more useful, although less accurate.
	scale:[f32; 3],
	children:Vec<Entity>
}
impl Entity {

	/* CONSTRUCTOR METHODS */
	
	/// Create a new entity from a mesh.
	pub fn new(name:&str, mesh:Mesh) -> Entity {
		Entity {
			name: String::from(name),
			mesh_link: StaticMesh::create(&format!("RAW:{name}"), mesh),
			scale: [1.0; 3],
			position: [0.0; 3],
			rotation: [0.0; 3],
			euler_rotation: [0.0; 3],
			children: Vec::new()
		}
	}

	/// Create a new entity.
	pub fn from_obj(name:&str, file:&str) -> Result<Entity, Box<dyn Error>> {
		Ok(Entity {
			name: String::from(name),
			mesh_link: StaticMesh::from_obj(file)?,
			scale: [1.0; 3],
			position: [0.0; 3],
			rotation: [0.0; 3],
			euler_rotation: [0.0; 3],
			children: Vec::new()
		})
	}



	/* PROPERTY GETTERS */

	/// Get the name of the entity.
	pub fn get_name(&self) -> String {
		String::from(&self.name)
	}

	/// Get the position of the entity.
	pub fn get_position(&self) -> &[f32; 3] {
		&self.position
	}

	/// Get the rotation of the entity.
	pub fn get_rotation(&self) -> &[f32; 3] {
		&self.rotation
	}

	/// Get the euler rotation of the entity.
	pub fn get_euler_rotation(&self) -> &[f32; 3] {
		&self.euler_rotation
	}

	/// Get the scale of the entity.
	pub fn get_scale(&self) -> &[f32; 3] {
		&self.scale
	}



	/* CONTROL METHODS */

	/// Set a new scale to the entity.
	pub fn set_scale(&mut self, scale:&[f32; 3]) {
		self.scale = *scale;
	}
	
	/// Set a position to the entity.
	pub fn set_position(&mut self, position:&[f32; 3]) {
		self.position = *position;
	}
	
	/// Set a rotation to the entity. Rotation is in degrees for use-friendliness.
	#[allow(clippy::needless_range_loop)]
	pub fn set_rotation(&mut self, rotation:&[f32; 3]) {
		let mut rotation:[f32; 3] = *rotation;
		for axis in 0..3 {
			while rotation[axis] < 0.0 {
				rotation[axis] += 360.0;
			}
			self.rotation[axis] = rotation[axis] % 360.0;
		}
	}
	
	/// Set a euler rotation to the entity. Rotation is in degrees for use-friendliness.
	#[allow(clippy::needless_range_loop)]
	pub fn set_euler_rotation(&mut self, rotation:&[f32; 3]) {
		let mut rotation:[f32; 3] = *rotation;
		for axis in 0..3 {
			while rotation[axis] < 0.0 {
				rotation[axis] += 360.0;
			}
			self.euler_rotation[axis] = rotation[axis] % 360.0;
		}
	}



	/* MESH ADAPTER METHODS */

	/// Displace the entity in-place.
	pub fn displace(&mut self, offset:&Vertex) {
		self.position.displace(offset);
	}
	
	/// Return a displaced version of the entity.
	pub fn displaced(&self, offset:&Vertex) -> Entity {
		let mut clone:Entity = self.clone();
		clone.displace(offset);
		clone
	}

	/// Return a rotated version of the entity. Rotation is in degrees for use-friendliness.
	pub fn rotated(&self, rotation:&[f32; 3]) -> Entity {
		let mut clone:Entity = self.clone();
		clone.rotate(rotation);
		clone
	}
	
	/// Rotate the entity. Rotation is in degrees for use-friendliness.
	#[allow(clippy::needless_range_loop)]
	pub fn rotate(&mut self, rotation:&[f32; 3]) {
		for axis in 0..3 {
			self.rotation[axis] += rotation[axis];
		}
	}

	/// Return a euler rotated version of the entity. Rotation is in degrees for use-friendliness.
	pub fn euler_rotated(&self, rotation:&[f32; 3]) -> Entity {
		let mut clone:Entity = self.clone();
		clone.euler_rotate(rotation);
		clone
	}
	
	/// Euler rotate the entity. Rotation is in degrees for use-friendliness.
	#[allow(clippy::needless_range_loop)]
	pub fn euler_rotate(&mut self, rotation:&[f32; 3]) {
		for axis in 0..3 {
			self.euler_rotation[axis] += rotation[axis];
		}
	}

	/// Return a scaled version of the entity. Rotation is in degrees for use-friendliness.
	pub fn scaled(&self, scale:&[f32; 3]) -> Entity {
		let mut clone:Entity = self.clone();
		clone.scale(scale);
		clone
	}

	/// Scale the entity. Rotation is in degrees for use-friendliness.
	#[allow(clippy::needless_range_loop)]
	pub fn scale(&mut self, scale:&[f32; 3]) {
		for axis in 0..3 {
			self.scale[axis] *= scale[axis];
		}
	}



	/* CHILD METHODS */

	/// Return clone of self with a list of chilren.
	pub fn with_children(&mut self, children:Vec<Entity>) -> Entity {
		let mut clone:Entity = self.clone();
		clone.children.extend_from_slice(&children);
		clone
	}

	/// Get a specific child by name.
	pub fn child_by_name(&self, name:&str) -> Option<&Entity> {
		if self.name == name { return Some(self); }
		for child in &self.children {
			if let Some(child) = child.child_by_name(name) {
				return Some(child);
			}
		}
		None
	}

	/// Get a specific mutable child by name.
	pub fn child_by_name_mut(&mut self, name:&str) -> Option<&mut Entity> {
		if self.name == name { return Some(self); }
		for child in &mut self.children {
			if let Some(child) = child.child_by_name_mut(name) {
				return Some(child);
			}
		}
		None
	}

	/// Get a reference to all children.
	pub fn children(&self) -> &Vec<Entity> {
		&self.children
	}

	/// Get a mutable reference to all children.
	pub fn children_mut(&mut self) -> &mut Vec<Entity> {
		&mut self.children
	}

	/// Get a reference to a child by index.
	pub fn child(&self, index:usize) -> &Entity {
		&self.children[index]
	}

	/// Get a mutable reference to a child by index.
	pub fn child_mut(&mut self, index:usize) -> &mut Entity {
		&mut self.children[index]
	}

	/// Add a child to the mesh.
	pub fn add_child(&mut self, child:Entity) {
		self.children.push(child);
	}

	/// Add a child to the mesh.
	pub fn add_children(&mut self, children:Vec<Entity>) {
		for child in children {
			self.add_child(child);
		}
	}

	/// Remove all children.
	pub fn remove_children(&mut self) {
		self.children = Vec::new();
	}

	/// Remove a child by name.
	pub fn remove_child_by_name(&mut self, name:&str) {
		for child_index in (0..self.children.len()).rev() {
			if self.children[child_index].get_name() == name {
				self.children.remove(child_index);
			}
			self.children[child_index].remove_child_by_name(name);
		}
	}



	/* MESH METHODS */

	/// Get a reference to the mesh.
	pub fn mesh(&self) -> Mesh {
		self.mesh_link.get_mesh().unwrap_or(Mesh::raw(Vec::new(), Vec::new()))
	}

	/// Set a new mesh to the entity.
	pub fn set_mesh(&mut self, new_mesh:Mesh) {
		self.mesh_link = StaticMesh::force_create(&format!("RAW:{}", self.name), new_mesh);
	}

	/// Set a new obj to the entity.
	pub fn set_obj(&mut self, path:&str) -> Result<(), Box<dyn Error>> {
		self.mesh_link = StaticMesh::from_obj(path)?;
		Ok(())
	}



	/* DRAWING METHODS */

	/// Clone the source mesh and scale, position and rotate it to match the entity settings. Rotation is in degrees for use-friendliness.
	pub fn processed_mesh(&self) -> Mesh {
		let mut output_mesh:Mesh = self.mesh();

		// Process children.
		for child in &self.children {
			let processed_mesh:Mesh = child.processed_mesh();
			if !processed_mesh.vertices().is_empty() {
				output_mesh.combine_with(&processed_mesh);
			}
		}

		// Process self.
		if self.scale    != [0.0; 3] { output_mesh.scale(&self.scale); }
		if self.rotation != [0.0; 3] { output_mesh.rotate(&self.rotation, &None); }
		if self.euler_rotation != [0.0; 3] { output_mesh.euler_rotate(&self.euler_rotation, &None); }
		if self.position != [0.0; 3] { output_mesh.displace(&self.position); }

		// Return object.
		output_mesh
	}
}