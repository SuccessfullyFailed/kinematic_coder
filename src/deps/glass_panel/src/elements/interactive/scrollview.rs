use crate::{ elements::{ Class, Positioned, Rectangle, Row, SubSet }, DrawBuffer, Drawable, DrawableData, DrawableDataSettingDataType, ListenerType };
use std::rc::Rc;

pub struct ScrollView {
	data_set:DrawableData
}
impl ScrollView {

	/// Create a new ScrollView.
	pub fn new(width:usize, height:usize, outer_bar_color:u32, inner_bar_color:u32, children:Vec<&dyn Drawable>) -> ScrollView {
		
		let bar_size:usize = 10;

		// Create element.
		let mut scroll_view:ScrollView = ScrollView {
			data_set: DrawableData::new::<Vec<u8>>(
				vec![("width", width.to_bytes()), ("height", height.to_bytes()), ("outer_bar_color", outer_bar_color.to_bytes()), ("inner_bar_color", inner_bar_color.to_bytes()), ("scroll", 0_usize.to_bytes())],
				vec![
					&Row::new(vec![
						&Rectangle::new(width - bar_size, height, 0x00000000, vec![
							&SubSet::new(0, 0, width - bar_size, height, vec![
								&Class::new("scrollview_contents", children)
							])
						]),
						&Class::new("scrollview_bar", vec![
							&Rectangle::new(bar_size, height, outer_bar_color, vec![
								&Class::new("scrollview_bar_progress", vec![
									&Positioned::new(0, 0, vec![
										&Rectangle::new(bar_size, height, inner_bar_color, vec![])
									])
								])
							])
						])
					])
				]
			)
		};

		// Add listener.
		let inner_bar = scroll_view.child_by_name_mut(".scrollview_bar .scrollview_bar_progress positioned").unwrap();
		inner_bar.add_listener("ScrollDrag", ListenerType::DragLeft, Rc::new(|drag_offset:[isize; 2], _, listener_name:&str, element_data:&mut DrawableData| {
			if listener_name != "ScrollDrag" { return; }
			let calculated:isize = element_data.get_setting_value_or::<usize>("y", 0) as isize + drag_offset[1];
			let min:isize = 0;
			let max:isize = element_data.get_setting_value_or::<usize>("max_y", usize::MAX) as isize;
			element_data.set_setting_value::<usize>("y", *[*[calculated, min].iter().max().unwrap(), max].iter().min().unwrap() as usize);
		}));

		scroll_view
	}

	/// Scroll to the required amount.
	pub fn scroll_to(&mut self, target:usize) {
		if let Some(bar) = self.child_by_name_mut(".scrollview_bar .scrollview_bar_progress positioned") {
			let max:usize = bar.data_set().get_setting_value_or::<usize>("max_y", usize::MAX);
			bar.data_set_mut().set_setting_value::<usize>("y", target.min(max));
		}
	}
}
impl Drawable for ScrollView {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("scrollview")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(ScrollView { data_set: self.data_set.clone() })
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[
			0,
			0,
			self.data_set().get_setting_value_or::<usize>("width", 0),
			self.data_set().get_setting_value_or::<usize>("height", 0)
		]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {

		// Figure out scrollbar size and position.
		let available_height:usize = self.data_set().get_setting_value_or::<usize>("height", 0);
		let content_height:usize = match self.child_by_name(".scrollview_contents") { Some(child) => child.position()[3], None => 0 };
		let scroll_indication_scale:f32 = 1.0 / content_height as f32 * available_height as f32;

		// Set scrollbar position.
		let noted_scroll_progress:usize = self.data_set().get_setting_value_or::<usize>("scroll", 0);
		if let Some(positioned_bar) = self.child_by_name_mut(".scrollview_bar .scrollview_bar_progress positioned") {
			let new_scroll_progress:usize = positioned_bar.data_set().get_setting_value_or::<usize>("y", 0);

			// Calculate amount of scroll for content based on the amount of scroll of the scrollbar.
			let available_scroll_progress:usize = positioned_bar.data_set().get_setting_value_or::<usize>("max_y", 0);
			let normalized_scroll_progress:f32 = 1.0 / available_scroll_progress as f32 * new_scroll_progress as f32;
			let content_scroll:usize = *[((content_height as f32 - available_height as f32) * normalized_scroll_progress) as isize, 0].iter().max().unwrap() as usize;

			// Update scrollbar position.
			if noted_scroll_progress != new_scroll_progress {
				self.data_set_mut().set_setting_value::<usize>("scroll", new_scroll_progress);

				// Update scroll content.
				if let Some(contents) = self.child_by_name_mut("subset") {
					contents.data_set_mut().set_setting_value::<usize>("y", content_scroll);
				}
			}
		}
		
		// Set scrollbar size.
		if let Some(bar) = self.child_by_name_mut(".scrollview_bar .scrollview_bar_progress positioned rectangle") {
			let scrollbar_height:usize = (available_height as f32 * if scroll_indication_scale < 1.0 { scroll_indication_scale } else { 1.0 }) as usize;
			bar.data_set_mut().set_setting_value::<usize>("height", scrollbar_height);
			if let Some(positioned_bar) = self.child_by_name_mut(".scrollview_bar .scrollview_bar_progress positioned") {
				positioned_bar.data_set_mut().set_setting_value::<usize>("max_y", available_height - scrollbar_height);
			}
		}

		self.draw_children()
	}

	

	/* DATA SET METHODS */

	/// Get a reference to the instances Data object.
	fn data_set(&self) -> &DrawableData {
		&self.data_set
	}

	/// Get a mutable reference to the instances Data object.
	fn data_set_mut(&mut self) -> &mut DrawableData {
		&mut self.data_set
	}
}