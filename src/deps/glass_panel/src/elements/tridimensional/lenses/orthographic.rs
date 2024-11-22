use super::super::model::Vertex;
use super::super::Lens;

pub struct OrthographicLens {
}
impl Default for OrthographicLens {
    fn default() -> Self {
        Self::new()
    }
}

impl OrthographicLens {
	
	/// Create a new OrthographicLens lens.
	pub fn new() -> OrthographicLens {
		OrthographicLens {
		}
	}
}
impl Lens for OrthographicLens {

	/// Wrap the lens in a box.
	fn boxify(&self) -> Box<dyn Lens> {
		Box::new(OrthographicLens::new())
	}
	
	/// Map the vertices from their 3D unprocessed locations to locations relative to the perspective of the lens.
	fn warp_vertices(&self, vertices:Vec<Vertex>) -> Vec<Vertex> {
		vertices.to_vec()
	}
}