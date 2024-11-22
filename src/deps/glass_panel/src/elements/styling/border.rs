use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct Border {
	data_set:DrawableData
}
impl Border {

	/// Create a new rectangle.
	pub fn new(size:usize, color:u32, children:Vec<&dyn Drawable>) -> Border {
		Border {
			data_set: DrawableData::new(vec![("size", size as u32), ("color", color)], children)
		}
	}
}
impl Drawable for Border {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("border")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Border {
			data_set: self.data_set().clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		let children_bounding:[usize; 4] = self.children_bounding_box();
		let border_size:usize = self.data_set().get_setting_value_or::<u32>("size", 0) as usize;
		[
			0,
			0,
			children_bounding[0] + children_bounding[2] + (2 * border_size), 
			children_bounding[1] + children_bounding[3] + (2 * border_size)
		]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		let position:[usize; 4] = self.position();
		let border_size:usize = self.data_set().get_setting_value_or::<u32>("size", 0) as usize;
		let color:u32 = self.data_set().get_setting_value_or::<u32>("color", 0);
		
		let mut buffer:DrawBuffer = DrawBuffer::new(vec![0x00000000; position[2] * position[3]], position[2], position[3]);
		for i in 0..border_size {
			for x in 0..buffer.width {
				buffer.data[(i * buffer.width) + x] = color;
				buffer.data[((buffer.height - i - 1) * buffer.width) + x] = color;
			}
			for y in 0..buffer.height {
				buffer.data[(y * buffer.width) + i] = color;
				buffer.data[(y * buffer.width) + (buffer.width - i - 1)] = color;
			}
		}
		
		
		buffer.append_at(self.draw_children(), border_size as isize, border_size as isize)
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

// ToDo: Add unit tests