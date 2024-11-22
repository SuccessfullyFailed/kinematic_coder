#[cfg(test)]
mod test {
	use crate::{ Drawable, elements::{ Col, Positioned, Rectangle, Id } };
    
	#[test]
	fn col_has_correct_size() {
		let c:u32 = 0xFF000000;

		// Simple col.
		let col:Col = Col::new(vec![
			&Rectangle::new(30, 30, c, vec![]),
			&Rectangle::new(40, 20, c, vec![]),
		]);
		assert_eq!(col.position(), [0, 0, 40, 50]);

		// Offsetted elements.
		let col:Col = Col::new(vec![
			&Positioned::new(10, 10, vec![
				&Rectangle::new(30, 30, c, vec![]),
			]),
			&Rectangle::new(40, 20, c, vec![]),
		]);
		assert_eq!(col.position(), [0, 0, 40, 60]);
	}
    
	#[test]
	fn col_allows_growth() {
		let c:u32 = 0xFF000000;

		// Create col.
		let mut col:Col = Col::new(vec![
			&Rectangle::new(30, 30, c, vec![]),
			&Id::new("singular", vec![&Rectangle::new(40, 20, c, vec![])]),
		]);
		assert_eq!(col.position(), [0, 0, 40, 50]);

		// Create new child.
		col.add_child(&Rectangle::new(60, 60, c, vec![]));
		col.update(&[0, 0]);
		col.draw();
		assert_eq!(col.position(), [0, 0, 60, 110]);

		// Grow existing child.
		col.child_by_name_mut("#singular rectangle").unwrap().data_set_mut().set_setting_value::<usize>("height", 120);
		col.update(&[0, 0]);
		col.draw();
		assert_eq!(col.position(), [0, 0, 60, 210]);
	}
}