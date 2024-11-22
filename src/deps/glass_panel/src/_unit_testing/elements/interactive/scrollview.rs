#[cfg(test)]
mod test {
	use crate::{ Drawable, elements::{ ScrollView, Col, Rectangle } };

	/// Create a simple scrollview to run tests with.
	fn create_test_scrollview() -> ScrollView {
		let mut scroll_view:ScrollView = ScrollView::new(100, 80, 0xFF222222, 0xFF888888, vec![
			&Col::new(vec![
				&Rectangle::new(80, 40, 0xFFFF0000, vec![]),
				&Rectangle::new(80, 40, 0xFF00FF00, vec![]),
				&Rectangle::new(80, 40, 0xFF0000FF, vec![]),
				&Rectangle::new(80, 40, 0xFFFFFFFF, vec![]),
			])
		]);
		scroll_view.update(&[0, 0]);
		scroll_view
	}
    
	#[test]
	fn scrollview_can_scroll() {

		// Create a scrollview.
		let mut scroll_view:ScrollView = create_test_scrollview();
		
		// Keep checking topleft colors while scrolling.
		let expected_colors:Vec<([usize; 2], u32)> = vec![([0, 19], 0xFFFF0000), ([20, 39], 0xFF00FF00), ([40, 59], 0xFF0000FF)];
		for color_set in &expected_colors {
			for scroll_amount in color_set.0[0]..color_set.0[1] {
				scroll_view.scroll_to(scroll_amount);
				assert_eq!(scroll_view.draw().data[0], color_set.1);
			}
		}
	}

	#[test]
	fn scrollview_keeps_bounds() {
		let bar_path:String = String::from(".scrollview_bar .scrollview_bar_progress positioned");

		// Create a scrollview.
		let mut scroll_view:ScrollView = create_test_scrollview();

		// Bottom bounds.
		scroll_view.scroll_to(40 as usize);
		scroll_view.draw();
		assert_eq!(scroll_view.child_by_name(&bar_path).unwrap().data_set().get_setting_value::<usize>("y").unwrap(), 40);
		scroll_view.scroll_to(41 as usize);
		assert_eq!(scroll_view.child_by_name(&bar_path).unwrap().data_set().get_setting_value::<usize>("y").unwrap(), 40);
		scroll_view.scroll_to(100 as usize);
		assert_eq!(scroll_view.child_by_name(&bar_path).unwrap().data_set().get_setting_value::<usize>("y").unwrap(), 40);
	}
}