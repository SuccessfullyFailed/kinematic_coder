use super::{ RobotSkeletonLeg, RobotSkeletonSegment };
use glass_panel::tridimensional::model::VertexMath;
use std::f32::consts::PI;

/// Calculate how to rotate the motors to get to the target point.
pub fn calculate_leg_rotations(leg:&RobotSkeletonLeg, target_offset:[f32; 3]) -> Option<Vec<(usize, f32)>> {
	let segments:&Vec<RobotSkeletonSegment> = leg.segments();
	let total_endpoint_offset:[f32; 3] = leg.total_offset();

	// Find out which axes to use for the calculations.
	let motors_per_axis:Vec<Vec<(usize, &RobotSkeletonSegment)>> = (0..3).map(|axis| segments.iter().enumerate().filter(|(_, segment)| *segment.axis() == axis).collect()).collect::<Vec<Vec<(usize, &RobotSkeletonSegment)>>>();
	let (rigid_axis, flexible_axis, alternate_axis) = if motors_per_axis[0].len() < motors_per_axis[2].len() { (0_usize, 2_usize, 1_usize) } else { (2_usize, 0_usize, 1_usize) };

	let mut rotation_per_joint:Vec<(usize, f32)> = Vec::new();

	// If the first motor rotates around the rigid axis, simply point it at the target position, then calculate the other axis.
	if !segments.is_empty() && segments[0].axis() == &(rigid_axis as u8) {

		// Rotate the rigid axis to the correct position.
		let neutral_rigid_offset_rotation:f32 = total_endpoint_offset[flexible_axis].atan2(total_endpoint_offset[alternate_axis]);
		let rigid_rotation:f32 = (PI * 0.5) - target_offset[alternate_axis].atan2(target_offset[flexible_axis]) - neutral_rigid_offset_rotation;
		rotation_per_joint.push((segments[0].joint_range()[0], rigid_rotation.to_degrees()));

		// Rotate the flexible axis to the correct position.
		if !motors_per_axis[flexible_axis].is_empty() {
			let first_flex_index:usize = motors_per_axis[flexible_axis][0].0;

			// Calculate where the flexible axis starts.
			let mut flex_start_position:[f32; 3] = [0.0; 3];
			for segment in &segments[..first_flex_index] {
				flex_start_position.displace(segment.endpoint());
			}
			let mut flex_start_rotation:[f32; 3] = [0.0; 3];
			flex_start_rotation[rigid_axis] = rigid_rotation;
			flex_start_position.euler_rotate(&flex_start_rotation, &None);

			// Calculate the distance required to cover with the flex segments.
			let flex_offset:[f32; 3] = target_offset.displaced(&flex_start_position.negative());
			let flex_target_distance:f32 = (flex_offset[0].powi(2) + flex_offset[1].powi(2) + flex_offset[2].powi(2)).sqrt();

			// Join segments where there is no motor over the flex axis.
			let mut flex_joint_segments:Vec<(usize, [f32; 3])> = Vec::new();
			let mut flex_joint_segment:(usize, [f32; 3]) = (first_flex_index, *segments[first_flex_index].endpoint());
			for (index, segment) in segments.iter().enumerate().skip(first_flex_index+1) {
				if *segment.axis() as usize == flexible_axis {
					flex_joint_segments.push(flex_joint_segment);
					flex_joint_segment = (index, [0.0; 3]);
				}
				flex_joint_segment.1.displace(segment.endpoint());
			}
			flex_joint_segments.push(flex_joint_segment);

			// Calculate how much distance the joints can cover.
			let distances:Vec<f32> = flex_joint_segments.iter().map(|(_, segment)| (segment[alternate_axis].powi(2) + segment[rigid_axis].powi(2) + segment[flexible_axis].powi(2)).sqrt()).collect::<Vec<f32>>();
			let available_distance:f32 = distances.iter().sum();
			let z_offset_rotation:f32 = -(flex_offset[rigid_axis] / (flex_offset[alternate_axis].powi(2) + flex_offset[flexible_axis].powi(2)).sqrt()).atan().to_degrees();

			// Check point is reachable.
			if available_distance.abs() < flex_target_distance.abs() {
				return None;
			}
			
			// Calculate leg rotation.
			if distances.len() == 2 {

				// Calculate point of intersection.
				let intersection_x:f32 = (flex_target_distance.powi(2) - distances[1].powi(2) + distances[0].powi(2)) / (flex_target_distance * 2.0);
				let intersection_y:f32 = -(distances[0].powi(2) - intersection_x.powi(2)).sqrt();

				// Calculate angles.
				let angle1:f32 = intersection_y.atan2(intersection_x);
				let angle2:f32 = (flex_target_distance - intersection_x).atan2(intersection_y) - (PI / 2.0);

				// Set angles to joints.
				rotation_per_joint.push((first_flex_index, angle1.to_degrees() + z_offset_rotation));
				rotation_per_joint.push((first_flex_index + 1, -angle1.to_degrees() + angle2.to_degrees()));

				// Return dataset.
				return Some(rotation_per_joint);
			}
			if distances.len() == 3 {
				
				// ToDo: Allow triple join legs with different distances, then create a recursive version to allow any amount of joints.
				let angle:f32 = -(((flex_target_distance - distances[1]) / 2.0) / ((available_distance - distances[1]) / 2.0)).acos().to_degrees();
				rotation_per_joint.push((first_flex_index, angle + z_offset_rotation));
				rotation_per_joint.push((first_flex_index + 1, -angle));
				rotation_per_joint.push((first_flex_index + 2, -angle));
				return Some(rotation_per_joint);

				/*

				// Prepare position and distance variables.
				let target_offset:[f32; 2] = [flex_target_distance, 0.0]; // TODO: Dynamic axes.
				let left_distance:f32 = distances[0];
				let middle_distance:f32 = distances[1];
				let right_distance:f32 = distances[2];

				// Calculate first angle.
				let center = [target_offset[0] * 0.5, target_offset[1] * 0.5];
				let intersection_1 = get_intersect_point([0.0; 2], center, [left_distance, middle_distance]);
				let intersection_2 = get_intersect_point(center, target_offset, [middle_distance, right_distance]);

				// Displace center circle to match required length.
				let center_rotator:f32 = (target_offset[0] - intersection_1[0]).atan2(intersection_1[1]) - (PI / 2.0);

				// Calculate angles.
				let angle1:f32 = intersection_1[1].atan2(intersection_1[0]) - (center_rotator * 0.5);
				let angle2:f32 = -angle1;
				let angle3:f32 = (PI * 2.0) - intersection_2[1].atan2(target_offset[0] - intersection_2[0]);

				// Set angles to joints.
				rotation_per_joint.push((first_flex_index, angle1.to_degrees() + z_offset_rotation));
				rotation_per_joint.push((first_flex_index + 1, angle2.to_degrees()));
				rotation_per_joint.push((first_flex_index + 2, angle3.to_degrees()));
				
				// Return dataset.
				return Some(rotation_per_joint);
				*/
			}
		}
	}

	None
}


/*
fn get_intersect_point(start:[f32; 2], target:[f32; 2], radii:[f32; 2]) -> [f32; 2] {
	let target_offset:[f32; 2] = [target[0] - start[0], target[1] - start[1]];
	let target_distance:f32 = (target_offset[0].powi(2) + target_offset[1].powi(2)).sqrt();
	let intersection_x:f32 = (target_distance.powi(2) - radii[1].powi(2) + radii[0].powi(2)) / (target_distance * 2.0);
	let intersection_y:f32 = -(radii[0].powi(2) - intersection_x.powi(2)).sqrt();
	[start[0] + intersection_x, start[1] + intersection_y]
}
*/