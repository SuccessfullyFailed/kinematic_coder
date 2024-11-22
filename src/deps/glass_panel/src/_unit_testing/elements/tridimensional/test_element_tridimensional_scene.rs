#[cfg(test)]
mod test {
	use crate::{ tridimensional::{ lenses::OrthographicLens, model::{ Material, materials::SimpleColorMaterial }, Entity, Scene }, DrawBuffer, Drawable };
	use crate::_unit_testing::support::{ TEST_SCENE_SIZE, TEST_CUBE_LOCATION, TEST_CUBE_SIZE };



	/* HELPER METHODS */

	/// Get a default testing 3D scene with a loaded cube.
	fn get_testing_scene() -> Scene {
		let cube:Entity = Entity::from_obj("DEFAULT_TEST_CUBE", TEST_CUBE_LOCATION).unwrap_or_else(|_| panic!("Could not create entity from default cube at '{TEST_CUBE_LOCATION}'")).displaced(&[0.0, TEST_CUBE_SIZE as f32 * 2.0, 0.0]);
		Scene::new(TEST_SCENE_SIZE[0], TEST_SCENE_SIZE[1], &OrthographicLens::new(), vec![cube])
	}

	/// Find the location of vertices in a rendered DrawBuffer.
	fn vertices_in_buffer(buffer:&DrawBuffer) -> Vec<[usize; 2]> {
		let default_vertex_color:u32 = SimpleColorMaterial::default().color_vertex(0, [0.0; 3]).color;
		let mut vertex_locations:Vec<[usize; 2]> = Vec::new();
		for pixel_index in 0..buffer.data.len() {
			if buffer.data[pixel_index] == default_vertex_color {
				vertex_locations.push([pixel_index % TEST_SCENE_SIZE[0], pixel_index / TEST_SCENE_SIZE[0]]);
			}
		}
		vertex_locations
	}



	/* UNIT TEST METHODS */

	#[test]
	fn tridimensional_scene_default_cube_vertex_locations() {
		let mut scene:Scene = get_testing_scene();
		let buffer:DrawBuffer = scene.draw();

		// Find the location of all vertices.
		let vertex_locations:Vec<[usize; 2]> = vertices_in_buffer(&buffer);

		// As the block is rendered flat, only the 4 vertices of the front face should be visible.
		assert_eq!(vertex_locations.len(), 4);

		for vertex_location in &vertex_locations {

			// As the cube renders a square at the front, each vertex should have a X and Y value that matches one other, not more.
			let matching_x_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[0] == vertex_location[0]).collect::<Vec<&[usize; 2]>>();
			let matching_y_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[1] == vertex_location[1]).collect::<Vec<&[usize; 2]>>();
			assert_eq!(matching_x_locations.len(), 2);
			assert_eq!(matching_y_locations.len(), 2);
			
