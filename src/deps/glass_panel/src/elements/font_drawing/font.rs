use std::error::Error;

use crate::DrawBuffer;

/// A struct to keep all data required to be able to write text to an imagebuffer based on a font file.
#[derive(Clone)]
pub struct Font {
	font:rusttype::Font<'static>,
	padding:f32
}
impl Font {

	/// Create a new Font instance.
	pub fn new(file_path:&str, padding:f32) -> Result<Font, Box<dyn Error>> {
		use std::fs::read;
		
		Ok(Font {
			font: rusttype::Font::try_from_vec(read(file_path)?).unwrap(),
			padding
		})
	}

	/// Get the bounding box a buffer of a text would have.
	pub fn bounding_rect_of(&self, text:&str, line_height:usize) -> [usize; 4] {
		use rusttype::Scale;
		
		// Prepare widths.
		let line_height:f32 = line_height as f32;
		let scale:Scale = Scale::uniform(line_height);
		let space_width:f32 = self.font.glyph(' ').scaled(scale).h_metrics().advance_width;
		let padding:f32 = line_height * self.padding;

		// Calculate size of each line of text.
		let text:String = text.replace('\r', "\n");
		let lines:Vec<&str> = text.split('\n').collect::<Vec<&str>>();
		let line_widths:Vec<f32> = lines.iter().map(|text| 
			text.chars().map(|character|
				match character {
					' ' => space_width,
					'\t' => (space_width + padding) * 8.0,
					_ => match self.font.glyph(character).scaled(scale).positioned(rusttype::point(0.0, 0.0)).pixel_bounding_box() {
						Some(bounding_box) => bounding_box.max.x as f32,
						None => 0.0
					}
				} + padding
			).sum()
		).collect::<Vec<f32>>();

		// Return size.
		[0, 0, line_widths.iter().map(|width| *width as usize).max().unwrap(), lines.len() * (line_height + 2.0 * padding) as usize]
	}

	/// Draw a character.
	pub fn draw_text(&self, text:&str, line_height:usize, color:u32) -> DrawBuffer {
		use rusttype::{ Scale, Glyph, ScaledGlyph, PositionedGlyph };

		// Calculate settings.
		let line_height:f32 = line_height as f32;
		let scale:Scale = Scale::uniform(line_height);
		let space_width:f32 = self.font.glyph(' ').scaled(scale).h_metrics().advance_width;
		let padding:f32 = line_height * self.padding;
		let [_, _, buffer_width, buffer_height] = self.bounding_rect_of(text, line_height as usize);

		// Create buffer.
		let mut buffer:DrawBuffer = DrawBuffer::new(vec![0x00000000; buffer_width * buffer_height], buffer_width, buffer_height);
		let mut caret:[f32; 2] = [0.0, 0.0];
		for character in text.chars() {

			// Space.
			if character == ' ' {
				caret[0] += space_width + padding;
				continue;
			}

			// Tab.
			if character == '\t' {
				caret[0] += (space_width + padding) * 8.0;
				continue;
			}

			// Line-breaks.
			if character == '\n' || character == '\r' {
				caret[0] = 0.0;
				caret[1] += line_height + padding;
				continue;
			}

			// Other.
			let glyph:Glyph<'_> = self.font.glyph(character);
			let scaled_glyph:ScaledGlyph<'_> = glyph.scaled(scale);
			let positioned_glyph:PositionedGlyph<'_> = scaled_glyph.clone().positioned(rusttype::point(caret[0], caret[1]));
			if let Some(bounding_box) = positioned_glyph.pixel_bounding_box() {
				positioned_glyph.draw(|x, y, v| {
					let x:i32 = x as i32 + bounding_box.min.x;
					let y:i32 = y as i32 + line_height as i32 + bounding_box.min.y;
					let color_alpha:u32 = ((v * 255.0) as u32) << 24 | (color & 0x00FFFFFF);
					if x < buffer_width as i32 && y < buffer_height as i32 {
						buffer.data[(y as usize * buffer.width) + x as usize] = color_alpha;
					}
				});
				caret[0] = bounding_box.max.x as f32 + padding;
			}
		}

		buffer
	}
}