#[cfg(test)]
mod test {
	use crate::{ Drawable, DrawBuffer, elements::{ SubSet, Row, Col, Rectangle } };
    
	#[test]
	fn subset_shows_correct_image() {
		let colors:[[u32; 2]; 2] = [[0xFFF00000, 0xFF0F0000], [0xFF00F000, 0xFF000F00]];
		let field_size:usize = 6;

		// Create subset.
		let mut subset:SubSet = SubSet::new(0, 0, field_size * 2, field_size * 2, vec![
			&Col::new(vec![
				&Row::new(vec![
					&Rectangle::new(field_size, field_size, colors[0][0], vec![]),
					&Rectangle::new(field_size, field_size, colors[0][1], vec![])
				]),
				&Row::new(vec![
					&Rectangle::new(field_size, field_size, colors[1][0], vec![]),
					&Rectangle::new(field_size, field_size, colors[1][1], vec![])
				])
			])
		]);
		subset.update(&[0, 0]);
		assert_eq!(subset.position(), [0, 0, field_size * 2, field_size * 2]);

		// Check all colors.
		let buffer:DrawBuffer = subset.draw();
		for field_y in 0..2 {
			for field_x in 0..2 {
				for x in field_size * field_x..field_size * (field_x + 1) {
					for y in field_size * field_y..field_size * (field_y + 1) {
						assert_eq!(buffer.data[(y * buffer.width) + x], colors[field_y][field_x]);
					}
				}
			}
		}

		// Move subset.
		subset.data_set_mut().set_setting_value::<usize>("x", field_size / 2);
		subset.data_set_mut().set_setting_value::<usize>("y", field_size / 2);
		subset.data_set_mut().set_setting_value::<usize>("w", field_size);
		subset.data_set_mut().set_setting_value::<usize>("h", field_size);
		subset.update(&[0, 0]);
		
		// Check all colors.
		let buffer:DrawBuffer = subset.draw();
		for x in 0..field_size {
			for y in 0..field_size {
				assert_eq!(buffer.data[(y * buffer.width) + x], colors[if y >= field_size / 2 { 1 } else { 0 }][if x >= field_size / 2 { 1 } else { 0 }]);
			}
		}
	}
}