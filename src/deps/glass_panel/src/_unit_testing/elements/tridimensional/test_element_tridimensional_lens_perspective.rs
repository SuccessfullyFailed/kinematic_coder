#[cfg(test)]
mod test {
	use crate::tridimensional::{lenses::PerspectiveLens, model::Vertex, Lens, D_AXIS, X_AXIS, Y_AXIS};

	#[test]
	fn tridimensional_lens_perspective_sample_scene() {

		// Create a list of vertices.
		let vertices:Vec<Vertex> = vec![
			[  0.0, -10.0,  0.0], // Invisible.
			[  0.0,   0.0,  0.0], // Invisible.
			[  0.0, 100.0,  0.0], // Visible center.
			[-20.0, 100.0,  0.0], // Visible left.
			[  0.0, 100.0, 20.0], // Visible top.
			[ 20.0, 100.0, 20.0]  // Visible diagonal.
		];

		// Warp vertices using lens.
		let lens:PerspectiveLens = PerspectiveLens::new(400.0, 90.0);
		let warped_vertices:Vec<Vertex> = lens.warp_vertices(vertices);

		// Compare vertices.
		assert!(warped_vertices[0][D_AXIS] <= 0.0); // Invisible.
		assert!(warped_vertices[1][D_AXIS] <= 0.0); // Invisible.
		assert!(warped_vertices[2][D_AXIS] > 0.0 && warped_vertices[2][X_AXIS] == warped_vertices[2][Y_AXIS]); // Visible center.
		assert!(warped_vertices[3][D_AXIS] > 0.0 && warped_vertices[3][X_AXIS] < 0.0 && warped_vertices[3][Y_AXIS] == 0.0); // Visible left.
		assert!(warped_vertices[4][D_AXIS] > 0.0 && warped_vertices[4][X_AXIS] == 0.0 && warped_vertices[4][Y_AXIS] > 0.0); // Visible top.
		assert!(warped_vertices[5][D_AXIS] > 0.0 && warped_vertices[5][X_AXIS] > 0.0 && warped_vertices[5][Y_AXIS] == warped_vertices[5][X_AXIS]); // Visible diagonal.
	}

	#[test]
	fn tridimensional_lens_perspective_field_of_view() {

		// Create a list of vertices.
		let vertices:Vec<Vertex> = vec![
			[  0.0, -10.0,   0.0],
			[  0.0,   0.0,   0.0],
			[  0.0, 100.0,   0.0],
			[-10.0, 100.0,  10.0],
			[-10.0, 100.0, -10.0],
			[ 10.0, 100.0,  10.0],
			[ 10.0, 100.0, -10.0]
		];

		// Warp vertices using lens.
		let lens_normal:PerspectiveLens = PerspectiveLens::new(400.0, 90.0);
		let lens_narrow:PerspectiveLens = PerspectiveLens::new(400.0, 50.0);
		let warped_vertices_normal:Vec<Vertex> = lens_normal.warp_vertices(vertices.clone());
		let warped_vertices_narrow:Vec<Vertex> = lens_narrow.warp_vertices(vertices.clone());

		// Compare vertices.
		for vertex_index in 0..vertices.len() {
			for axis_index in [0, 2] {
				let original:f32 = vertices[vertex_index][axis_index];
				let warped_normal:f32 = warped_vertices_normal[vertex_index][axis_index];
				let warped_narrow:f32 = warped_vertices_narrow[vertex_index][axis_index];
				if original == 0.0 {
					assert!(warped_narrow == warped_normal);
				} else if original < 0.0 {
					assert!(warped_narrow > warped_normal); // Narrow lens creates 'stretched' picture due to "fitting less angle in the same picture".
				} else {
					assert!(warped_narrow < warped_normal);
				}
			}
		}
	}

	#[test]
	fn tridimensional_lens_perspective_depth() {
		let mut vertex:Vertex = [80.0, 100.0, 0.0];
		let lens:PerspectiveLens = PerspectiveLens::new(400.0, 90.0);

		// Compare vertices.
		let mut last_x:f32 = lens.warp_vertices(vec![vertex])[0][0];
		for _ in 0..6 {
			vertex[1] *= 2.0;
			let current_x:f32 = lens.warp_vertices(vec![vertex])[0][0];
			assert!(current_x < last_x);
			last_x = current_x;
		}
	}
}