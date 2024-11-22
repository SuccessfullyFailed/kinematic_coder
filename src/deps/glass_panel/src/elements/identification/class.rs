use crate::{ Drawable, DrawableData };

pub struct Class {
	class_name:String,
	data_set:DrawableData
}
impl Class {

	/// Create a new Class.
	pub fn new(class_name:&str, children:Vec<&dyn Drawable>) -> Class {
		Class {
			class_name: String::from(class_name),
			data_set: DrawableData::new::<u32>(vec![], children)
		}
	}
}
impl Drawable for Class {

	/// Get the name of the components.
	fn name(&self) -> String {
		format!(".{}", self.class_name)
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(Class {
			class_name: String::from(&self.class_name),
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