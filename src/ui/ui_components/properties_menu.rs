use glass_panel::{ Drawable, ListenerType, elements::{ Rectangle, Row, Col, Id, Class, Border, Centered, Text } };
use crate::robot_configuration::RobotConfig;

use super::super::Window;
use std::rc::Rc;

impl Window {

	/* GENERAL PROPERTIES MENU METHODS */

	/// Create the robot properties menu.
	pub(crate) fn create_properties_menu(&self) -> Id {

		// Create the menu tab selectors.
		let tab_selector_border_color:u32 = self.setting("color_background");
		let tab_selector_color:u32 = self.setting("color_foreground");
		let tab_selector_width:usize = self.setting("menu_tab_selector_width");
		let mut tab_selectors:Vec<Id> = self.menu_tab_names().iter().map(|name|
			Id::new(name, vec![
				&Border::new(1, tab_selector_border_color, vec![
					&Rectangle::new(tab_selector_width - 2, tab_selector_width - 2, tab_selector_color, vec![
						&Centered::new(true, true, tab_selector_width - 2, tab_selector_width - 2, vec![
							&Text::new(self.font(), &name[0..1], tab_selector_width - 2, self.setting("color_text")) // ToDo: Replace by icons
						])
					])
				])
			])
		).collect::<Vec<Id>>();

		// Add listeners to the selectors.
		for (index, selector) in tab_selectors.iter_mut().enumerate() {
			selector.data_set_mut().set_setting_value::<usize>("tab_index", index);
			selector.add_listener("RobotConfigTabSelection", ListenerType::LeftDown, Rc::new(|_,_,_, data| {
				let tab_index:usize = data.get_setting_value::<usize>("tab_index").unwrap();
				let ui:&mut Window = Window::get();
				*ui.active_tab_mut() = tab_index;
				ui.update_robot_config_synchronized();
			}));
		}

		// Create the complete menu item.
		let scroll_view_width:usize = self.setting("menu_content_width");
		let tabs_width:usize = self.setting::<usize>("menu_properties_content_width");
		let scroll_view_height:usize = self.setting("menu_robot_properties_menu_height");
		Id::new("robot_properties_menu", vec![
			&Rectangle::new(scroll_view_width, scroll_view_height, 0, vec![

				// Tab selectors.
				&Row::new(vec![
					&Id::new("tab_selectors", vec![
						&Col::new(
							tab_selectors.iter().map(|tab_selector| tab_selector as &dyn Drawable).collect::<Vec<&dyn Drawable>>()
						)
					]),
					&Rectangle::new(tabs_width, scroll_view_height, 0x00000000, vec![
						&Id::new("tab_contents", vec![]) // Populates in update method.
					])
				])
			])
		])
	}

	/// Update the robot properties menu tabs and values after all listeners have been processed.
	pub(crate) fn update_robot_properties_menu_synchronized(&mut self) {
		Window::get().execute_post_listener(&|| Window::get().update_robot_properties_menu_now());
	}

	/// Update the robot properties menu tabs and values.
	pub(crate) fn update_robot_properties_menu_now(&mut self) {

		// Get required settings.
		let active_tab:usize = *self.active_tab();
		let tab_selector_color:u32 = self.setting("color_foreground");
		let active_tab_selector_color:u32 = self.setting("color_highlight");

		// Get elements to update.
		let robot_properties_menu:&mut dyn Drawable = self.element_mut("#robot_properties_menu");

		// Update tab selector colors.
		for (index, selector) in robot_properties_menu.child_by_name_mut("#tab_selectors col").unwrap().children_mut().iter_mut().enumerate() {
			let color:u32 = if index == active_tab { active_tab_selector_color } else { tab_selector_color };
			selector.child_by_name_mut("rectangle").unwrap().data_set_mut().set_setting_value::<u32>("color", color);
		}
		
		// Create the tab contents.
		let title_scale:f32 = 1.3;
		let tab_contents:Col = Col::new(vec![
			&self.scaled_text(&self.menu_tab_names()[*self.active_tab()], title_scale),
			&(match self.active_tab_name() {
				"Displacement" => self.create_properties_tab_displacement(),
				"Motor" => self.create_properties_tab_motor(),
				"Kinematics" => self.create_properties_tab_kinematics(),
				"Controller" => self.create_properties_tab_controller(),
				_ => Col::new(vec![&self.default_text("Coming soon")])
			})
		]);
		self.window_mut().source_mut().child_by_name_mut("#tab_contents").unwrap().set_children(vec![&tab_contents]);
	}



