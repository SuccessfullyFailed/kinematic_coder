use crate::{ kinematics::LegSynchronizer, robot_configuration::{ KinematicsConfig, LegConfig, MotorConfig, RobotConfig }, ui::Window };



/* HELPER METHODS */

/// Get a joint by its indexes.
fn get_selected_joint_mut() -> Option<&'static mut LegConfig> {
	let ui:&mut Window = Window::get();
	let selected_joint_indexes:Option<[usize; 2]> = *ui.selected_joint();
	let robot_config:&mut RobotConfig = RobotConfig::get_mut();
	selected_joint_indexes.and_then(|joint_indexes| robot_config.get_joint_mut(joint_indexes[0], joint_indexes[1]))
}



/* DISPLACEMENT TAB METHODS */

/// Get the position of a specific leg by its indexes.
pub fn get_selected_joint_position_mut() -> Option<&'static mut [f32; 3]> {
	get_selected_joint_mut().map(|joint| joint.position_mut())
}

/// Get the coordinates of the selected leg.
pub fn get_leg_position() -> [f32; 3] {
	get_selected_joint_position_mut().map(|position| *position).unwrap_or([0.0; 3])
}

/// Set the position of the selected leg.
pub fn set_leg_position(index:usize, value:f32) {
	if let Some(joint_position) = get_selected_joint_position_mut() {
		joint_position[index] = value;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}



/* MOTOR TAB METHODS */

/// Add a motor at position 0.
pub fn add_motor() {
	if let Some(joint) = get_selected_joint_mut() {
		if joint.motor().is_none() {
			*joint.motor_mut() = Some(MotorConfig::empty());
			let ui:&mut Window = Window::get();
			ui.update_robot_properties_menu_synchronized();
			ui.update_robot_config_in_scene_synchronized();
		}
	}
}

/// Get the position of a specific leg by its indexes.
pub fn get_selected_joint_motor_mut() -> Option<&'static mut MotorConfig> {
	match get_selected_joint_mut(){
		Some(joint) => match joint.motor_mut() {
			Some(motor) => Some(motor),
			None => None
		},
		None => None
	}
}

/// Get the coordinates of the selected leg.
pub fn get_motor_position() -> [f32; 3] {
	get_selected_joint_motor_mut().map(|motor| *motor.position()).unwrap_or([0.0; 3])
}

