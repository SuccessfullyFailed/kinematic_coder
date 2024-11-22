use glass_panel::{ Drawable, ListenerType, ListenerCallback, elements::{ Rectangle, Col, Text, Id, VisibilityToggler, Positioned, Border } };
use super::super::Window;
use std::rc::Rc;

impl Window {
	
	/// Let the user select and OBJ file and return its path.
	pub(crate) fn select_obj(dir:Option<&str>) -> Option<String> {
		super::file_selector::select_file(dir, vec![("obj", &["obj"])])
	}

	/// Create the mesh replacement menu.
	pub(crate) fn create_mesh_replacement_tooltip(&self) -> Id {
		Id::new("mesh_replacement_tooltip", vec![
			&VisibilityToggler::new(false, vec![
				&Rectangle::new(self.setting::<usize>("window_width"), self.setting::<usize>("window_width"), self.setting::<u32>("color_tooltip_shadow"), vec![
					&Positioned::new(0, 0, vec![
						&Border::new(1, self.setting("color_highlight"), vec![
							&Id::new("mesh_replacement_tooltip_contents", vec![])
						])
					])
				])
			])
		])
	}

	/// Show a mesh replacement menu at the given location.
	pub(crate) fn show_mesh_replacement_tooltip(&mut self, location:[isize; 2], replace_fn:ListenerCallback, duplicate_fn:Option<ListenerCallback>, delete_fn:ListenerCallback) {

		// Suppress tree listeners.
		self.window_mut().source_mut().child_by_name_mut("#robot_components_tree").unwrap().suppress_listeners();

		// Shift location.
		let location:[usize; 2] = [if location[0] > 1 { location[0] as usize - 1 } else { 0 }, if location[1] > 1 { location[1] as usize - 1 } else { 0 }];

		// Create buttons.
		let button_color:u32 = self.setting("color_highlight");
		let button_texts:Vec<Text> = vec![self.default_text("Replace"), self.default_text("Delete"), self.default_text(if duplicate_fn.is_some() { "Duplicate" } else { "" }), self.default_text(""), self.default_text("Cancel")];
		let button_sizes:Vec<[usize; 4]> = button_texts.iter().map(|button| button.position()).collect::<Vec<[usize; 4]>>();
		let button_shared_width:usize = button_sizes.iter().map(|size| size[2]).reduce(|a, b| a.max(b)).unwrap();
		let button_shared_height:usize = button_sizes.iter().map(|size| size[3]).reduce(|a, b| a + b).unwrap();
		let mut buttons:Vec<Rectangle> = (0..button_texts.len()).map(|index|
			Rectangle::new(button_shared_width, button_sizes[index][3], button_color, vec![&button_texts[index]])
		).collect::<Vec<Rectangle>>();

		// Add listeners to buttons.
		buttons[0].add_listener("MeshReplacementToolTipReplaceButtonClick", ListenerType::LeftDown, Rc::new(move |a,b,c,d| {
			replace_fn(a, b, c, d);
			let ui:&mut Window = Window::get();
			ui.hide_mesh_replacement_tooltip();
			ui.update_robot_config_synchronized();
		}));
		buttons[1].add_listener("MeshReplacementToolTipDeleteButtonClick", ListenerType::LeftDown, Rc::new(move |a,b,c,d| {
			delete_fn(a, b, c, d);
			let ui:&mut Window = Window::get();
			ui.hide_mesh_replacement_tooltip();
			ui.update_robot_config_synchronized();
		}));
		if let Some(duplicate_function) = duplicate_fn {
			buttons[2].add_listener("MeshReplacementToolTipDuplicateButtonClick", ListenerType::LeftDown, Rc::new(move |a,b,c,d| {
				duplicate_function(a, b, c, d);
				let ui:&mut Window = Window::get();
				ui.hide_mesh_replacement_tooltip();
				ui.update_robot_config_synchronized();
			}));
		}

		// Create full tooltip.
		let tooltip:Rectangle = Rectangle::new(button_shared_width, button_shared_height, button_color, vec![
			&Col::new(
				buttons.iter().map(|button| button as &dyn Drawable).collect::<Vec<&dyn Drawable>>()
			)
		]);

		// Set tooltip in window.
		self.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip positioned").unwrap().data_set_mut().set_settings_values::<usize>(vec![("x", location[0]), ("y", location[1])]);
		self.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip visibility_toggler").unwrap().data_set_mut().set_setting_value::<bool>("visible", true);
		self.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip_contents").unwrap().set_children(vec![&tooltip]);

		// Add listener to remove tooltip when clicking background.
		self.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip").unwrap().unsuppress_listeners();
		self.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip").unwrap().add_listener("MeshReplacementToolTipRemoveToolTip", ListenerType::LeftDown, Rc::new(|_,_,_,_| Window::get().hide_mesh_replacement_tooltip()));
	}

	/// Hide the mesh replacement tooltip.
	pub(crate) fn hide_mesh_replacement_tooltip(&mut self) {

		// Schedule UI update.
		Window::get().execute_post_listener(&|| {
			let ui:&mut Window = Window::get();
			ui.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip").unwrap().suppress_listeners();
			ui.window_mut().source_mut().child_by_name_mut("#mesh_replacement_tooltip visibility_toggler").unwrap().data_set_mut().set_setting_value::<bool>("visible", false);
			ui.window_mut().source_mut().child_by_name_mut("#robot_components_tree").unwrap().unsuppress_listeners();
		});
	}
}