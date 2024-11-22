#[cfg(test)]
mod test {
	use crate::{ Drawable, DrawBuffer, elements::{ Rectangle, Id, Positioned } };
    
	#[test]
	fn element_positioned_has_correct_bounds() {
		let c:u32 = 0xFF000000;

		// Check variable size.
		let mut positioned:Positioned = Positioned::new(30, 20, vec![&Rectangle::new(100, 100, c, vec![])]);
		positioned.update(&[0, 0]);
		assert_eq!(positioned.position(), [30, 20, 100, 100]);

		// Check drawn size.
		let buffer:DrawBuffer = Id::new("", vec![&positioned]).draw();
		assert_eq!([buffer.width, buffer.height], [130, 120]);
	}

	#[test]
	fn element_positioned_can_move() {
		let c:u32 = 0xFF000000;

		// Initial struct.
		let mut positioned:Positioned = Positioned::new(30, 30, vec![&Rectangle::new(100, 100, c, vec![])]);
		positioned.update(&[0, 0]);
		assert_eq!(positioned.position(), [30, 30, 100, 100]);

		// Move position.
		positioned.data_set_mut().set_setting_value::<usize>("y", 500);
		positioned.update(&[0, 0]);
		assert_eq!(positioned.position(), [30, 500, 100, 100]);
	}

	#[test]
	fn element_positioned_works_recursively() {
		let c:u32 = 0xFF000000;

		// Initial struct.
		let mut positioned:Positioned = Positioned::new(30, 20, vec![
			&Rectangle::new(100, 100, c, vec![]),
			&Positioned::new(130, 30, vec![
				&Id::new("inner", vec![
					&Rectangle::new(100, 100, c, vec![])
				])
			])
		]);
		positioned.update(&[0, 0]);

		// Check outer and inner position.
		assert_eq!(positioned.position(), [30, 20, 230, 130]);
		assert_eq!(positioned.child_by_name("#inner").unwrap().position(), [0, 0, 100, 100]);
		assert_eq!(positioned.child_by_name("#inner").unwrap().data_set().absolute_position(), &[160, 50, 100, 100]);
	}
}