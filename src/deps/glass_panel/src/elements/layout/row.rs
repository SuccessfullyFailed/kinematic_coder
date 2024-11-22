use crate::{ Drawable, DrawableData, DrawBuffer, elements::Positioned };

pub struct Row {
	data_set:DrawableData
}
impl Row {

	/// Create a new Row.
	pub fn new(children:Vec<&dyn Drawable>) -> Row {

		// Create a new struct.
		let positioned_children:Vec<Positioned> = children.iter().map(|child| Positioned::new(0, 0, vec![*child])).collect::<Vec<Positioned>>();
		let mut row:Row = Row {
			data_set: DrawableData::new::<usize>(vec![], positioned_children.iter().map(|child| child as &dyn Drawable).collect::<Vec<&dyn Drawable>>())
		};

		// Trigger an initial draw to update the position.
		row.draw();

		// Return the struct.
		row
	}
}
impl Drawable for Row {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("row")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Row {
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
				last_child_pos[0] + last_child_pos[2],
				children_positions.iter().map(|position| position[1] + position[3]).max().unwrap()
			]
		}
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		let mut x:usize = 0;
		let children:&mut Vec<Box<dyn Drawable>> = self.data_set_mut().children_mut();
		for child in children {
			child.data_set_mut().set_setting_value::<usize>("x", x);
			let child_pos:[usize; 4] = child.position();
			x += child_pos[2];
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