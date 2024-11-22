use glass_panel::{ Drawable, DrawableData, ListenerType, elements::{ Centered, Class, Col, Rectangle, Row, Text, Border } };
use super::super::Window;
use std::rc::Rc;

impl Window {

	/* INPUT GROUP METHODS */

	/// Create a new input group.
	pub fn create_property_input_group(&self, name:&str, inputs:Vec<Class>) -> Class {
		let title_spacing:usize = 6;
		let border_width:usize = self.setting::<usize>("menu_properties_input_group_border_size");
		let group_width:usize = self.setting::<usize>("menu_properties_content_width");
		let group_height:usize = inputs.iter().map(|input| input.position()[3]).sum::<usize>() + self.setting::<usize>("default_font_height") + border_width * 2 + title_spacing;

		Class::new("input_group", vec![
			&Rectangle::new(group_width, group_height, self.setting::<u32>("color_highlight"), vec![
				&Border::new(border_width, 0, vec![
					&Col::new(vec![
						&self.default_text(name),
						&Rectangle::new(0, title_spacing, 0, Vec::new()),
						&Col::new(inputs.iter().map(|input| input as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
					])
				])
			])
		])
	}

	/// Create a spacer to separate inputs and input-groups.
	pub fn create_property_input_spacer(&self) -> Class {
		Class::new("Spacer", vec![
			&Rectangle::new(0, self.setting("menu_properties_input_spacer_height"), 0, Vec::new())
		])
	}



	/* BUTTON INPUT METHODS */

	/// Create a simple button with a listener.
	pub fn create_property_input_button(&self, text:&str, click_event:&'static dyn Fn()) -> Class {
		let button_width:usize = self.setting::<usize>("menu_properties_input_group_width");
		let button_height:usize = self.setting::<usize>("default_font_height") + 6;

		// Create button.
		let text:Text = self.default_text(text);
		let mut button:Rectangle = Rectangle::new(button_width, button_height, 0x66333333, vec![
			&Centered::new(true, true, button_width, button_height, vec![
				&text
			])
		]);

		// Create listener.
		button.add_listener("InputButtonClickEvent", ListenerType::LeftDown, Rc::new(|_,_,_,_| click_event()));

		// Return button
		Class::new("input_field", vec![&button])
	}



	/* BOOLEAN INPUT METHODS */

	/// Create a simple button with a listener.
	pub fn create_property_input_bool(&self, name:&str, value_getter:&'static dyn Fn() -> bool, value_setter:&'static dyn Fn(bool)) -> Class {

		// Create name box.
		let name_box:Rectangle = self.name_box(name);

		// Create value box.
		let row_height:usize = name_box.position()[3];
		let inner_value_box_size:usize = (self.setting::<usize>("default_font_height") as f32 * 0.6) as usize;
		let mut inner_box:Border = Border::new(1, self.setting::<u32>("color_highlight"), vec![
			&Border::new(2, self.setting::<u32>("color_foreground"), vec![
				&Class::new("checkbox_inner", vec![
					&Rectangle::new(inner_value_box_size, inner_value_box_size, self.setting::<u32>(if value_getter() { "color_detail" } else { "color_foreground" }), vec![])
				])
			])
		]);

		// Add value box listeners.
		inner_box.add_listener("InputFieldModify", ListenerType::LeftDown, Rc::new(|_,_,_, element_data| {
			let enabled:bool = !value_getter();
			value_setter(enabled);
			let inner_element:&mut dyn Drawable = element_data.children_mut()[0].child_by_name_mut(".checkbox_inner rectangle").unwrap();
			let color:u32 = Window::get().setting::<u32>(if enabled { "color_detail" } else { "color_foreground" });
			inner_element.data_set_mut().set_setting_value::<u32>("color", color);
		}));

		// Return full field.
		self.input_from_boxes(&name_box, &Centered::new(true, true, row_height, row_height, vec![&inner_box]))
	}



	/* STRING INPUT METHODS */

	/// Create an input field for a list of strings.
	pub(super) fn create_property_input_str_list(&self, name:&str, value_getter:&'static dyn Fn() -> (usize, Vec<String>), value_setter:&'static dyn Fn(usize)) -> Class {

		// Create name box.
		let name_box:Rectangle = self.name_box(name);

		// Create value boxes.
		let (selected_option_index, options) = value_getter();
		let tabs_width:usize = self.setting::<usize>("menu_properties_input_group_width");
		let value_box_outer_width:usize = ((tabs_width / 2) as f32 / options.len() as f32).floor() as usize;
		let value_box_inner_width:usize = value_box_outer_width - 2 * self.setting::<usize>("menu_properties_input_border_width");
		let value_boxes:Vec<Rectangle> = options.iter().enumerate().map(|(index, option)| {

			// Create element.
			let mut value_box:Rectangle = self.value_box(option);
			if index == selected_option_index {
				value_box.child_by_name_mut("text").unwrap().data_set_mut().set_setting_value::<u32>("color", Window::get().setting("color_detail"));
			}
			value_box.data_set_mut().set_settings_values::<usize>(vec![("width", value_box_outer_width), ("option_index", index)]);
			value_box.child_by_name_mut("centered").unwrap().data_set_mut().set_setting_value::<usize>("width", value_box_inner_width);

			// Add listeners.
			value_box.add_listener("SelectOption", ListenerType::LeftDown, Rc::new(|_, _, _, element_data:&mut DrawableData| {
				let option_index:usize = element_data.get_setting_value::<usize>("option_index").unwrap();
				element_data.children_mut()[0].child_by_name_mut("text").unwrap().data_set_mut().set_setting_value::<u32>("color", Window::get().setting("color_detail"));
				value_setter(option_index);
			}));

			// Return element.
			value_box
		}).collect::<Vec<Rectangle>>();

		// Create a row of all value boxes.
		let mut value_box_row:Row = Row::new(value_boxes.iter().map(|input| input as &dyn Drawable).collect::<Vec<&dyn Drawable>>());
		value_box_row.add_listener("DeselectOption", ListenerType::LeftDown, Rc::new(|_, _, _, element_data:&mut DrawableData| {
			for child in element_data.children_mut() {
				child.child_by_name_mut("text").unwrap().data_set_mut().set_setting_value::<u32>("color", Window::get().setting("color_text"));
			}
		}));

		// Return full field.
		self.input_from_boxes(&name_box, &value_box_row)
	}



	/* FLOAT INPUT METHODS */

	/// Create an input field for a float.
	pub(super) fn create_property_input_float(&self, name:&str, value_getter:&'static dyn Fn() -> f32, value_setter:&'static dyn Fn(f32)) -> Class {

		// Create name box.
		let name_box:Rectangle = self.name_box(name);

		// Create value box.
		let mut value_box:Rectangle = self.value_box(&value_getter().to_string());

		// Add value box listeners.
		value_box.add_listener("InputFieldModify", ListenerType::DragLeft, Rc::new(|position_shift,_,_, element_data| {
			value_setter(value_getter() - position_shift[1] as f32);
			Self::change_value_box_value(element_data, &value_getter().to_string());
		}));
		value_box.add_listener("InputFieldReset", ListenerType::RightDown, Rc::new(|_,_,_, element_data| {
			value_setter(0.0);
			Self::change_value_box_value(element_data, &value_getter().to_string());
		}));

		// Return full field.
		self.input_from_boxes(&name_box, &value_box)
	}

	/// Create an input field for a Vec<f32>.
	pub(super) fn create_property_input_float_vec(&self, name:&str, value_getter:&'static dyn Fn() -> Vec<f32>, value_setter:&'static dyn Fn(usize, f32)) -> Class {

		// Create name box.
		let name_box:Rectangle = self.name_box(name);

		// Create value boxes.
		let values:Vec<f32> = value_getter();
		let tabs_width:usize = self.setting::<usize>("menu_properties_input_group_width");
		let value_box_outer_width:usize = ((tabs_width / 2) as f32 / values.len() as f32).floor() as usize;
		let value_box_inner_width:usize = value_box_outer_width - 2 * self.setting::<usize>("menu_properties_input_border_width");
		let value_boxes:Vec<Rectangle> = values.iter().enumerate().map(|(index, value)| {
			
			// Create element.
			let mut value_box:Rectangle = self.value_box(&value.to_string());
			value_box.data_set_mut().set_setting_value::<usize>("width", value_box_outer_width);
			value_box.child_by_name_mut("centered").unwrap().data_set_mut().set_setting_value::<usize>("width", value_box_inner_width);

			// Add listeners.
			value_box.data_set_mut().set_setting_value::<usize>("input_index", index);
			value_box.add_listener("InputFieldModify", ListenerType::DragLeft, Rc::new(|position_shift,_,_, element_data| {
				let index:usize = element_data.get_setting_value::<usize>("input_index").unwrap();
				let value:f32 = value_getter()[index] - position_shift[1] as f32;
				value_setter(index, value);
				Self::change_value_box_value(element_data, &value_getter()[index].to_string());
			}));
			value_box.add_listener("InputFieldReset", ListenerType::RightDown, Rc::new(|_,_,_, element_data| {
				let index:usize = element_data.get_setting_value::<usize>("input_index").unwrap();
				value_setter(index, 0.0);
				Self::change_value_box_value(element_data, &value_getter()[index].to_string());
			}));

			// Return element.
			value_box
		}).collect::<Vec<Rectangle>>();

		// Return full field.
		self.input_from_boxes(&name_box, &Row::new(value_boxes.iter().map(|value_box| value_box as &dyn Drawable).collect::<Vec<&dyn Drawable>>()))
	}



	/* HELPER METHODS */

	/// Create an element given a name box and value box.
	fn input_from_boxes(&self, name_box:&dyn Drawable, value_box:&dyn Drawable) -> Class {
		Class::new("input_field", vec![
			&Row::new(vec![
				name_box,
				value_box
			])
		])
	}

	/// Create a name box.
	fn name_box(&self, name:&str) -> Rectangle {
		let tabs_width:usize = self.setting::<usize>("menu_properties_input_group_width");
		let input_height:usize = 24;
		Rectangle::new(tabs_width / 2, input_height, 0x00000000, vec![
			&self.default_text(name)
		])
	}

	/// Create a value box.
	fn value_box(&self, value:&str) -> Rectangle {
		let outer_size:[usize; 2] = [self.setting::<usize>("menu_properties_input_group_width") / 2, 24];
		let border_size:usize = self.setting::<usize>("menu_properties_input_border_width");
		let inner_size:[usize; 2] = [outer_size[0] - border_size * 2, outer_size[1] - border_size * 2];
		Rectangle::new(outer_size[0], outer_size[1], self.setting::<u32>("color_foreground"), vec![
			&Border::new(1, self.setting::<u32>("color_highlight"), vec![
				&Border::new(border_size - 1, 0, vec![
					&Centered::new(true, true, inner_size[0], inner_size[1], vec![
						&Class::new("ValText", vec![&self.default_text(value)])
					])
				])
			])
		])
	}

	/// Update the value text of a value box based on the element data. Mainly used in drag listeners.
	fn change_value_box_value(element_data:&mut DrawableData, value:&str) {
		element_data.children_mut()[0].child_by_name_mut(".ValText text").unwrap().data_set_mut().set_setting_value::<String>("text", value.to_string());
	}
}