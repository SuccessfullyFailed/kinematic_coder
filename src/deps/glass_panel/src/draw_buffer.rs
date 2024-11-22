use crate::{ Drawable, DrawableData };

#[derive(Clone)]
pub struct DrawBuffer {
	pub data:Vec<u32>,
	pub width:usize,
	pub height:usize
}
impl DrawBuffer {

	/// Create a new instance.
	pub fn new(data:Vec<u32>, width:usize, height:usize) -> DrawBuffer {
		DrawBuffer {
			data,
			width,
			height
		}
	}

	/// Append another image over the current one.
	pub fn append(&mut self, addition:DrawBuffer) -> DrawBuffer {
		self.append_at(addition, 0, 0)
	}
	
	/// Append another image over the current one offsetted by a position.
	pub fn append_at(&self, addition:DrawBuffer, x:isize, y:isize) -> DrawBuffer {
		let mut new_buffer:DrawBuffer = DrawBuffer::new(self.data[..].to_vec(), self.width, self.height);

		// Filter out additions with broken or negative negative sizes.
		let addition_data_len:usize = addition.data.len();
		if addition_data_len < addition.width || addition_data_len < addition.height || addition_data_len != addition.width * addition.height {
			return new_buffer;
		}
		
		// Loop through pixels in the addition.
		for offset_y in 0..addition.height {
			for offset_x in 0..addition.width {
				let source_color:u32 = addition.data[(offset_y * addition.width) + offset_x];
				if source_color & 0xFF000000 != 0x00000000 {

					// Calculate the target location.
					let target_pos:[isize; 2] = [x + offset_x as isize, y + offset_y as isize];
					if target_pos[0] >= 0 && target_pos[1] >= 0 {
						let target_pos:[usize; 2] = [target_pos[0] as usize, target_pos[1] as usize];
						if target_pos[0] < new_buffer.width && target_pos[1] < new_buffer.height {
							let target_color:u32 = new_buffer.data[(target_pos[1] * self.width) + target_pos[0]];

							// Mix colors.
							let mix:f32 = 1.0 / 255.0 * ((source_color >> 24) as f32);
							let new_color:u32 = if mix < 0.1 {
								target_color
							} else if mix > 0.9 {
								source_color
							} else {
								(((source_color >> 24) + (target_color >> 24)) / 2) << 24 |
								(((((source_color >> 16) & 0xFF) as f32) * mix + (((target_color >> 16) & 0xFF) as f32) * (1.0 - mix)) as u32) << 16 |
								(((((source_color >> 8) & 0xFF) as f32) * mix + (((target_color >> 8) & 0xFF) as f32) * (1.0 - mix)) as u32) << 8 |
								((((source_color & 0xFF) as f32) * mix + ((target_color & 0xFF) as f32) * (1.0 - mix)) as u32)
							};

							// Draw the pixel.
							new_buffer.data[(target_pos[1] * self.width) + target_pos[0]] = new_color;
						}
					}
				}
			}
		}
		new_buffer
	}

	/// Take a part of the buffer.
	pub fn take(&self, x:usize, y:usize, w:usize, h:usize) -> DrawBuffer {
		let mut new_buffer:DrawBuffer = DrawBuffer::new(vec![0x00000000; w * h], w, h);
		for offset_y in 0..h {
			for offset_x in 0..w {
				let source_coord:[usize; 2] = [offset_x + x, offset_y + y];
				if source_coord[0] < self.width && source_coord[1] < self.height {
					new_buffer.data[(offset_y * w) + offset_x] = self.data[(source_coord[1] * self.width) + source_coord[0]];
				}
			}
		}
		new_buffer
	}
}


impl Drawable for DrawBuffer {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("draw_buffer")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(DrawBuffer::new(self.data[..].to_vec(), self.width, self.height))
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[0, 0, self.width, self.height]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		self.clone()
	}

	

	/* DATA SET METHODS */

	/// Get a reference to the instances Data object.
	fn data_set(&self) -> &DrawableData {
		panic!("Cannot get data_set of DrawBuffer.")
	}
	
	/// Get a mutable reference to the instances Data object.
	fn data_set_mut(&mut self) -> &mut DrawableData {
		panic!("Cannot get data_set of DrawBuffer.")
	}
}