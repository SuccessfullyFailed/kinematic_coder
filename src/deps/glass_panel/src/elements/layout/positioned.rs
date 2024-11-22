use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct Positioned {
	data_set:DrawableData
}
impl Positioned {

	/// Create a new Positioned.
	pub fn new(x:usize, y:usize, children:Vec<&dyn Drawable>) -> Positioned {
		Positioned {
			data_set: DrawableData::new::<usize>(vec![("x", x), ("y", y)], children)
		}
	}
}
impl Drawable for Positioned {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("positioned")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Positioned {
			data_set: self.data_set.clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		let children_bounds:[usize; 4] = self.children_bounding_box();
		[
			self.data_set().get_setting_value_or::<usize>("x", 0),
			self.data_set().get_setting_value_or::<usize>("y", 0),
			children_bounds[0] + children_bounds[2],
			children_bounds[1] + children_bounds[3]
		]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		self.draw_children()
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