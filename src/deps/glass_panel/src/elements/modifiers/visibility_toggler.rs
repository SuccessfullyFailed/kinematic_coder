use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct VisibilityToggler {
	data_set:DrawableData
}
impl VisibilityToggler {

	/// Create a new VisibilityToggler.
	pub fn new(visible:bool, children:Vec<&dyn Drawable>) -> VisibilityToggler {
		VisibilityToggler {
			data_set: DrawableData::new::<bool>(vec![("visible", visible)], children)
		}
	}
}
impl Drawable for VisibilityToggler {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("visibility_toggler")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(VisibilityToggler {
			data_set: self.data_set.clone()
		})
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		if self.data_set().get_setting_value::<bool>("visible").unwrap_or(true) {
			self.draw_children()
		} else {
			DrawBuffer::new(Vec::new(), 0, 0)
		}
	}

	/// Update and handle all own and child listeners.
	fn update_listeners(&mut self, mouse_position:&[usize; 2], mouse_down:&[bool; 2], initial:&[bool; 2]) {
		if !self.data_set().get_setting_value_or::<bool>("visible", true) { return; }
		if self.data_set().get_setting_value_or::<bool>("block_listeners", false) { return; }
		self.data_set_mut().update_listeners(mouse_position, mouse_down, initial);
		self.data_set_mut().children_mut().iter_mut().for_each(|child| child.update_listeners(mouse_position, mouse_down, initial));
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