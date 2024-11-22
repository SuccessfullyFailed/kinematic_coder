use crate::{ DrawBuffer, Drawable, DrawableData, DrawableDataSettingDataType };

pub struct Rectangle {
	data_set:DrawableData
}
impl Rectangle {

	/// Create a new rectangle.
	pub fn new(width:usize, height:usize, color:u32, children:Vec<&dyn Drawable>) -> Rectangle {
		Rectangle {
			data_set: DrawableData::new::<Vec<u8>>(vec![("width", width.to_bytes()), ("height", height.to_bytes()), ("color", color.to_bytes())], children)
		}
	}
}
impl Drawable for Rectangle {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("rectangle")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Rectangle {
			data_set: self.data_set().clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[0, 0, self.data_set().get_setting_value_or::<usize>("width", 0), self.data_set().get_setting_value_or::<usize>("height", 0)]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		let position:[usize; 4] = self.position();
		DrawBuffer::new(vec![self.data_set().get_setting_value_or::<u32>("color", 0); position[2] * position[3]], position[2], position[3]).append(self.draw_children())
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