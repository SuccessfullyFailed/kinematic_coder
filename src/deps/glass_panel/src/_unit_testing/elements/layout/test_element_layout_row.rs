#[cfg(test)]
mod test {
	use crate::{ Drawable, elements::{ Row, Positioned, Rectangle, Id } };
    
	#[test]
	fn row_has_correct_size() {
		let c:u32 = 0xFF000000;

		// Simple row.
		let row:Row = Row::new(vec![
			&Rectangle::new(30, 30, c, vec![]),
			&Rectangle::new(40, 20, c, vec![]),
		]);
		assert_eq!(row.position(), [0, 0, 70, 30]);

		// Offsetted elements.
		let row:Row = Row::new(vec![
			&Positioned::new(10, 10, vec![
				&Rectangle::new(30, 30, c, vec![]),
			]),
			&Rectangle::new(40, 20, c, vec![]),
		]);
		assert_eq!(row.position(), [0, 0, 80, 40]);
	}
    
	#[test]
	fn row_allows_growth() {
		let c:u32 = 0xFF000000;

		// Create row.
		let mut row:Row = Row::new(vec![
			&Rectangle::new(30, 30, c, vec![]),
			&Id::new("singular", vec![&Rectangle::new(40, 20, c, vec![])]),
		]);
		assert_eq!(row.position(), [0, 0, 70, 30]);

		// Create new child.
		row.add_child(&Rectangle::new(60, 60, c, vec![]));
		row.update(&[0, 0]);
		row.draw();
		assert_eq!(row.position(), [0, 0, 130, 60]);

		// Grow existing child.
		row.child_by_name_mut("#singular rectangle").unwrap().data_set_mut().set_setting_value::<usize>("width", 120);
		row.update(&[0, 0]);
		row.draw();
		assert_eq!(row.position(), [0, 0, 210, 60]);
	}
}