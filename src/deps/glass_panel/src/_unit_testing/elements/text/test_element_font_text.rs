#[cfg(test)]
mod test {
	use crate::{ Drawable, DrawBuffer, elements::{ Font, Text } };
	use crate::_unit_testing::support::TEST_FONT_LOCATION;

	#[test]
	fn element_text_load_font() {
		let _:Font = Font::new(TEST_FONT_LOCATION, 0.1).expect(&format!("Could not load font from {TEST_FONT_LOCATION}"));
	}

	#[test]
	fn element_text_size() {

		// Settings.
		let font:Font = Font::new(TEST_FONT_LOCATION, 0.0).expect(&format!("Could not load font from {TEST_FONT_LOCATION}"));
		let line_height:usize = 16;
		let text_lengths:[usize; 3] = [10, 20, 40];

		// Create texts.
		let texts:Vec<DrawBuffer> = text_lengths.iter().map(|length| Text::new(&font, &"abcde_f".repeat(*length), line_height, 0).draw()).collect::<Vec<DrawBuffer>>();

		// Compare sizes.
		assert_eq!(texts[0].width, texts[1].width / 2);
		assert_eq!(texts[1].width, texts[2].width / 2);
		assert_eq!(texts[0].height, line_height);
		assert_eq!(texts[1].height, line_height);
		assert_eq!(texts[2].height, line_height);
	}

	#[test]
	fn element_text_line_breaks() {

		// Settings.
		let font:Font = Font::new(TEST_FONT_LOCATION, 0.0).expect(&format!("Could not load font from {TEST_FONT_LOCATION}"));
		let line_height:usize = 16;

		// Create texts.
		let text_without_line_break:DrawBuffer = Text::new(&font, &"AAA", line_height, 0).draw();
		let text_with_line_break:DrawBuffer = Text::new(&font, &"AAA\nBBB", line_height, 0).draw();
		let text_with_two_line_breaks:DrawBuffer = Text::new(&font, &"AAA\n\nCCC", line_height, 0).draw();

		// Compare sizes.
		assert_eq!(text_without_line_break.height, text_with_line_break.height / 2);
		assert_eq!(text_with_line_break.height / 2, text_with_two_line_breaks.height / 3);
	}
}