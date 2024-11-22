use glass_panel::{ Drawable, ListenerType, elements::{ Id, Class, Row, Col, Rectangle, Border, Text, Centered } };
use crate::ui::Window;
use std::rc::Rc;

impl Window {

	/* INPUT GROUP METHODS */

	/// Create a new input group.
	pub(crate) fn create_toolbar(&self) -> Id {
		use crate::{ storage, robot_configuration::RobotConfig };
		use super::file_selector::save_file;
		use std::thread;

		// Size calculations.
		let toolbar_width:usize = self.setting::<usize>("window_width");
		let toolbar_height:usize = self.setting::<usize>("toolbar_height");
		let border_color:u32 = self.setting::<u32>("color_highlight");
		let spacer:Rectangle = Rectangle::new(5, 0, 0, vec![]);

		// Create toolbar element.
		Id::new("toolbar", vec![
			&Col::new(vec![
				&Rectangle::new(toolbar_width, toolbar_height - 1, 0, vec![
					&Border::new(1, 0, vec![
						&Row::new(vec![
							&self.create_toolbar_button("Save", &RobotConfig::save),
							&spacer,
							&self.create_toolbar_button("Save as", &|| {
								thread::spawn(||{
									if let Some(new_dir) = save_file(Some(&storage::projects_dir()), Vec::new()) {
										if let Err(error) = storage::store_project_at(&new_dir) {
											eprint!("COULD NOT SAVE ROBOT CONFIG TO FILE. {error}");
										}
									}
								});
							}),
							&spacer,
							&self.create_toolbar_button("Open", &|| Window::get().execute_post_listener(&|| Window::get().show_project_loading_tooltip()))
						])
					])
				]),
				&Rectangle::new(toolbar_width, 1, border_color, vec![])
			])
		])
	}

	fn create_toolbar_button(&self, text:&str, handler:&'static dyn Fn()) -> Class {

		// Size calculations.
		let button_height:usize = self.setting::<usize>("toolbar_height") - 9;
		let border_color:u32 = self.setting::<u32>("color_highlight");
		let text:Text = self.default_text(text);

		// Create button element.
		let mut button:Class = Class::new("toolbar_button", vec![
			&Border::new(1, border_color, vec![
				&Border::new(2, 0, vec![
					&Centered::new(true, true, text.position()[2], button_height, vec![
						&text
					])
				])
			])
		]);

		// Add listener.
		button.add_listener("ToolbarButtonListener", ListenerType::LeftDown, Rc::new(|_, _, _, _| handler()));

		// Return button element.
		button
	}
}