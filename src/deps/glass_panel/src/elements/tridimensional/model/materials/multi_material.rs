use super::super::{ Material, Vertex, VertexColoring, ColoredVertex };

#[derive(Clone)]
pub struct MultiMaterial {
	materials:Vec<SplitMaterial>
}
impl MultiMaterial {

	/// Create a new material with simple colors.
	pub fn new(split_materials:Vec<([usize; 2], &dyn Material)>) -> MultiMaterial {
		MultiMaterial {
			materials: split_materials.iter().map(|split_material| SplitMaterial {
				face_range: split_material.0,
				material: split_material.1.boxify()
			}).collect::<Vec<SplitMaterial>>()
		}
	}

	/// Create an empty new multimaterial.
	pub fn empty() -> MultiMaterial {
		MultiMaterial::new(Vec::new())
	}

	/// Add a new split material.
	pub fn add(&mut self, vertex_range:[usize; 2], material:&dyn Material) {
		self.materials.push(SplitMaterial {
			face_range: vertex_range,
			material: material.boxify()
		});
	}

	/// Get a reference to all split materials.
	pub fn materials(&self) -> Vec<&SplitMaterial> {
		self.materials.iter().collect::<Vec<&SplitMaterial>>()
	}

	/// Find the correct material for a specific vertex index.
	fn material_for_face(&self, index:usize) -> Option<&dyn Material> {
		let matching_materials:Vec<&dyn Material> = self.materials.iter().filter(|split_material| index >= split_material.face_range[0] && index < split_material.face_range[1]).map(|split_material| &*split_material.material).collect::<Vec<&dyn Material>>();
		if !matching_materials.is_empty() {
			Some(matching_materials[0])
		} else {
			None
		}
	}
}
impl Material for MultiMaterial {

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Material> {
		Box::new(MultiMaterial {
			materials: self.materials.clone()
		})
	}

	/// Return a colored version of the vertex.
	fn color_vertex(&self, face_index:usize, vertex:Vertex) -> ColoredVertex {
		match self.material_for_face(face_index) {
			Some(material) => material.color_vertex(face_index, vertex),
			None => vertex.colored(0x00000000)
		}
	}
	
	/// Return a colored version of each vertex.
	fn color_vertices(&self, face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex> {
		match self.material_for_face(face_index) {
			Some(material) => material.color_vertices(face_index, vertices),
			None => vertices.iter().map(|vertex| vertex.colored(0x00000000)).collect::<Vec<ColoredVertex>>()
		}
	}

	/// Return a colored version of each vertex in an edge.
	fn color_edge(&self, face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex> {
		match self.material_for_face(face_index) {
			Some(material) => material.color_edge(face_index, vertices),
			None => vertices.iter().map(|vertex| vertex.colored(0x00000000)).collect::<Vec<ColoredVertex>>()
		}
	}

	/// Return a colored version of each vertex in a face.
	fn color_face(&self, face_index:usize, vertices:Vec<Vertex>) -> Vec<ColoredVertex> {
		match self.material_for_face(face_index) {
			Some(material) => material.color_face(face_index, vertices),
			None => vertices.iter().map(|vertex| vertex.colored(0x00000000)).collect::<Vec<ColoredVertex>>()
		}
	}
}



pub struct SplitMaterial {
	face_range:[usize; 2],
	material:Box<dyn Material>
}
impl SplitMaterial {

	/// Create a new split material.
	pub fn new(face_range:[usize; 2], material:&dyn Material) -> SplitMaterial {
		SplitMaterial {
			face_range,
			material: material.boxify()
		}
	}

	/// Get the face range of the split material.
	pub fn face_range(&self) -> [usize; 2] {
		self.face_range
	}

	/// Get a reference to the face range of the split material.
	pub fn material(&self) -> &dyn Material {
		&*self.material
	}
}
impl Clone for SplitMaterial {
	fn clone(&self) -> SplitMaterial {
		SplitMaterial::new(self.face_range, &*self.material)
	}
}