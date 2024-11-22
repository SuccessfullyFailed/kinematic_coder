use crate::{ Drawable, DrawableData, DrawBuffer, elements::Positioned };

pub struct Centered {
	data_set:DrawableData
}
impl Centered {

	/// Create a new Centered.
	pub fn new(center_horizontally:bool, center_vertically:bool, width:usize, height:usize, children:Vec<&dyn Drawable>) -> Centered {
		Centered {
			data_set: DrawableData::new::<usize>(
				vec![
					("center_horizontally", if center_horizontally { 1 } else { 0 }),
					("center_vertically", 	if center_vertically { 1 } else { 0 }),
					("width", width),
					("height", height)
				],
				vec![
					&Positioned::new(0, 0, children)
				]
			)
		}
	}
}
impl Drawable for Centered {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("centered")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Centered {
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

		// Figure out how much padding is required.
		let children_bounds:[usize; 4] = self.children_bounding_box();
		let data_set:&DrawableData = self.data_set();
		let outer_size:[usize; 2] = [
			data_set.get_setting_value_or::<usize>("width", 0),
			data_set.get_setting_value_or::<usize>("height", 0)
		];
		let center_axis:[bool; 2] = [
			data_set.get_setting_value_or::<usize>("center_horizontally", 0) == 1,
			data_set.get_setting_value_or::<usize>("center_vertically", 0) == 1
		];
		let padding:[isize; 2] = [
			if !center_axis[0] { 0 } else { outer_size[0] as isize - children_bounds[2] as isize },
			if !center_axis[1] { 0 } else { outer_size[1] as isize - children_bounds[3] as isize }
		];

		// Update 'Positioned' child.
		// TODO: Handle negative padding.
		let positioned_child_data_set:&mut DrawableData = self.data_set_mut().children_mut()[0].data_set_mut();
		positioned_child_data_set.set_setting_value::<usize>("x", if padding[0] > 0 { padding[0] as usize / 2 } else { 0 });
		positioned_child_data_set.set_setting_value::<usize>("y", if padding[1] > 0 { padding[1] as usize / 2 } else { 0 });

		// Draw children.
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


// TODO: Make unit tests.