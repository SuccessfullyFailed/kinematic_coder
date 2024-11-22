use crate::{ ui::Window, robot_configuration::RobotConfig };
use super::{ RobotSkeleton, LegSynchronizer };


static mut REALTIME_KINEMATICS_ACTIVE:bool = false;

/// Check if the realtime kinematics are active.
pub fn realtime_kinematics_active() -> bool {
	unsafe { REALTIME_KINEMATICS_ACTIVE }
}

/// Activate the realtime kinematics.
pub fn activate_realtime_kinematics() {
	if unsafe { REALTIME_KINEMATICS_ACTIVE } { return; }
	unsafe { REALTIME_KINEMATICS_ACTIVE = true; }

	RobotSkeleton::create();
	LegSynchronizer::create();
	override_step_sizes(Some(*LegSynchronizer::get().smallest_step()));

	Window::get().execute_post_listener(&||{
		let _ = Window::get().update_robot_config_in_scene_now();
		update_realtime_kinematics()
	});
}

/// Deactivate the realtime kinematics.
pub fn deactivate_realtime_kinematics() {
	if unsafe { !REALTIME_KINEMATICS_ACTIVE } { return; }
	unsafe { REALTIME_KINEMATICS_ACTIVE = false; }

	override_step_sizes(None);

	// Reset motors to default rotation.
	for leg in RobotConfig::get_mut().legs_mut() {
		for joint in leg {
			if let Some(motor) = joint.motor_mut() {
				*motor.current_rotation_mut() = *motor.default_rotation();
			}
		}
	}

	Window::get().update_robot_config_in_scene_synchronized();
}

/// Update the realtime kinematics. Will call the function again on each frame.
fn update_realtime_kinematics() {
	use glass_panel::tridimensional::model::VertexMath;
	use crate::kinematics::calculate_leg_rotations;

	// Fetch mainly used variables.
	let ui:&mut Window = Window::get();
	let skeleton:&RobotSkeleton = RobotSkeleton::get();
	let robot_config:&mut RobotConfig = RobotConfig::get_mut();
	let strafe_angle:f32 = LegSynchronizer::get().strafe().to_radians();

	// Check if the system should stop.
	if unsafe { !REALTIME_KINEMATICS_ACTIVE } {
		return;
	}

	// Update motor rotations according to target positions.
	for (leg_index, target_offset) in &LegSynchronizer::get().realtime_target_offsets() {
		if let Some(leg) = &skeleton.legs()[*leg_index] {
			let step_position:[f32; 3] = *leg.step_position();
			let target_position:[f32; 3] = step_position.displaced(&target_offset.rotated(&[0.0, 0.0, strafe_angle], &None));

			// Find attached joint.
			if let Some(rotations) = calculate_leg_rotations(leg, target_position) {
				for (joint_index, target_rotation) in rotations {
					if let Some(motor_joint) = robot_config.get_joint_mut(*leg_index, joint_index) {
						if let Some(motor) = motor_joint.motor_mut() {

							// Rotate joint.
							*motor.current_rotation_mut() = target_rotation;
						}
					}
				}
			}
		}
	}

	// Update leg rotations according to target rotations.
	ui.update_scene_motors();

	// Trigger new update.
	ui.execute_post_listener(&update_realtime_kinematics);
}

/// Set the step size of all kinematic steps.
fn override_step_sizes(size:Option<f32>) {
	for leg in RobotConfig::get_mut().legs_mut() {
		for joint in leg {
			if let Some(kinematics) = joint.kinematics_config_mut() {
				*kinematics.step_distance_override() = size;
			}
		}
	}
}