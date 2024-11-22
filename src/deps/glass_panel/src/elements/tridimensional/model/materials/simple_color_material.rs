use super::super::{ Material, Vertex, ColoredVertex, VertexColoring };

pub struct SimpleColorMaterial {
	vertex_color:u32,
	edge_color:u32,
	face_color:u32
}
impl SimpleColorMaterial {

	/// Create a new material with simple colors.
	pub fn new(vertex_color:u32, edge_color:u32, face_color:u32) -> SimpleColorMaterial {
		SimpleColorMaterial {
			vertex_color,
			edge_color,
			face_color
		}
	}
}
impl Material for SimpleColorMaterial {

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Material> {
		Box::new(SimpleColorMaterial::new(self.vertex_color, self.edge_color, self.face_color))
	}

	/// Return a colored version of the vertex.
	fn color_vertex(&self, _face_index:usize, vertex:Vertex) -> ColoredVertex {
		ColoredVertex { vertex, color: self.vertex_color }
	}

	/// Return a colored version of each vertex in an edge.
	fn color_edge(&self, _face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex> {
		vertices.iter().map(|vertex| vertex.colored(self.edge_color)).collect::<Vec<ColoredVertex>>()
	}

	/// Return a colored version of each vertex in a face.
	fn color_face(&self, _face_index:usize, vertices:Vec<Vertex>)  -> Vec<ColoredVertex> {
		vertices.iter().map(|vertex| vertex.colored(self.face_color)).collect::<Vec<ColoredVertex>>()
	}
}

impl Default for SimpleColorMaterial {
	fn default() -> Self {
		SimpleColorMaterial::new(0xFFFF0000, 0xFFFFFF00, 0xFF333333)
	}
}