use glass_panel::{ Drawable, ListenerType, ListenerCallback, elements::{ Rectangle, Positioned, Col, ScrollView, Id } };
use crate::robot_configuration::{ RobotConfig, LegConfig };
use glass_panel::DrawableData;
use super::super::Window;
use std::rc::Rc;

impl Window {

	/// Update all update-able elements in the UI after all listeners have been processed.
	pub(crate) fn update_robot_config_synchronized(&mut self) {
		Window::get().execute_post_listener(&|| Window::get().update_robot_config_now());
	}

	/// Update all update-able elements in the UI after all listeners have been processed.
	pub(crate) fn update_robot_config_now(&mut self) {
		self.update_robot_config_synchronized_tree_now();
		if let Err(error) = self.update_robot_config_in_scene_now() {
			eprintln!("{error}");
		}
		self.update_robot_properties_menu_now();
	}

	/// Update the robot components tree.
	fn update_robot_config_synchronized_tree_now(&mut self) {
		use super::robot_config_tree_listener_handlers::*;

		let robot_config:&RobotConfig = RobotConfig::get();

		// Throughout this function, many threads will be created and removed shortly.
		// This is not very efficient and should be replaced soon, but it is important to not remove UI elements from within a callback function stored in that UI element.
		// In general, removing the actively executed block from memory is a bad idea.
		
		// Create rows for tree.
		let mut tree_row_entries:Vec<Id> = Vec::new();
		if robot_config.body().is_none() {

			// Add body button.
			tree_row_entries.push(
				self.create_ui_robot_tree_row(0, "+ body", vec![
					("RobotTreeAddBody", ListenerType::LeftDown, Rc::new(|_,_,_,_| replace_body()))
				])
			);

		} else {

			// Body entry.
			tree_row_entries.push(
				self.create_ui_robot_tree_row(0, "Body", vec![
					("RobotTreeShowReplacementToolTip", ListenerType::LeftDown, Rc::new(|_, _, _, _| set_no_joints_selected())),
					("RobotTreeShowReplacementToolTip", ListenerType::RightDown, Rc::new(|_, absolute_position, _, _| show_body_replacement_menu(absolute_position)))
				])
			);

			// Loop through legs.
			for leg_index in 0..robot_config.legs().len() {
				let leg:&Vec<LegConfig> = &robot_config.legs()[leg_index];

				// Joint entries.
				for joint_index in 0..leg.len() {
					let mut joint_entry:Id = self.create_ui_robot_tree_row(1 + joint_index, &format!("Leg{leg_index} Joint{joint_index}"), vec![
						("RobotTreeSelectJoint", ListenerType::LeftDown, Rc::new(|_, _, _, element_data| select_clicked_joint(element_data))),
						("RobotTreeShowReplacementToolTip", ListenerType::RightDown, Rc::new(|_, absolute_position, _, element_data| show_joint_replacement_menu(absolute_position, element_data)))
					]);
					joint_entry.data_set_mut().set_settings_values::<usize>(vec![("leg_index", leg_index), ("joint_index", joint_index)]);
					tree_row_entries.push(joint_entry);
				}

				// Add joint button.
				let mut add_joint_button:Id = self.create_ui_robot_tree_row(1 + leg.len(), "+ joint", vec![
					("RobotTreeAddJoint", ListenerType::LeftDown, Rc::new(|_, _, _, element_data:&mut DrawableData| add_joint(element_data)))
				]);
				add_joint_button.data_set_mut().set_setting_value::<usize>("leg_index", leg_index);
				tree_row_entries.push(add_joint_button);
			}

			// Add leg button.
			tree_row_entries.push(
				self.create_ui_robot_tree_row(1, "+ leg", vec![
					("RobotTreeAddLeg", ListenerType::LeftDown, Rc::new(|_,_,_,_| add_leg()))
				])
			);
		}

		// Create the column list.
		let scroll_view_width:usize = self.setting("menu_content_width");
		let scroll_view_height:usize = self.setting("menu_robot_config_tree_height");
		let color_scrollbar_outer:u32 = self.setting("color_scrollbar_outer");
		let color_scrollbar_inner:u32 = self.setting("color_scrollbar_inner");
		let tree:ScrollView = ScrollView::new(scroll_view_width, scroll_view_height, color_scrollbar_outer, color_scrollbar_inner, vec![
			&Col::new(tree_row_entries.iter().map(|row| row as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
		]);

		// Set them in te tree.
		self.element_mut("#robot_components_tree").set_children(vec![&tree]);
		
	}

	/// Create a singular rot in the robot components tree.
	fn create_ui_robot_tree_row(&self, depth:usize, text:&str, listeners:Vec<(&str, ListenerType, ListenerCallback)>) -> Id {

		// Settings.
		let margin:usize = self.setting::<usize>("menu_content_margin") * 3;

		// Create element.
		let mut row:Id = Id::new(&text.replace(' ', ""), vec![
			&Rectangle::new(self.setting::<usize>("menu_content_width") - 10, 20, self.setting("color_foreground"), vec![
				&Positioned::new(3 + depth * margin, 0, vec![
					&self.default_text(text)
				])
			])
		]);

		// Add listeners.
		row.add_listener("RobotComponentRowHover", ListenerType::Hover, Rc::new(|_,_,_,element_data:&mut DrawableData| { element_data.set_setting_value::<u32>("color", Window::get().setting::<u32>("color_highlight")); }));
		row.add_listener("RobotComponentRowNonHover", ListenerType::NonHover, Rc::new(|_,_,_,element_data:&mut DrawableData| { element_data.set_setting_value::<u32>("color", Window::get().setting::<u32>("color_foreground")); }));
		for listener in listeners {
			row.add_listener(listener.0, listener.1, listener.2);
		}
			
		// Return.
		row
	}
}