/// Set the position of the selected leg.
pub fn set_motor_position(index:usize, value:f32) {
	if let Some(motor) = get_selected_joint_motor_mut() {
		motor.position_mut()[index] = value;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the default rotation of the selected motor.
pub fn get_motor_rotation_axis() -> u8 {
	get_selected_joint_motor_mut().map(|motor| *motor.rotation_axis()).unwrap_or(0)
}

/// Set the default rotation of the selected motor.
pub fn set_motor_rotation_axis(axis:u8) {
	if let Some(motor) = get_selected_joint_motor_mut() {
		*motor.rotation_axis_mut() = axis;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the default rotation of the selected motor.
pub fn get_motor_default_rotation() -> f32 {
	get_selected_joint_motor_mut().map(|motor| *motor.default_rotation()).unwrap_or(0.0)
}

/// Set the default rotation of the selected motor.
pub fn set_motor_default_rotation(rotation:f32) {
	if let Some(motor) = get_selected_joint_motor_mut() {
		let bounds:&[f32; 2] = motor.rotation_range();
		*motor.default_rotation_mut() = rotation.max(bounds[0]).min(bounds[1]);
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the rotation range of the selected motor.
pub fn get_motor_rotation_range() -> [f32; 2] {
	get_selected_joint_motor_mut().map(|motor| *motor.rotation_range()).unwrap_or([0.0; 2])
}

/// Set the rotation range of the selected motor.
pub fn set_motor_rotation_range(index:usize, value:f32) {
	if let Some(motor) = get_selected_joint_motor_mut() {

		// Get default and current rotation.
		let default_rotation:f32 = *motor.default_rotation();
		let current_rotation:f32 = *motor.current_rotation();

		// Update Range.
		let range:&mut [f32; 2] = motor.rotation_range_mut();
		range[index] = value;
		if index == 0 {
			range[0] = range[0].min(range[1]).min(default_rotation).min(current_rotation);
		} else {
			range[1] = range[0].max(range[1]).max(default_rotation).max(current_rotation);
		}

		// Update UI.
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the current rotation of the selected motor.
pub fn get_motor_current_rotation() -> f32 {
	get_selected_joint_motor_mut().map(|motor| *motor.current_rotation()).unwrap_or(0.0)
}

/// Set the current rotation of the selected motor.
pub fn set_motor_current_rotation(rotation:f32) {
	if let Some(selected_joint_indexes) = Window::get().selected_joint() {
		if let Some(selected_joint) = RobotConfig::get_mut().get_joint_mut(selected_joint_indexes[0], selected_joint_indexes[1]) {
			if let Some(motor) = selected_joint.motor_mut() {
				let bounds:&[f32; 2] = motor.rotation_range();
				*motor.current_rotation_mut() = rotation.max(bounds[0]).min(bounds[1]);
				Window::get().update_scene_motor(selected_joint_indexes[0], selected_joint_indexes[1]);
			}
		}
	}
}



/* KINEMATICS TAB METHODS */

/// Add a motor at position 0.
pub fn add_kinematics() {
	if let Some(joint) = get_selected_joint_mut() {
		if joint.kinematics_config().is_none() {
			*joint.kinematics_config_mut() = Some(KinematicsConfig::empty());
			let ui:&mut Window = Window::get();
			ui.update_robot_properties_menu_synchronized();
			ui.update_robot_config_in_scene_synchronized();
		}
	}
}

/// Get the position of the kinematics endpoint of a specific leg.
pub fn get_selected_joint_kinematics_mut() -> Option<&'static mut KinematicsConfig> {
	match get_selected_joint_mut(){
		Some(joint) => match joint.kinematics_config_mut() {
			Some(kinematics) => Some(kinematics),
			None => None
		},
		None => None
	}
}

/// Get the position of the kinematics endpoint of the selected leg.
pub fn get_kinematics_endpoint() -> [f32; 3] {
	get_selected_joint_kinematics_mut().map(|kinematics| *kinematics.leg_endpoint()).unwrap_or([0.0; 3])
}

/// Set the position of the kinematics endpoint of the selected leg.
pub fn set_kinematics_endpoint(index:usize, value:f32) {
	if let Some(kinematics) = get_selected_joint_kinematics_mut() {
		kinematics.leg_endpoint_mut()[index] = value;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the position of the kinematics step.
pub fn get_kinematics_step_position() -> [f32; 3] {
	get_selected_joint_kinematics_mut().map(|kinematics| *kinematics.step_position()).unwrap_or([0.0; 3])
}

/// Set the position of the kinematics step.
pub fn set_kinematics_step_position(index:usize, value:f32) {
	if let Some(kinematics) = get_selected_joint_kinematics_mut() {
		kinematics.step_position_mut()[index] = value;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the distance of the kinematics step.
pub fn get_kinematics_step_distance() -> f32 {
	get_selected_joint_kinematics_mut().map(|kinematics| *kinematics.step_distance()).unwrap_or(0.0)
}

/// Get the distance of the kinematics step.
pub fn set_kinematics_step_distance(value:f32) {
	if let Some(kinematics) = get_selected_joint_kinematics_mut() {
		*kinematics.step_distance_mut() = value;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}

/// Get the height of the kinematics step.
pub fn get_kinematics_step_height() -> f32 {
	get_selected_joint_kinematics_mut().map(|kinematics| *kinematics.step_height()).unwrap_or(0.0)
}

/// Get the height of the kinematics step.
pub fn set_kinematics_step_height(value:f32) {
	if let Some(kinematics) = get_selected_joint_kinematics_mut() {
		*kinematics.step_height_mut() = value;
		Window::get().update_robot_config_in_scene_synchronized();
	}
}



/* CONTROLLER TAB METHODS */

/// Get a boolean indicating if the realtime kinematics are showing.
pub fn get_kinematics_realtime() -> bool {
	crate::kinematics::realtime_kinematics_active()
}

/// Set a boolean indicating if the realtime kinematics are showing.
pub fn set_kinematics_realtime(enabled:bool) {
	use crate::kinematics::{ activate_realtime_kinematics, deactivate_realtime_kinematics };

	if enabled {
		activate_realtime_kinematics()
	} else {
		deactivate_realtime_kinematics();
	}
}

/// Get the speed of the realtime kinematics.
pub fn get_kinematics_speed() -> f32 {
	*LegSynchronizer::get().speed()
}

/// Set the speed of the realtime kinematics
pub fn set_kinematics_speed(speed:f32) {
	*LegSynchronizer::get().speed_mut() = speed;
}

/// Get the amount of sideways motions the robot is doing in the realtime kinematics.
pub fn get_kinematics_strafe() -> f32 {
	*LegSynchronizer::get().strafe()
}

/// Set the amount of sideways motions the robot is doing in the realtime kinematics.
pub fn set_kinematics_strafe(strafe:f32) {
	*LegSynchronizer::get().strafe_mut() = strafe;
	Window::get().update_robot_config_in_scene_synchronized();
}