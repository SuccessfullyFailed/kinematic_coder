#[derive(Clone, Debug, PartialEq)]
pub struct Face {
	vertex_indexes:[usize; 3]
}
impl Face {

	/// Create a new face from an array of vector ids.
	pub fn new(vertex_indexes:[usize; 3]) -> Face {
		Face {
			vertex_indexes
		}
	}

	/// Get a reference to the indexes of the linked vectors.
	pub fn vertex_indexes(&self) -> &[usize; 3] {
		&self.vertex_indexes
	}

	/// Get a mutable reference to the indexes of the linked vectors.
	pub fn vertex_indexes_mut(&mut self) -> &mut [usize; 3] {
		&mut self.vertex_indexes
	}
}