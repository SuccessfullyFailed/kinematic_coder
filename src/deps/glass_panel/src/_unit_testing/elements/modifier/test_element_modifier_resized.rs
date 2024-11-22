#[cfg(test)]
mod test {
	use crate::{ Drawable, DrawBuffer, elements::{ Resized, Row, Col, Rectangle } };
    
	#[test]
	fn resize_creates_correct_size() {
		let c:u32 = 0xFF000000;

		// Shrinking size.
		let mut resized_image:Resized = Resized::new(30, 20, vec![&Rectangle::new(100, 100, c, vec![])]);
		resized_image.update(&[0, 0]);
		assert_eq!(resized_image.position(), [0, 0, 30, 20]);

		// Growing size.
		let mut resized_image:Resized = Resized::new(300, 200, vec![&Rectangle::new(100, 100, c, vec![])]);
		resized_image.update(&[0, 0]);
		assert_eq!(resized_image.position(), [0, 0, 300, 200]);
	}
    
	#[test]
	fn resize_creates_correct_image_on_shrink() {
		let colors:[[u32; 2]; 2] = [[0xFFF00000, 0xFF0F0000], [0xFF00F000, 0xFF000F00]];
		let field_size:usize = 6;

		// Create subset.
		let mut resized_image:Resized = Resized::new(field_size, field_size, vec![
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
		resized_image.update(&[0, 0]);

		// Check correct colors.
		let buffer:DrawBuffer = resized_image.draw();
		for y in 0..field_size {
			for x in 0..field_size {
				assert_eq!(buffer.data[(y * buffer.width) + x], colors[if y >= field_size / 2 { 1 } else { 0 }][if x >= field_size / 2 { 1 } else { 0 }]);
			}
		}
	}
    
	#[test]
	fn resize_creates_correct_image_on_grow() {
		let colors:[[u32; 2]; 2] = [[0xFFF00000, 0xFF0F0000], [0xFF00F000, 0xFF000F00]];
		let field_size:usize = 6;

		// Create subset.
		let mut resized_image:Resized = Resized::new(field_size * 4, field_size * 4, vec![
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
		resized_image.update(&[0, 0]);

		// Check correct colors.
		let buffer:DrawBuffer = resized_image.draw();
		for y in 0..field_size * 4 {
			for x in 0..field_size * 4 {
				assert_eq!(buffer.data[(y * buffer.width) + x], colors[if y >= field_size * 2 { 1 } else { 0 }][if x >= field_size * 2 { 1 } else { 0 }]);
			}
		}
	}
}