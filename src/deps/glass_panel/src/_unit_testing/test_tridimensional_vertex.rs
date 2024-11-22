#[cfg(test)]
mod test {
	use crate::tridimensional::model::{ Vertex, VertexMath };
	use std::f32::consts::PI;

	static FLOAT_ROUNDING_MAX_OFFSET:f32 = 0.001;
	static SCALING_ROUNDING_MAX_OFFSET_PCT:f32 = 0.5;
	static ROTATION_ROUNDING_MAX_OFFSET:f32 = 0.01;

	#[test]
	fn tridimensional_vertex_displace_correct_calculation() {
		let mut vertex:Vertex = [0.0, 0.0, 0.0];
		assert_eq!(&vertex, &[0.0, 0.0, 0.0]);

		vertex.displace(&[1.0, 2.0, 3.0]);
		assert_eq!(&vertex, &[1.0, 2.0, 3.0]);

		vertex.displace(&[1.0, 2.0, 3.0]);
		assert_eq!(&vertex, &[2.0, 4.0, 6.0]);

		vertex.displace(&[-5.0, 1.0, 9.0]);
		assert_eq!(&vertex, &[-3.0, 5.0, 15.0]);

		vertex.displace(&[4.2, -1.0, -7.7]);
		assert!((vertex[0] - 1.2).abs() < FLOAT_ROUNDING_MAX_OFFSET); // Cannot check for literals as result here is 1.1999998 instead of 1.2
		assert!((vertex[1] - 4.0).abs() < FLOAT_ROUNDING_MAX_OFFSET);
		assert!((vertex[2] - 7.3).abs() < FLOAT_ROUNDING_MAX_OFFSET);
	}

	#[test]
	fn tridimensional_vertex_scale_correct_calculation() {

		// Settings.
		let original_vertex:Vertex = [1.0, -1.0, 8.0];
		let scales:Vec<[f32; 3]> = vec![[0.5, 0.5, 0.5], [3.0, 3.0, 3.0], [8.2, 9.6, 10.3], [120.6, 120.7, 120.8], [9999.0, 999.0, -10000.0]];

		// Loop through scales and validate each.
		let mut scaling_vertex:Vertex = original_vertex.clone_position();
		let mut combined_scale:[f32; 3] = [1.0; 3];
		for scale in &scales {
			scaling_vertex.scale(scale);

			// Calculate validation.
			for axis in 0..3 {
				combined_scale[axis] *= scale[axis];
				let validation_value:f32 = original_vertex[axis] * combined_scale[axis];
				let max_allowed_offset:f32 = validation_value.abs() / 100.0 * SCALING_ROUNDING_MAX_OFFSET_PCT;

				// Check correct size.
				assert!((scaling_vertex[axis] - validation_value).abs() < max_allowed_offset);
			}
		}
	}

	/* ROTATION UNIT TESTS. Very important as this is a more complicated algorithm and very important for the Scene rendering. */

	#[test]
	fn tridimensional_vertex_rotate_90n() {
		let original_vertex:Vertex = [0.0, 0.0, 1.0];

		// Loop 2 full circles.
		for axis in 0..3 {
			let mut rotation:[f32; 3] = [0.0; 3];
			while rotation[axis] < PI * 8.0 {

				// Validate rotation.
				// Given that only one axis has a value, any rotation that is an incrementation of 90 degrees over any axis should result in a vertex with only one value.
				let validation_vertex:Vertex = original_vertex.rotated(&rotation, &None);
				let valued_vertices:Vec<&f32> = validation_vertex.iter().filter(|value| value.abs() > ROTATION_ROUNDING_MAX_OFFSET).collect::<Vec<&f32>>();
				assert_eq!(valued_vertices.len(), 1);
				assert!((valued_vertices[0].abs() - 1.0).abs() < ROTATION_ROUNDING_MAX_OFFSET);

				rotation[axis] += PI / 2.0;
			}
		}
	}

