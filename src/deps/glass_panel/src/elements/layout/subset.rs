use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct SubSet {
	data_set:DrawableData
}
impl SubSet {

	/// Create a new SubSet.
	pub fn new(x:usize, y:usize, w:usize, h:usize, children:Vec<&dyn Drawable>) -> SubSet {
		SubSet {
			data_set: DrawableData::new::<usize>(vec![("x", x), ("y", y), ("w", w), ("h", h)], children)
		}
	}
}
impl Drawable for SubSet {

	/* USAGE METHODS */

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("subset")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(SubSet {
			data_set: self.data_set.clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[
			0,
			0,
			self.data_set().get_setting_value_or::<usize>("w", 0),
			self.data_set().get_setting_value_or::<usize>("h", 0)
		]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		self.draw_children().take(
			self.data_set().get_setting_value_or::<usize>("x", 0),
			self.data_set().get_setting_value_or::<usize>("y", 0),
			self.data_set().get_setting_value_or::<usize>("w", 0),
			self.data_set().get_setting_value_or::<usize>("h", 0)
		)
	}

	/// Update and handle all own and child listeners.
	fn update_listeners(&mut self, mouse_position:&[usize; 2], mouse_down:&[bool; 2], initial:&[bool; 2]) {
		if self.data_set().get_setting_value_or::<bool>("block_listeners", false) { return; }
		let bounds:[usize; 4] = ["x", "y", "w", "h"].iter().map(|property| self.data_set().get_setting_value_or::<usize>(property, 0)).collect::<Vec<usize>>().try_into().unwrap();
		let relative_mouse_position:[isize; 2] = [mouse_position[0] as isize - self.data_set().absolute_position()[0] as isize, mouse_position[1] as isize - self.data_set().absolute_position()[1] as isize];
		if relative_mouse_position[0] < bounds[2] as isize && relative_mouse_position[1] < bounds[3] as isize {
			let mouse_position:[usize; 2] = [mouse_position[0] + bounds[0], mouse_position[1] + bounds[1]];
			self.data_set_mut().update_listeners(&mouse_position, mouse_down, initial);
			self.data_set_mut().children_mut().iter_mut().for_each(|child| child.update_listeners(&mouse_position, mouse_down, initial));
		}
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