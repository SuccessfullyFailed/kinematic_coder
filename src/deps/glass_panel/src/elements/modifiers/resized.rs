use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct Resized {
	data_set:DrawableData
}
impl Resized {

	/// Create a new Resized.
	pub fn new(width:usize, height:usize, children:Vec<&dyn Drawable>) -> Resized {
		Resized {
			data_set: DrawableData::new::<usize>(vec![("width", width), ("height", height)], children)
		}
	}
}
impl Drawable for Resized {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("resize")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Resized {
			data_set: self.data_set.clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[
			0,
			0,
			self.data_set().get_setting_value_or::<usize>("width", 0),
			self.data_set().get_setting_value_or::<usize>("height", 0)
		]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {

		// Create a new buffer.
		let buffer:DrawBuffer = self.draw_children();
		let width:usize = self.data_set().get_setting_value_or::<usize>("width", 0);
		let height:usize = self.data_set().get_setting_value_or::<usize>("height", 0);
		let scale_x:f32 = (width  as f32) / (buffer.width  as f32);
		let scale_y:f32 = (height as f32) / (buffer.height as f32);
		let mut rescaled:DrawBuffer = DrawBuffer::new(vec![0x00000000; width * height], width, height);

		// Always use largest image as source to prevent holes in the image.
		if scale_x + scale_y >= 2.0 {
			for y in 0..rescaled.height {
				for x in 0..rescaled.width {
					let source_x:usize = ((x as f32) / scale_x).floor() as usize;
					let source_y:usize = ((y as f32) / scale_y).floor() as usize;
					if rescaled.data[(y * width) + x] == 0x00000000 {
						let source_index:usize = (source_y * buffer.width) + source_x;
						let target_index:usize = (y * rescaled.width) + x;
						if target_index < rescaled.data.len() && source_index < buffer.data.len() {
							rescaled.data[target_index] = buffer.data[source_index];
						}
					}
				}
			}
		} else {
			for y in 0..buffer.height {
				for x in 0..buffer.width {
					let target_x:usize = ((x as f32) * scale_x).floor() as usize;
					let target_y:usize = ((y as f32) * scale_y).floor() as usize;
					if rescaled.data[(target_y * width) + target_x] == 0x00000000 {
						let source_index:usize = (y * buffer.width) + x;
						let target_index:usize = (target_y * rescaled.width) + target_x;
						if target_index < rescaled.data.len() && source_index < buffer.data.len() {
							rescaled.data[target_index] = buffer.data[source_index];
						}
					}
				}
			}
		}

		rescaled
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