

#[cfg(test)]
mod test {
	use crate::{ elements::{ Id, Positioned, Rectangle }, Drawable, DrawableData, ListenerCallback, ListenerType };
	use std::rc::Rc;

	static INNER_ELEMENT_SIZE:[usize; 2] = [3, 3];
	static CLICK_LOCATION:[usize; 2] = [50, 30];
	static LISTENER_NAME:&str = "UnitTestClickListener";
	static LISTENER_ELEMENT_PATH:&str = "#ClickSource rectangle";



	/* HELPER METHODS */

	/// Create an element with a listener.
	fn get_listener_debug_element(click_handler:ListenerCallback) -> Rectangle {

		// Create an element for the listener.
		let mut element:Rectangle  = Rectangle::new(100, 100, 0xFFFFFFFF, vec![
			&Positioned::new(CLICK_LOCATION[0], CLICK_LOCATION[1], vec![
				&Id::new("ClickSource", vec![&Rectangle::new(INNER_ELEMENT_SIZE[0], INNER_ELEMENT_SIZE[1], 0xFF000000, vec![])])
			])
		]);

		// Create the listener.
		element.child_by_name_mut(LISTENER_ELEMENT_PATH).unwrap().add_listener(LISTENER_NAME, ListenerType::LeftDown, click_handler);

		// Update the cache of all elements.
		element.update(&[0; 2]);

		element
	}



	/* TESTING METHODS */

	#[test]
	fn listener_is_created() {
		let element:Rectangle = get_listener_debug_element(Rc::new(|_,_,_,_|{}));
		element.has_listeners();
	}

	static mut HANDLER_ADAPTOR:u8 = 0;
	#[test]
	fn listener_triggers_executor() {
		unsafe {
			HANDLER_ADAPTOR = 0;
			let mut element:Rectangle = get_listener_debug_element(Rc::new(|_,_,_,_| {
				HANDLER_ADAPTOR += 1;
			}));

			// Validate modification of handler adaptor.
			assert_eq!(HANDLER_ADAPTOR, 0);
			element.update_listeners(&[0, 0], &[false; 2], &[false; 2]);
			assert_eq!(HANDLER_ADAPTOR, 0);
			element.update_listeners(&[0, 0], &[true; 2], &[true; 2]);
			assert_eq!(HANDLER_ADAPTOR, 0);
			element.update_listeners(&[CLICK_LOCATION[0], CLICK_LOCATION[1]], &[false; 2], &[false; 2]);
			assert_eq!(HANDLER_ADAPTOR, 0);
			element.update_listeners(&[CLICK_LOCATION[0], CLICK_LOCATION[1]], &[true; 2], &[false; 2]);
			assert_eq!(HANDLER_ADAPTOR, 0);
			element.update_listeners(&[CLICK_LOCATION[0], CLICK_LOCATION[1]], &[false; 2], &[true; 2]);
			assert_eq!(HANDLER_ADAPTOR, 0);
			element.update_listeners(&[CLICK_LOCATION[0], CLICK_LOCATION[1]], &[true; 2], &[true; 2]);
			assert_eq!(HANDLER_ADAPTOR, 1);
			element.update_listeners(&[CLICK_LOCATION[0], CLICK_LOCATION[1]], &[true; 2], &[true; 2]);
			assert_eq!(HANDLER_ADAPTOR, 2);
		}
	}