	#[test]
	fn tridimensional_vertex_rotate_90n_plus_45() {
		let original_vertices:[Vertex; 3] = [[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]];

		// Loop 2 full circles over each axis.
		for axis in 0..3 {
			let mut rotation:[f32; 3] = [0.0; 3];
			rotation[axis] = PI / 4.0;
			while rotation[axis] < PI * 8.0 {

				// Validate rotation.
				// Given that only one axis has a value, any rotation that is an incrementation of 90 degrees plus 45 should result in a vertex where 2 axes have a value. The positive values of these axes should be the same.
				let validation_vertex:Vertex = original_vertices[axis].rotated(&rotation, &None);
				let valued_vertices:Vec<&f32> = validation_vertex.iter().filter(|value| value.abs() > ROTATION_ROUNDING_MAX_OFFSET).collect::<Vec<&f32>>();
				assert_eq!(valued_vertices.len(), 2);
				assert!((valued_vertices[0].abs() - valued_vertices[1].abs()).abs() < ROTATION_ROUNDING_MAX_OFFSET);

				rotation[axis] += PI / 2.0;
			}
		}
	}

	#[test]
	fn tridimensional_vertex_rotate_1n() {
		let original_vertex:Vertex = [0.0, 0.0, 1.0];
		let step:f32 = 1.0 / 180.0 * PI;
		let mut rotation:f32 = step;
		while rotation < PI * 0.5 {

			let current_vertex:Vertex = original_vertex.rotated(&[rotation, 0.0, 0.0], &None);
			let previous_vertex:Vertex = original_vertex.rotated(&[rotation - step, 0.0, 0.0], &None);
			assert_eq!(current_vertex[0], 0.0);
			assert_eq!(previous_vertex[0], 0.0);
			assert!(current_vertex[1] > previous_vertex[1]);
			assert!(current_vertex[2] < previous_vertex[2]);

			rotation += step;
		}
	}

	#[test]
	fn tridimensional_vertex_rotate_acceptable_warping() {
		let loops:usize = 100_000;
		let step:f32 = PI * 2.0;
		let original_vertex:Vertex = [1.0, 2.0, 3.0];
		let mut compare_vertex:Vertex = original_vertex.clone_position();
		for _ in 0..loops {
			compare_vertex.rotate(&[step, 0.0, 0.0], &None);
		}
		for axis in 0..3 {
			assert!((original_vertex[axis].abs() - compare_vertex[axis].abs()).abs() < ROTATION_ROUNDING_MAX_OFFSET);
		}
	}

	#[test]
	fn tridimensional_vertex_rotate_negative() {
		let step:f32 = (90.0 / 16.0) / 180.0 * PI;
		let mut vertex:Vertex = [0.0, 0.0, 3.0];
		for _ in 0..16 {
			vertex.rotate(&[step, 0.0, 0.0], &None);
		}
		assert!(vertex[0] == 0.0);
		assert!((vertex[1].abs() - 3.0) < ROTATION_ROUNDING_MAX_OFFSET);
		assert!(vertex[2].abs() < ROTATION_ROUNDING_MAX_OFFSET);
	}

	#[test]
	fn tridimensional_vertex_rotation_large_number() {
		let vertex:Vertex = [0.0, 0.0, 3.0].rotated(&[(PI * 2.0) * 8000.0 + (PI * 0.25), 0.0, 0.0], &None);
		assert!(vertex[0] == 0.0);
		assert!(vertex[1].abs() - vertex[2].abs() < ROTATION_ROUNDING_MAX_OFFSET);
	}

	#[test]
	fn tridimensional_vertex_rotation_multiple_random_rotations() {
		let original_vertex:Vertex = [0.0, 0.0, 3.0];

		for rotation_axis in 0..3 { // Can only add rotation in one singular axis or the rotation will become inaccurate.

			// Keep rotating vertex.
			let mut rotating_vertex:Vertex = original_vertex.clone_position();
			let mut combined_rotation:[f32; 3] = [0.0, 0.0, 0.0];
			for _ in 0..3 {
				let mut rotation:[f32; 3] = [0.0; 3];
				rotation[rotation_axis] = 1.0;

				rotating_vertex.rotate(&rotation, &None);
				for axis in 0..3 { combined_rotation[axis] += rotation[axis]; }
			}
			
			// Validate outcome.
			let validation_vertex:Vertex = original_vertex.rotated(&combined_rotation, &None);
			for axis in 0..3 {
				assert!((rotating_vertex[axis].abs() - validation_vertex[axis].abs()).abs() < ROTATION_ROUNDING_MAX_OFFSET);
			}
		}
	}
}