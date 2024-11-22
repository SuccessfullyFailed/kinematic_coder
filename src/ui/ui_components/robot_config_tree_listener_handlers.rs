use crate::{ robot_configuration::{LegConfig, RobotConfig}, ui::Window };
use glass_panel::DrawableData;
use std::{ thread, rc::Rc };



/* BODY MESH METHODS */

/// Show the tooltip to replace or remove the body mesh.
pub(super) fn show_body_replacement_menu(menu_position:[isize; 2]) {
	Window::get().show_mesh_replacement_tooltip(
		menu_position,
		Rc::new(|_,_,_,_| replace_body()),
		None,
		Rc::new(|_,_,_,_| remove_body())
	)
}

/// Let the user select a new body mesh using the file selector window.
pub(super) fn replace_body() {
	thread::spawn(||{
		if let Some(path) = Window::select_obj(None) {
			let robot_config:&mut RobotConfig = RobotConfig::get_mut();
			if &Some(path.clone()) != robot_config.body() {
				*robot_config.body_mut() = Some(path);
				Window::get().update_robot_config_synchronized();
				set_joint_as_actively_selected(None);
			}
		}
	});
}

/// Remove the body mesh entirely from the robot config.
fn remove_body() {
	let robot_config:&mut RobotConfig = RobotConfig::get_mut();
	if robot_config.body().is_some() {
		*robot_config.body_mut() = None;
		Window::get().update_robot_config_synchronized();
		set_joint_as_actively_selected(None);
	}
}



/* SELECTED JOINT METHODS */

/// Set the clicked joint as currently selected.
pub(super) fn select_clicked_joint(element_data:&mut DrawableData) {
	if let Some(leg_index) = element_data.get_setting_value::<usize>("leg_index") {
		if let Some(joint_index) = element_data.get_setting_value::<usize>("joint_index") {
			set_joint_as_actively_selected(Some([leg_index, joint_index]));
		}
	}
}

/// Set a joint as currently selected.
fn set_joint_as_actively_selected(joint_indexes:Option<[usize; 2]>) {
	let ui:&mut Window = Window::get();

	// Reset color of previously selected.
	if let Some(selected_indexes) = ui.selected_joint() {
		set_joint_color(selected_indexes[0], selected_indexes[1], ui.setting("color_text"));
	}

	// Set selected joint.
	*ui.selected_joint_mut() = joint_indexes;

	// Update UI.
	ui.execute_post_listener(&||{
		let ui:&mut Window = Window::get();
		ui.update_robot_properties_menu_now();

		// Modify selected object color.
		if let Some(selected_indexes) = ui.selected_joint() {
			set_joint_color(selected_indexes[0], selected_indexes[1], ui.setting("color_detail"));
		}
	});
}

/// Set the color of a specific joint in the objects tree.
fn set_joint_color(leg_index:usize, joint_index:usize, color:u32) {
	let ui:&mut Window = Window::get();
	if let Some(joint_obj) = ui.window_mut().source_mut().child_by_name_mut(&format!("#Leg{leg_index}Joint{joint_index} text")) {
		joint_obj.data_set_mut().set_setting_value::<u32>("color", color);
	}
}

/// Set no joint as currently selected.
pub(super) fn set_no_joints_selected() {
	set_joint_as_actively_selected(None);
}



/* JOINT MESH METHODS */

/// Add a new leg mesh.
pub(super) fn add_leg() {
	thread::spawn(|| {
		if let Some(path) = Window::select_obj(None) {
			RobotConfig::get_mut().add_leg(&path);
			Window::get().update_robot_config_synchronized();
			set_joint_as_actively_selected(Some([RobotConfig::get().legs().len() - 1, 0]));
		}
	});
}

/// Add a new joint mesh.
pub(super) fn add_joint(element_data:&mut DrawableData) {
	if let Some(leg_index) = element_data.get_setting_value::<usize>("leg_index") {
		thread::spawn(move || {
			if let Some(path) = Window::select_obj(None) {
				RobotConfig::get_mut().add_joint(leg_index, &path);
				Window::get().update_robot_config_synchronized();
				set_joint_as_actively_selected(Some([leg_index, RobotConfig::get().legs()[leg_index].len() - 1]));
			}
		});
	}
}

/// Show the tooltip to replace or remove one of the joints' mesh.
pub(super) fn show_joint_replacement_menu(menu_position:[isize; 2], element_data:&mut DrawableData) {

	// Find the leg and joint to modify.
	if let Some(leg_index) = element_data.get_setting_value::<usize>("leg_index") {
		if let Some(joint_index) = element_data.get_setting_value::<usize>("joint_index") {
			Window::get().show_mesh_replacement_tooltip(
				menu_position,
				Rc::new(move |_,_,_,_| replace_joint(leg_index, joint_index)),
				Some(Rc::new(move |_,_,_,_| duplicate_joint(leg_index, joint_index))),
				Rc::new(move |_,_,_,_| remove_joint(leg_index, joint_index))
			);
		}
	}
}

/// Replace one of the joints' mesh.
fn replace_joint(leg_index:usize, joint_index:usize) {
	thread::spawn(move ||{
		if let Some(path) = Window::select_obj(None) {
			let robot_config:&mut RobotConfig = RobotConfig::get_mut();
			if Some(&path) != robot_config.get_joint(leg_index, joint_index).map(|joint| joint.obj()) {
				robot_config.set_joint(leg_index, joint_index, &path);
				Window::get().update_robot_config_synchronized();
			}
			set_joint_as_actively_selected(Some([leg_index, joint_index]));
		}
	});
}

/// Duplicate one of the joints to a new tree.
fn duplicate_joint(leg_index:usize, joint_index:usize) {
	let robot_config:&mut RobotConfig = RobotConfig::get_mut();
	if leg_index < robot_config.legs().len() && joint_index < robot_config.legs()[leg_index].len() {
		let new_joints:Vec<LegConfig> = robot_config.legs()[leg_index][joint_index..].to_vec();
		robot_config.legs_mut().push(new_joints);
		Window::get().update_robot_config_synchronized();
	}
}

/// Remove one of the joints' mesh entirely from the robot config.
fn remove_joint(leg_index:usize, joint_index:usize) {
	let robot_config:&mut RobotConfig = RobotConfig::get_mut();
	robot_config.remove_joint(leg_index, joint_index);
	set_joint_as_actively_selected(None);
}