	/* SPECIFIC TAB METHODS */

	/// Create the displacement tab of the properties menu.
	pub(super) fn create_properties_tab_displacement(&self) -> Col {
		use super::properties_menu_listener_handlers::*;

		// Create input elements.
		let input_elements:Vec<Class> = match &self.selected_joint() {
			Some(_) => vec![self.create_property_input_float_vec("Position", &|| get_leg_position().to_vec(), &set_leg_position)],
			None => Vec::new()
		};

		// Return column of inputs.
		Col::new(input_elements.iter().map(|element| element as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
	}

	/// Create the motor tab of the properties menu.
	pub(super) fn create_properties_tab_motor(&self) -> Col {
		use super::properties_menu_listener_handlers as handlers;

		// Create input elements.
		let mut input_elements:Vec<Class> = Vec::new();
		if let Some(selected_joint_indexes) = &self.selected_joint() {
			if let Some(selected_joint) = RobotConfig::get().get_joint(selected_joint_indexes[0], selected_joint_indexes[1]) {
				if selected_joint.motor().is_some() {
					input_elements = vec![
						self.create_property_input_group("Motor position", vec![
							self.create_property_input_float_vec("Position", &|| handlers::get_motor_position().to_vec(), &handlers::set_motor_position)
						]),
						self.create_property_input_spacer(),
						Class::new("MotorRotationForm", vec![
							&self.create_property_input_group("Motor rotation", vec![
								self.create_property_input_str_list("Axis", &|| (handlers::get_motor_rotation_axis() as usize, vec!["X".to_string(), "Y".to_string(), "Z".to_string()]), &|axis| handlers::set_motor_rotation_axis(axis as u8)),
								self.create_property_input_float("Default", &handlers::get_motor_default_rotation, &handlers::set_motor_default_rotation),
								self.create_property_input_float_vec("Range", &|| handlers::get_motor_rotation_range().to_vec(), &handlers::set_motor_rotation_range),
								self.create_property_input_spacer(),
								self.create_property_input_float("Current", &handlers::get_motor_current_rotation, &handlers::set_motor_current_rotation),
							])
						])
					];
				} else {
					input_elements = vec![
						self.create_property_input_button("+Motor", &handlers::add_motor)
					];
				}
			}
		}

		// Return column of inputs.
		Col::new(input_elements.iter().map(|element| element as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
	}

	/// Create the kinematics tab of the properties menu.
	pub(super) fn create_properties_tab_kinematics(&self) -> Col {
		use super::properties_menu_listener_handlers as handlers;

		// Create input elements.
		let mut input_elements:Vec<Class> = Vec::new();
		if let Some(selected_joint_indexes) = &self.selected_joint() {
			if let Some(selected_joint) = RobotConfig::get().get_joint(selected_joint_indexes[0], selected_joint_indexes[1]) {
				if selected_joint.kinematics_config().is_some() {
					input_elements = vec![
						self.create_property_input_group("Endpoint", vec![
							self.create_property_input_float_vec("Position", &|| handlers::get_kinematics_endpoint().to_vec(), &handlers::set_kinematics_endpoint)
						]),
						self.create_property_input_spacer(),
						self.create_property_input_group("Step configuration", vec![
							self.create_property_input_float_vec("Position", &|| handlers::get_kinematics_step_position().to_vec(), &handlers::set_kinematics_step_position),
							self.create_property_input_float("Distance", &handlers::get_kinematics_step_distance, &handlers::set_kinematics_step_distance),
							self.create_property_input_float("Height", &handlers::get_kinematics_step_height, &handlers::set_kinematics_step_height)
						])
					];
				} else {
					input_elements = vec![
						self.create_property_input_button("+Kinematics", &handlers::add_kinematics)
					];
				}
			}
		}

		// Return column of inputs.
		Col::new(input_elements.iter().map(|element| element as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
	}

	/// Create the controller tab of the properties menu.
	pub(super) fn create_properties_tab_controller(&self) -> Col {
		use super::properties_menu_listener_handlers as handlers;

		Col::new(vec![
			&self.create_property_input_group("Controller", vec![
				self.create_property_input_bool("Realtime", &handlers::get_kinematics_realtime, &handlers::set_kinematics_realtime),
				self.create_property_input_float("Speed", &handlers::get_kinematics_speed, &handlers::set_kinematics_speed),
				self.create_property_input_float("Strafe", &handlers::get_kinematics_strafe, &handlers::set_kinematics_strafe),
			])
		])
	}
}