	#[test]
	fn listener_each_type_triggers_executor() {
		unsafe {
			let location_offset:usize = 80;

			HANDLER_ADAPTOR = 0;

			// Create element and listeners.
			let mut element:Rectangle  = Rectangle::new(100, 100, 0xFFFFFFFF, vec![]);
			element.add_listener("TEST_LISTENER", ListenerType::Hover, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR = 1; }));
			element.add_listener("TEST_LISTENER", ListenerType::LeftDown, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR = 2; }));
			element.add_listener("TEST_LISTENER", ListenerType::LeftUp, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR = 3; }));
			element.add_listener("TEST_LISTENER", ListenerType::RightDown, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR = 4; }));
			element.add_listener("TEST_LISTENER", ListenerType::RightUp, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR = 5; }));
			element.update(&[location_offset; 2]);

			// None.
			element.update_listeners(&[0; 2], &[false, false], &[false, false]);
			assert_eq!(HANDLER_ADAPTOR, 0);

			// Hover.
			element.update_listeners(&[location_offset; 2], &[false, false], &[false, false]);
			assert_eq!(HANDLER_ADAPTOR, 1);

			// LeftDown.
			element.update_listeners(&[location_offset; 2], &[true, false], &[true, false]);
			assert_eq!(HANDLER_ADAPTOR, 2);

			// LeftUp.
			element.update_listeners(&[location_offset; 2], &[false, false], &[true, false]);
			assert_eq!(HANDLER_ADAPTOR, 3);

			// RightDown.
			element.update_listeners(&[location_offset; 2], &[false, true], &[false, true]);
			assert_eq!(HANDLER_ADAPTOR, 4);

			// RightUp.
			element.update_listeners(&[location_offset; 2], &[false, false], &[false, true]);
			assert_eq!(HANDLER_ADAPTOR, 5);


			// Drag handlers could override the code set by Down handlers, so do them separately.
			element.remove_listener("TEST_LISTENER");
			element.add_listener("TEST_LISTENER", ListenerType::DragLeft, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR += 1; }));
			element.add_listener("TEST_LISTENER", ListenerType::DragRight, 	Rc::new(|_,_,_,_|{ HANDLER_ADAPTOR += 2; }));
			element.update(&[location_offset; 2]);

			// DragLeft.
			HANDLER_ADAPTOR = 0;
			element.update_listeners(&[location_offset; 2], &[true, false], &[true, false]);
			element.update_listeners(&[location_offset; 2], &[true, false], &[false, false]);
			element.update_listeners(&[location_offset; 2], &[false, false], &[true, false]);
			assert_eq!(HANDLER_ADAPTOR, 1);

			// DragRight.
			HANDLER_ADAPTOR = 0;
			element.update_listeners(&[location_offset; 2], &[false, true], &[false, true]);
			element.update_listeners(&[location_offset; 2], &[false, true], &[false, false]);
			element.update_listeners(&[location_offset; 2], &[false, false], &[false, true]);
			assert_eq!(HANDLER_ADAPTOR, 2);
		}
	}

	#[test]
	fn listener_correct_arguments() {
		let mut element:Rectangle = get_listener_debug_element(Rc::new(|rel_pos:[isize; 2], abs_pos:[isize; 2], _:&str, settings:&mut DrawableData| {
			
			// Validate click positions.
			assert_eq!(rel_pos[0], 0);
			assert_eq!(rel_pos[1], 0);
			assert_eq!(abs_pos[0], CLICK_LOCATION[0] as isize);
			assert_eq!(abs_pos[1], CLICK_LOCATION[1] as isize);

			// Validate settings object.
			let modified_size:usize = 30;
			assert_eq!(settings.get_setting_value::<usize>("width").unwrap(), INNER_ELEMENT_SIZE[0]);
			settings.set_setting_value::<usize>("width", modified_size);
			assert_eq!(settings.get_setting_value::<usize>("width").unwrap(), modified_size);
		}));
		element.update_listeners(&[CLICK_LOCATION[0], CLICK_LOCATION[1]], &[true; 2], &[true; 2]);
	}

	#[test]
	fn listener_removed() {
		let mut element:Rectangle = get_listener_debug_element(Rc::new(|_, _, _, _| {}));
		assert!(element.has_listeners());
		element.child_by_name_mut(LISTENER_ELEMENT_PATH).unwrap().remove_listener("NonexistentListener");
		assert!(element.has_listeners());
		element.child_by_name_mut(LISTENER_ELEMENT_PATH).unwrap().remove_listener(LISTENER_NAME);
		assert!(!element.has_listeners());
	}
}