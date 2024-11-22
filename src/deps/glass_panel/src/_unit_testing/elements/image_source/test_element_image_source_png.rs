#[cfg(test)]
mod test {
	use crate::{ Drawable, DrawBuffer, elements::Png };
	use crate::_unit_testing::support::TEST_TEXTURE_LOCATION;

	#[test]
	fn element_png_reads_png() {
		
		// Read the testing file.
		let mut image:Png = Png::new(TEST_TEXTURE_LOCATION, vec![]).unwrap_or_else(|_| panic!("Could not create Png element from '{TEST_TEXTURE_LOCATION}'"));
		let buffer:DrawBuffer = image.draw();

		// Count how much each color is used.
		let mut color_usage:Vec<(u32, usize)> = Vec::new();
		for color in &buffer.data {
			match color_usage.iter().position(|color_usage_set| &color_usage_set.0 == color) {
				Some(index) => color_usage[index].1 += 1,
				None => color_usage.push((*color, 1))
			}
		}

		// For largely expected colors, check a minimum count. For 'blended' colors, check a maximum count.
		let most_common_colors_min_counts:Vec<(u32, usize)> = vec![(0xffed1c24, 15000), (0xffa349a4, 15000), (0xff22b14c, 15000), (0xff00a2e8, 15000), (0xffffffff, 1600)];
		let uncommon_colors_max_count:usize = 100;
		for color_validation_set in &color_usage {
			let usage_index:usize = color_usage.iter().position(|color_usage_set| &color_usage_set.0 == &color_validation_set.0).unwrap(); // impossible to run into pixels not counted earlier.
			let expectation_index:Option<usize> = most_common_colors_min_counts.iter().position(|expectation_set| &expectation_set.0 == &color_validation_set.0);
			match expectation_index {
				Some(validation_index) => assert!(color_usage[usage_index].1 > most_common_colors_min_counts[validation_index].1),
				None => assert!(color_usage[usage_index].1 < uncommon_colors_max_count)
			}
		}
	}

	#[test]
	fn element_png_handles_nonexistent() {
		match Png::new("nonexistent.png", vec![]) {
			Ok(_) => panic!("Png accepted nonexistent file."),
			Err(_) => {}
		}
	}
}