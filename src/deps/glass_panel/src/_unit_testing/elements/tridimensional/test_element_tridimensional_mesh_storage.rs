#[cfg(test)]
mod test {
	use crate::tridimensional::{ StaticMeshLink, StaticMesh, model::Mesh };
	use crate::_unit_testing::support::TEST_CUBE_LOCATION;



	/* HELPER METHODS */

	/// Create a "random" mesh.
	fn create_random_mesh() -> Mesh {
		use std::time::{ SystemTime, UNIX_EPOCH };

		let random_modifier:f32 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f32; // "Random" modifier.
		Mesh::raw(
			(0..12).map(|vertex_index|
				(0..3).map(|axis_index| (vertex_index * axis_index) as f32 * random_modifier).collect::<Vec<f32>>().try_into().unwrap()
			).collect::<Vec<[f32; 3]>>(),
			(0..12).map(|face_index|
				(0..3).map(|face_vertex_index| ((face_index * face_vertex_index) as f32 * random_modifier) as usize % 12).collect::<Vec<usize>>().try_into().unwrap()
			).collect::<Vec<[usize; 3]>>()
		)
	}

	#[test]
	fn tridimensional_mesh_storage_can_store_and_load_mesh() {

		// Create.
		let random_mesh:Mesh = create_random_mesh();

		// Store.
		assert_eq!(StaticMesh::find_source(TEST_CUBE_LOCATION), None);
		let link:StaticMeshLink = StaticMesh::create("TEST_MESH", random_mesh.clone());

		// Compare.
		assert_eq!(link.get_mesh().unwrap().vertices(), random_mesh.vertices());
		assert_eq!(link.get_mesh().unwrap().faces(), random_mesh.faces());
	}

	#[test]
	fn tridimensional_mesh_storage_can_store_obj() {

		// Store.
		assert_eq!(StaticMesh::find_source(TEST_CUBE_LOCATION), None);
		let link:StaticMeshLink = StaticMesh::from_obj(TEST_CUBE_LOCATION).unwrap();

		// Compare.
		let cube:Mesh = Mesh::from_obj(TEST_CUBE_LOCATION).unwrap();
		assert_eq!(link.get_mesh().unwrap().vertices(), cube.vertices());
		assert_eq!(link.get_mesh().unwrap().faces(), cube.faces());
	}

	#[test]
	fn tridimensional_mesh_storage_shares_assets() {
		let primary_random_mesh:Mesh = create_random_mesh();
		let secondary_random_mesh:Mesh = create_random_mesh();

		// Create first.
		let first_link:StaticMeshLink = StaticMesh::create("PRIMARY_TEST_MESH", primary_random_mesh.clone());
		let first_size:usize = StaticMesh::memory_size();

		// Add second.
		let second_link:StaticMeshLink = StaticMesh::create("PRIMARY_TEST_MESH", primary_random_mesh.clone());
		let second_size:usize = StaticMesh::memory_size();

		// Compare.
		assert_eq!(first_link, second_link);
		assert_eq!(first_size, second_size);

		// Add third.
		let third_link:StaticMeshLink = StaticMesh::create("SECONDARY_TEST_MESH", secondary_random_mesh.clone());
		let third_size:usize = StaticMesh::memory_size();

		// Compare.
		assert!(first_link != third_link);
		assert!(first_size != third_size);
	}
}