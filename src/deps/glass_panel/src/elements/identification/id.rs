use crate::{ Drawable, DrawableData };

pub struct Id {
	id:String,
	data_set:DrawableData
}
impl Id {

	/// Create a new Id.
	pub fn new(id:&str, children:Vec<&dyn Drawable>) -> Id {
		Id {
			id: String::from(id),
			data_set: DrawableData::new::<u32>(vec![], children)
		}
	}
}
impl Drawable for Id {

	/// Get the name of the components.
	fn name(&self) -> String {
		format!("#{}", self.id)
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Id {
			id: String::from(&self.id),
			data_set: self.data_set.clone()
		})
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