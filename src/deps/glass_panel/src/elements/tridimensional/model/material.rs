use super::Vertex;

pub trait Material {

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Material>;

	/// Return a colored version of the vertex.
	fn color_vertex(&self, face_index:usize, vertex:Vertex) -> ColoredVertex;

	/// Return a colored version of each vertex.
	fn color_vertices(&self, face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex> {
		vertices.iter().map(|vertex| self.color_vertex(face_index, *vertex)).collect::<Vec<ColoredVertex>>()
	}

	/// Return a colored version of each vertex in an edge.
	fn color_edge(&self, face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex>;

	/// Return a colored version of each vertex in a face.
	fn color_face(&self, face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex>;
}

#[derive(Clone)]
pub struct ColoredVertex {
	pub vertex:Vertex,
	pub color:u32
}
pub trait VertexColoring {
	fn colored(&self, color:u32) -> ColoredVertex;
}
impl VertexColoring for Vertex {
	fn colored(&self, color:u32) -> ColoredVertex {
		ColoredVertex { vertex: *self, color }
	}
}