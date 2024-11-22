use super::super::model::Vertex;

pub trait Lens {

	/// Wrap the lens in a box.
	fn boxify(&self) -> Box<dyn Lens>;

	/// Map the vertices from their 3D unprocessed locations to locations relative to the perspective of the lens.
	fn warp_vertices(&self, vertices:Vec<Vertex>) -> Vec<Vertex>;
}