use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct Colored {
	data_set:DrawableData
}
impl Colored {

	/// Create a new Colored.
	pub fn new(color:u32, children:Vec<&dyn Drawable>) -> Colored {
		Colored {
			data_set: DrawableData::new::<u32>(vec![("color", color)], children)
		}
	}
}
impl Drawable for Colored {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("colored")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Colored {
			data_set: self.data_set.clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		self.children_bounding_box()
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


// ToDo: Add unit tests