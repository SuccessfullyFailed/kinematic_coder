use crate::{ Drawable, DrawableData, DrawBuffer, elements::Positioned };

pub struct Col {
	data_set:DrawableData
}
impl Col {

	/// Create a new Col.
	pub fn new(children:Vec<&dyn Drawable>) -> Col {

		// Create a new struct.
		let positioned_children:Vec<Positioned> = children.iter().map(|child| Positioned::new(0, 0, vec![*child])).collect::<Vec<Positioned>>();
		let mut col:Col = Col {
			data_set: DrawableData::new::<usize>(vec![], positioned_children.iter().map(|child| child as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
		};

		// Trigger an initial draw to update the position.
		col.draw();

		// Return the struct.
		col
	}
}
impl Drawable for Col {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("col")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Col {
			data_set: self.data_set().clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		let children_positions:Vec<[usize; 4]> = self.data_set().children().iter().map(|child| child.position()).collect::<Vec<[usize; 4]>>();
		if children_positions.is_empty() {
			[0; 4]
		} else {
			let last_child_pos:&[usize; 4] = &children_positions[children_positions.len() - 1];
			[
				0,
				0,
				children_positions.iter().map(|position| position[0] + position[2]).max().unwrap(),
				last_child_pos[1] + last_child_pos[3]
			]
		}
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		let mut y:usize = 0;
		let children:&mut Vec<Box<dyn Drawable>> = self.data_set_mut().children_mut();
		for child in children {
			child.data_set_mut().set_setting_value::<usize>("y", y);
			let child_pos:[usize; 4] = child.position();
			y += child_pos[3];
		}
		self.draw_children()
	}



	/* CHILD ELEMENT METHODS */
	
	/// Add a child element.
	fn add_child(&mut self, child:&dyn Drawable) {
		self.data_set_mut().add_child(&Positioned::new(0, 0, vec![child]));
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