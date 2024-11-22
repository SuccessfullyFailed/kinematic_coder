use crate::{ Drawable, DrawableData, DrawBuffer };
use std::error::Error;

pub struct Png {
	image_cache:DrawBuffer,
	path_cache:String,
	data_set:DrawableData
}
impl Png {

	/// Create a new rectangle.
	pub fn new(image:&str, children:Vec<&dyn Drawable>) -> Result<Png, Box<dyn Error>> {
		Ok(Png {
			image_cache: Png::read(image)?,
			path_cache: String::from(image),
			data_set: DrawableData::new::<String>(vec![("image", String::from(image))], children)
		})
	}

	/// If the image exists, load the image using the lodepng library. Otherwise return an empty buffer.
	fn read(file:&str) -> Result<DrawBuffer, Box<dyn Error>> {
		use image::{ RgbaImage, io::Reader };
		use std::path::Path;

		if Path::new(file).exists() {
			let read_image:RgbaImage = Reader::open(file)?.decode()?.to_rgba8();
			let data:Vec<u32> = read_image.pixels().map(|rgba| (rgba[3] as u32) << 24 | (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | rgba[2] as u32).collect::<Vec<u32>>();
			Ok(DrawBuffer::new(data, read_image.width() as usize, read_image.height() as usize))
		} else {
			Err(format!("Could not read png data from file '{file}'").into())
		}
	}
}
impl Drawable for Png {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("png")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Png {
			image_cache: self.image_cache.clone(),
			path_cache: String::from(&self.path_cache),
			data_set: self.data_set().clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[0, 0, self.image_cache.width, self.image_cache.height]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {

		// Only update the image if the path has changes.
		let image_path:String = self.data_set().get_setting_value_or::<String>("image", String::from(&self.path_cache));
		if image_path != self.path_cache {
			self.image_cache = Png::read(&image_path).unwrap_or(DrawBuffer::new(Vec::new(), 0, 0));
			self.path_cache = image_path;
		}

		// Create and return buffer.
		let mut buffer:DrawBuffer = self.image_cache.clone();
		buffer.append(self.draw_children())
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