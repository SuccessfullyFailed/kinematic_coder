#[cfg(test)]
mod test {
	use crate::tridimensional::model::{ Vertex, Face, Mesh };
	use crate::_unit_testing::support::TEST_CUBE_LOCATION;

	#[test]
	fn tridimensional_mesh_create_raw() {

		// Create.
		let vertices:Vec<Vertex> = (0..8).map(|index| index as f32).map(|index| [index * 10.0, index * -5.0, index * 1.0]).collect::<Vec<[f32; 3]>>();
		let faces:Vec<[usize; 3]> = (0..3).map(|index| [index * 9 % 8, index * 6 % 8, index * 2 % 8]).collect::<Vec<[usize; 3]>>();
		let mesh:Mesh = Mesh::raw(vertices.clone(), faces.clone());

		// Compare.
		assert_eq!(mesh.vertices(), &vertices);
		assert_eq!(mesh.faces(), &faces.iter().map(|face_data| Face::new(*face_data)).collect::<Vec<Face>>());
	}

	#[test]
	fn tridimensional_mesh_load_default_cube() {
		
		let cube:Mesh = Mesh::from_obj(TEST_CUBE_LOCATION).unwrap_or_else(|_| panic!("Could not load default cube at '{TEST_CUBE_LOCATION}'"));

		// Set up expected data.
		let expected_vertices:Vec<Vertex> = vec![
			[-20.000000, -20.000000, 20.000000],
			[-20.000000, 20.000000, 20.000000],
			[-20.000000, -20.000000, -20.000000],
			[-20.000000, 20.000000, -20.000000],
			[20.000000, -20.000000, 20.000000],
			[20.000000, 20.000000, 20.000000],
			[20.000000, -20.000000, -20.000000],
			[20.000000, 20.000000, -20.000000]
		];
		let mut found_vertices:Vec<bool> = vec![false; expected_vertices.len()];

		// Loop through loaded vertices.
		for vertex in cube.vertices() {
			let index_in:usize = expected_vertices.iter().position(|expected_vert| &expected_vert == &vertex).unwrap_or_else(|| panic!("Found unexpected vertex in default cube: {:?}", vertex));

			// Check double vertices.
			if found_vertices[index_in] {
				panic!("Found double vertex in default cube: {:?}", vertex);
			}

			found_vertices[index_in] = true;
		}

		// Check if any vertices are missing.
		let missing_vertices:Vec<Vertex> = found_vertices.iter().enumerate().filter(|(_index, found)| !**found).map(|(index, _found)| expected_vertices[index]).collect::<Vec<Vertex>>();
		if !missing_vertices.is_empty() {
			panic!("Missing vertices in default cube:\n[{}]", missing_vertices.iter().map(|vertex| format!("\t{:?}", vertex)).collect::<Vec<String>>().join("\n"));
		}
	}


	#[test]
	fn tridimensional_mesh_displacement() {
		let vertices:usize = 12;
		let offset_per_vertex:[f32; 3] = [0.1, 0.2, 0.3];
		let original_vertex:Vertex = [4.0, 0.0, 1.0];
		let displacement:Vertex = [3.0, 6.0, 8.0];

		// Create mesh.
		let original_mesh:Mesh = Mesh::raw(
			(0..vertices).map(|vertex_index|
				(0..3).map(|axis| original_vertex[axis] + offset_per_vertex[axis] * vertex_index as f32).collect::<Vec<f32>>().try_into().unwrap()
			).collect::<Vec<[f32; 3]>>(),
			Vec::new()
		);

		// Create displaced meshes for both displacement methods.
		let primary_mesh:Mesh = original_mesh.displaced(&displacement);
		let mut secondary_mesh:Mesh = original_mesh;
		secondary_mesh.displace(&displacement);

		// Validate displaced meshes.
		for vertex_index in 0..12 {
			for axis in 0..3 {
				let expected_value:f32 = original_vertex[axis] + displacement[axis] + offset_per_vertex[axis] * vertex_index as f32;
				assert_eq!(primary_mesh.vertices()[vertex_index][axis], expected_value);
				assert_eq!(secondary_mesh.vertices()[vertex_index][axis], expected_value);
			}
		}
	}

	#[test]
	fn tridimensional_mesh_scaling() {
		let vertices:usize = 12;
		let offset_per_vertex:[f32; 3] = [0.1, 0.2, 0.3];
		let original_vertex:Vertex = [4.0, 0.0, 1.0];
		let scale:Vertex = [3.0, 6.0, 8.0];

		// Create mesh.
		let original_mesh:Mesh = Mesh::raw(
			(0..vertices).map(|vertex_index|
				(0..3).map(|axis| original_vertex[axis] + offset_per_vertex[axis] * vertex_index as f32).collect::<Vec<f32>>().try_into().unwrap()
			).collect::<Vec<[f32; 3]>>(),
			Vec::new()
		);

		// Create scaled meshes for both displacement methods.
		let primary_mesh:Mesh = original_mesh.scaled(&scale);
		let mut secondary_mesh:Mesh = original_mesh;
		secondary_mesh.scale(&scale);

		// Validate scaled meshes.
		for vertex_index in 0..12 {
			for axis in 0..3 {
				let expected_value:f32 = (original_vertex[axis] + offset_per_vertex[axis] * vertex_index as f32) * scale[axis];
				assert_eq!(primary_mesh.vertices()[vertex_index][axis], expected_value);
				assert_eq!(secondary_mesh.vertices()[vertex_index][axis], expected_value);
			}
		}
	}

	#[test]
	fn tridimensional_mesh_rotating() {
		let rounding_max_off:f32 = 0.001;
		let original_vertex:Vertex = [4.0, 3.0, 1.0];
		let rotation:[f32; 3] = [90.0, 0.0, 0.0];
		let expected_vertex:[f32; 3] = [4.0, 1.0, -3.0];

		// Create mesh.
		let original_mesh:Mesh = Mesh::raw(vec![original_vertex], Vec::new());

		// Create rotated meshes for both displacement methods.
		let primary_mesh:Mesh = original_mesh.rotated(&rotation, &None);
		let mut secondary_mesh:Mesh = original_mesh;
		secondary_mesh.rotate(&rotation, &None);

		// Validate rotated meshes.
		for axis in 0..3 {
			assert!((primary_mesh.vertices()[0][axis] - expected_vertex[axis]).abs() < rounding_max_off);
			assert!((secondary_mesh.vertices()[0][axis] - expected_vertex[axis]).abs() < rounding_max_off);
		}
	}

	#[test]
	fn tridimensional_mesh_rotating_euler() {
		let rounding_max_off:f32 = 0.001;
		let original_vertex:Vertex = [4.0, 3.0, 1.0];
		let rotation:[f32; 3] = [90.0, 0.0, 0.0];
		let expected_vertex:[f32; 3] = [4.0, 1.0, -3.0];

		// Create mesh.
		let original_mesh:Mesh = Mesh::raw(vec![original_vertex], Vec::new());

		// Create rotated meshes for both displacement methods.
		let primary_mesh:Mesh = original_mesh.euler_rotated(&rotation, &None);
		let mut secondary_mesh:Mesh = original_mesh;
		secondary_mesh.euler_rotate(&rotation, &None);

		// Validate rotated meshes.
		for axis in 0..3 {
			assert!((primary_mesh.vertices()[0][axis] - expected_vertex[axis]).abs() < rounding_max_off);
			assert!((secondary_mesh.vertices()[0][axis] - expected_vertex[axis]).abs() < rounding_max_off);
		}
	}
}