			// Given the test cube is 40x40 pixels, each matching on the same axis should be 40 pixels removed from each other.
			assert_eq!(matching_x_locations[1][1] - matching_x_locations[0][1], TEST_CUBE_SIZE);
			assert_eq!(matching_y_locations[1][0] - matching_y_locations[0][0], TEST_CUBE_SIZE);
		}
	}

	#[test]
	fn tridimensional_scene_default_cube_edge_locations() {
		let mut scene:Scene = get_testing_scene();
		let buffer:DrawBuffer = scene.draw();
		let default_edge_color:u32 = SimpleColorMaterial::default().color_edge(0, vec![[0.0; 3]])[0].color;

		// Find the location of all vertices, tested earlier.
		let vertex_locations:Vec<[usize; 2]> = vertices_in_buffer(&buffer);
		for vertex_location in &vertex_locations {

			// An edge should be drawn straight between the vertices with a matching X value. By doing it in its own scope, the similar Y axis check cannot accidentally use these variables.
			{
				let matching_x_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[0] == vertex_location[0]).collect::<Vec<&[usize; 2]>>();
				for y in matching_x_locations[0][1] + 1..matching_x_locations[1][1] {
					let x:usize = matching_x_locations[0][0];
					assert_eq!(buffer.data[(y * TEST_SCENE_SIZE[0]) + x], default_edge_color);
				}
			}
			
			// An edge should be drawn straight between the vertices with a matching Y value.
			{
				let matching_y_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[1] == vertex_location[1]).collect::<Vec<&[usize; 2]>>();
				for x in matching_y_locations[0][0] + 1..matching_y_locations[1][0] {
					let y:usize = matching_y_locations[0][1];
					assert_eq!(buffer.data[(y * TEST_SCENE_SIZE[0]) + x], default_edge_color);
				}
			}
		}
		
		// Considering all meshes are triangulated, there should also be a diagonal line across.
		let min_vertex_x:usize = vertex_locations.iter().map(|location| location[0]).min().unwrap();
		let min_vertex_y:usize = vertex_locations.iter().map(|location| location[1]).min().unwrap();
		let max_vertex_x:usize = vertex_locations.iter().map(|location| location[0]).max().unwrap();
		let max_vertex_y:usize = vertex_locations.iter().map(|location| location[1]).max().unwrap();

		// Considering the cube is square, the line should have the same width and height.
		assert_eq!(max_vertex_x - min_vertex_x, max_vertex_y - min_vertex_y);

		// Check each pixel in the line.
		for index in 1..max_vertex_x - min_vertex_x {
			let x:usize = min_vertex_x + index;
			let y:usize = min_vertex_y + index;
			assert_eq!(buffer.data[y * TEST_SCENE_SIZE[0] + x], default_edge_color);
		}
	}

	#[test]
	fn tridimensional_scene_default_cube_face_locations() {
		let mut scene:Scene = get_testing_scene();
		let buffer:DrawBuffer = scene.draw();
		let default_face_color:u32 = SimpleColorMaterial::default().color_face(0, vec![[0.0; 3]])[0].color;

		// Figure out where the cube starts relative to the top-left corner.
		let margin:[usize; 2] = [(TEST_SCENE_SIZE[0] - TEST_CUBE_SIZE) / 2, (TEST_SCENE_SIZE[1] - TEST_CUBE_SIZE) / 2 - 1];

		// Loop through each pixel in the cube, excluding the edge.
		for offset_y in 1..TEST_CUBE_SIZE {
			let y:usize = margin[1] + offset_y;
			for offset_x in 1..TEST_CUBE_SIZE {
				let x:usize = margin[0] + offset_x;

				// Exclude diagonal line.
				if offset_x == offset_y { continue; }
				assert_eq!(buffer.data[y * TEST_SCENE_SIZE[0] + x], default_face_color);
			}
		}
	}

	#[test]
	fn tridimensional_scene_default_cube_45deg_rotation() {
		let mut scene:Scene = get_testing_scene();
		scene.entities_mut()[0].set_rotation(&[0.0, 0.0, 45.0]);
		let buffer:DrawBuffer = scene.draw();
		let vertex_locations:Vec<[usize; 2]> = vertices_in_buffer(&buffer);

		// After rotating 45 degrees, there should now be 6 vertices.
		assert_eq!(vertex_locations.len(), 6);

		for vertex_location in &vertex_locations {

			// As the cube renders a square at the front, each vertex should have a X value that matches one other and a Y value that matches two others.
			let matching_x_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[0] == vertex_location[0]).collect::<Vec<&[usize; 2]>>();
			let matching_y_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[1] == vertex_location[1]).collect::<Vec<&[usize; 2]>>();
			assert_eq!(matching_x_locations.len(), 2);
			assert_eq!(matching_y_locations.len(), 3);

			// Given the test cube is 40x40 pixels, each matching on the X axis should still be 40 pixels removed from each other. The Y axis should not.
			assert_ne!(matching_x_locations[1][0] - matching_x_locations[0][0], TEST_CUBE_SIZE);
			assert_eq!(matching_x_locations[1][1] - matching_x_locations[0][1], TEST_CUBE_SIZE);
		}

		// Because the rotation is exactly 45 degrees, the average location of the vertices should be exactly the middle of the rendered buffer.
		let summed_location:[usize; 2] = vertex_locations.iter().copied().reduce(|a, b| [a[0] + b[0], a[1] + b[1]]).unwrap();
		let average_location:[usize; 2] = [summed_location[0] / vertex_locations.len(), summed_location[1] / vertex_locations.len()];
		assert_eq!(average_location[0], TEST_SCENE_SIZE[0] / 2);
		assert_eq!(average_location[1], TEST_SCENE_SIZE[1] / 2 - 1);
	}

	#[test]
	fn tridimensional_scene_default_cube_20deg_rotation() {
		let mut scene:Scene = get_testing_scene();
		scene.entities_mut()[0].set_rotation(&[0.0, 0.0, 22.5]);
		let buffer:DrawBuffer = scene.draw();
		let vertex_locations:Vec<[usize; 2]> = vertices_in_buffer(&buffer);

		// After rotating 22.5 degrees, there should now be 6 vertices.
		assert_eq!(vertex_locations.len(), 6);

		for vertex_location in &vertex_locations {

			// As the cube renders a square at the front, each vertex should have a X value that matches one other and a Y value that matches two others.
			let matching_x_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[0] == vertex_location[0]).collect::<Vec<&[usize; 2]>>();
			let matching_y_locations:Vec<&[usize; 2]> = vertex_locations.iter().filter(|compare_vertex_location| compare_vertex_location[1] == vertex_location[1]).collect::<Vec<&[usize; 2]>>();
			assert_eq!(matching_x_locations.len(), 2);
			assert_eq!(matching_y_locations.len(), 3);

			// Given the test cube is 40x40 pixels, each matching on the X axis should still be 40 pixels removed from each other. The Y axis should not.
			assert_ne!(matching_x_locations[1][0] - matching_x_locations[0][0], TEST_CUBE_SIZE);
			assert_eq!(matching_x_locations[1][1] - matching_x_locations[0][1], TEST_CUBE_SIZE);
		}

		// Because the rotation is 22.5 degrees, the front face has not turned as much to the left as the right side face has turned to the front, so the average vertex location should be in the middle of Y, but to the left of the middle of X.
		let summed_location:[usize; 2] = vertex_locations.iter().copied().reduce(|a, b| [a[0] + b[0], a[1] + b[1]]).unwrap();
		let average_location:[usize; 2] = [summed_location[0] / vertex_locations.len(), summed_location[1] / vertex_locations.len()];
		assert!(average_location[0] > TEST_SCENE_SIZE[0] / 2);
		assert_eq!(average_location[1], TEST_SCENE_SIZE[1] / 2 - 1);
	}
}