use super::super::{ Lens, scene::{ X_AXIS, Y_AXIS, D_AXIS }, model::Vertex };

pub struct PerspectiveLens {
	lens_depth:f32,
	field_of_view:f32
}
impl PerspectiveLens {
	
	/// Create a new PerspectiveLens lens.
	pub fn new(lens_depth:f32, field_of_view:f32) -> PerspectiveLens {
		PerspectiveLens {
			lens_depth,
			field_of_view
		}
	}
}
impl Lens for PerspectiveLens {

	/// Wrap the lens in a box.
	fn boxify(&self) -> Box<dyn Lens> {
		Box::new(PerspectiveLens::new(self.lens_depth, self.field_of_view))
	}
	
	/// Map the vertices from their 3D unprocessed locations to locations relative to the perspective of the lens.
	fn warp_vertices(&self, vertices:Vec<Vertex>) -> Vec<Vertex> {
		let fov_adjustment:f32 = (self.field_of_view / 2.0).to_radians().tan();
		vertices.iter().map(|vertex| {
			let distance:f32 = (vertex[0].abs().powi(2) + vertex[1].max(0.0).powi(2) + vertex[2].abs().powi(2)).sqrt();
			if distance == 0.0 || distance.is_nan() || distance.is_infinite() {
				[0.0; 3]
			} else if vertex[D_AXIS] > 0.0 {
				[
					vertex[X_AXIS] / distance * self.lens_depth * fov_adjustment,
					vertex[D_AXIS],
					vertex[Y_AXIS] / distance * self.lens_depth * fov_adjustment
				]
			} else {
				// TODO: When D negative, stops at 0 instead of moving on. Fix to fix warped edges and faces.
				[
					vertex[X_AXIS] / distance * self.lens_depth * fov_adjustment,
					0.0,
					vertex[Y_AXIS] / distance * self.lens_depth * fov_adjustment
				]
			}
		}).collect::<Vec<Vertex>>()
	}
}