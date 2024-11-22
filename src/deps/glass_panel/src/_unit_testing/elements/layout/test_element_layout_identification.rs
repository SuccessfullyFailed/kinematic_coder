#[cfg(test)]
mod test {
	use crate::{ Drawable, elements::{ Id, Class } };
    
	#[test]
	fn element_identification_id() {
		assert_eq!(Id::new("Test", vec![]).name(), "#Test");
		assert_eq!(Id::new("Validation", vec![]).name(), "#Validation");
	}

	#[test]
	fn element_identification_class() {
		assert_eq!(Class::new("Test", vec![]).name(), ".Test");
		assert_eq!(Class::new("Validation", vec![]).name(), ".Validation");
	}
}