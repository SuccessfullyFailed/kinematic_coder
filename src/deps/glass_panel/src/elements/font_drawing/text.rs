use crate::{ Drawable, DrawableData, DrawableDataSettingDataType, DrawBuffer };
use super::Font;

pub struct Text {
	font:Font,
	data_set:DrawableData
}
impl Text {

	/// Create a new rectangle.
	pub fn new(font:&Font, text:&str, line_height:usize, color:u32) -> Text {
		Text {
			font: font.clone(),
			data_set: DrawableData::new::<Vec<u8>>(vec![
				("text", String::from(text).to_bytes()),
				("color", color.to_bytes()),
				("line_height", line_height.to_bytes())
			], vec![])
		}
	}
}
impl Drawable for Text {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("text")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Text {
			font: self.font.clone(),
			data_set: self.data_set().clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		self.font.bounding_rect_of(
			&self.data_set().get_setting_value_or::<String>("text", String::new()),
			self.data_set().get_setting_value_or::<usize>("line_height", 0)
		)
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		let buffer:DrawBuffer = self.font.draw_text(
			&self.data_set().get_setting_value_or::<String>("text", String::new()),
			self.data_set().get_setting_value_or::<usize>("line_height", 0),
			self.data_set().get_setting_value_or::<u32>("color", 0x00000000)
		);
		self.data_set_mut().set_setting_value::<usize>("width", buffer.width);
		buffer
	}

	

	/* DATA SET METHODS */

	/// Get a reference to the instances Data object.
	fn data_set(&self) -> &DrawableData {
		&self.data_set
	}

	/// Get a mutable reference to the instances Data object.
	fn data_set_mut(&mut self) -> &mut DrawableData {
		&mut self.data_set
	}
}



/* This drawable does not have specific unit tests due to being dependant on so many variables. I might implement this later, but does not seem worth the time at the moment. It will also become very obvious when testing other things */