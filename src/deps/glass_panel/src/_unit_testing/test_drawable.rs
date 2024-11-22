#[cfg(test)]
mod test {
	use crate::{ Drawable, DrawBuffer, elements::{ Rectangle, Png } };
	use crate::_unit_testing::support::TestImage;

	/* PNG EXPORT TESTS */

	#[test]
	fn png_export_creates_file() {
		let test_img:TestImage = TestImage::get();
		Rectangle::new(1, 1, 0xFF2468AC, vec![]).draw_png(&test_img.path()).unwrap();
		assert!(test_img.exists());
	}

	#[test]
	#[should_panic(expected = "Zero width not allowed")]
	fn png_export_empty_buffer() {
		let test_img:TestImage = TestImage::get();
		Rectangle::new(0, 0, 0, vec![]).draw_png(&test_img.path()).unwrap();
		assert!(test_img.exists());
	}

	#[test]
	fn png_export_matches_buffer() {

		// Color and size settings.
		let size:[usize; 2] = [6, 3];
		let color:u32 = 0xFF2468AC;

		// Create and read image.
		let test_img:TestImage = TestImage::get();
		let mut test_rect:Rectangle = Rectangle::new(size[0], size[1], color, vec![]);
		test_rect.draw_png(&test_img.path()).expect("Could not create test png file");
		let read_image:DrawBuffer = Png::new(&test_img.path(), vec![]).expect("Could not open test image").draw();
		
		// Compare image.
		assert_eq!(read_image.width, size[0]);
		assert_eq!(read_image.height, size[1]);
		assert_eq!(read_image.data.len(), size[0] * size[1]);
		assert_eq!(read_image.data, test_rect.draw().data);
	}
}