use crate::{ DrawBuffer, DrawableData, ListenerCallback, ListenerType };
use std::error::Error;


static CHILD_PATH_SPLITTER:char = ' ';

pub trait Drawable {

	/// Get the name of the components.
	fn name(&self) -> String;

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable>;

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		let child_bounds:[usize; 4] = self.children_bounding_box();
		[0, 0, child_bounds[0] + child_bounds[2], child_bounds[1] + child_bounds[3]]
	}

	/// Get a bounding box that fits all children.
	fn children_bounding_box(&self) -> [usize; 4] {
		let child_positions:Vec<[usize; 4]> = self.data_set().children().iter().map(|child| child.position()).collect::<Vec<[usize; 4]>>();
		let min_x:usize = child_positions.iter().map(|position| position[0]).min().unwrap_or(0);
		let min_y:usize = child_positions.iter().map(|position| position[1]).min().unwrap_or(0);
		let max_x:usize = child_positions.iter().map(|position| position[0] + position[2]).max().unwrap_or(min_x) - min_x;
		let max_y:usize = child_positions.iter().map(|position| position[1] + position[3]).max().unwrap_or(min_y) - min_y;
		[min_x, min_y, max_x, max_y]
	}



	/* DATA SET METHODS */

	/// Get a reference to the instances Data object.
	fn data_set(&self) -> &DrawableData;

	/// Get a mutable reference to the instances Data object.
	fn data_set_mut(&mut self) -> &mut DrawableData;

	/// Update the full model. Returns a bool indicating if a real update has taken place. This can be useful for deciding wether or not to redraw the element.
	fn update(&mut self, parent_position:&[usize; 2]) -> bool {
		if self.data_set().requires_update() {
			let self_position:[usize; 4] = self.position();
			self.data_set_mut().update(&[parent_position[0] + self_position[0], parent_position[1] + self_position[1], self_position[2], self_position[3]]);
			true
		} else {
			false
		}
	}
	


	/* CHILD METHODS */

	/// Get all children.
	fn children(&self) -> &Vec<Box<dyn Drawable>> {
		self.data_set().children()
	}

	/// Get all children mutable.
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Drawable>> {
		self.data_set_mut().children_mut()
	}

	/// Find a child based on its name.
	fn child_by_name(&self, name:&str) -> Option<&dyn Drawable> {
		self.child_by_path(&name.split(CHILD_PATH_SPLITTER).collect::<Vec<&str>>()[..])
	}

	/// Find a child based on its name.
	fn child_by_name_mut(&mut self, name:&str) -> Option<&mut dyn Drawable> {
		self.child_by_path_mut(&name.split(CHILD_PATH_SPLITTER).collect::<Vec<&str>>()[..])
	}

	/// Find a child based on its path.
	fn child_by_path(&self, path:&[&str]) -> Option<&dyn Drawable> {
		for child in self.data_set().children() {
			if child.name() == path[0] {
				if path.len() == 1 {
					return Some(&**child);
				} else if let Some(found_child) = child.child_by_path(&path[1..]) {
    						return Some(found_child);
    					}
			} else if let Some(found_child) = child.child_by_path(path) {
				return Some(found_child);
			}
		}
		None
	}

	/// Find a child based on its path.
	fn child_by_path_mut(&mut self, path:&[&str]) -> Option<&mut dyn Drawable> {
		for child in self.data_set_mut().children_mut() {
			if child.name() == path[0] {
				if path.len() == 1 {
					return Some(&mut **child);
				} else if let Some(found_child) = child.child_by_path_mut(&path[1..]) {
    						return Some(found_child);
    					}
			} else if let Some(found_child) = child.child_by_path_mut(path) {
				return Some(found_child);
			}
		}
		None
	}

	/// Add a child element.
	fn add_child(&mut self, child:&dyn Drawable) {
		self.data_set_mut().add_child(child);
	}

	/// Set all children.
	fn set_children(&mut self, children:Vec<&dyn Drawable>) {
		self.remove_children();
		for child in children {
			self.data_set_mut().add_child(child);
		}
	}

	/// Remove a child element by its index.
	fn remove_child(&mut self, index:usize) {
		self.data_set_mut().children_mut().remove(index);
	}

	/// Remove all children.
	fn remove_children(&mut self) {
		*self.data_set_mut().children_mut() = Vec::new();
	}

	/// Make selfs children equal to the children of another element.
	fn set_children_from(&mut self, child_source:&dyn Drawable) {
		self.remove_children();
		for child in child_source.children() {
			self.add_child(&**child);
		}
	}

	/// Print all children as a tree.
	fn print_tree(&self, depth:usize) {
		println!("{}{}", "  ".repeat(depth), self.name());
		self.data_set().children().iter().for_each(|child| child.print_tree(depth + 1));
	}



	/* LISTENER METHODS */

	/// Check if self or any children has listeners.
	fn has_listeners(&self) -> bool {
		self.data_set().has_listeners()
	}

	/// Add a listener to this element.
	fn add_listener(&mut self, name:&str, listener_type:ListenerType, handler:ListenerCallback) {
		self.data_set_mut().add_listener(name, listener_type, handler);
	}

	/// Remove all listeners with this name.
	fn remove_listener(&mut self, name:&str) {
		self.data_set_mut().remove_listener(name);
	}

	/// Update and handle all own and child listeners.
	fn update_listeners(&mut self, mouse_position:&[usize; 2], mouse_down:&[bool; 2], initial:&[bool; 2]) {
		if self.data_set().get_setting_value_or::<bool>("block_listeners", false) { return; }
		self.data_set_mut().update_listeners(mouse_position, mouse_down, initial);
		self.data_set_mut().children_mut().iter_mut().for_each(|child| child.update_listeners(mouse_position, mouse_down, initial));
	}

	/// Suppress listeners of this element and children.
	fn suppress_listeners(&mut self) {
		self.data_set_mut().set_setting_value("block_listeners", true);
	}
	
	/// Stop suppressing listeners of this element and children.
	fn unsuppress_listeners(&mut self) {
		self.data_set_mut().set_setting_value("block_listeners", false);
	}


	
	/* DRAW METHODS */
	
	/// Draw this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		self.draw_children()
	}

	/// Draw each child of this instance.
	fn draw_children(&mut self) -> DrawBuffer {
		let child_bounding_box:[usize; 4] = self.children_bounding_box();
		let buffer_size:[usize; 2] = [child_bounding_box[0] + child_bounding_box[2], child_bounding_box[1] + child_bounding_box[3]];
		let children:&mut Vec<Box<dyn Drawable>> = self.data_set_mut().children_mut();
		let mut combined_buffer:DrawBuffer = DrawBuffer::new(vec![0x00000000; buffer_size[0] * buffer_size[1]], buffer_size[0], buffer_size[1]);
		for child in children {
			let child_position:[usize; 4] = child.position();
			combined_buffer = combined_buffer.append_at(child.draw(), child_position[0] as isize, child_position[1] as isize);
		}
		combined_buffer
	}

	/// Draw the buffer to a PNG file.
	fn draw_png(&mut self, path:&str) -> Result<(), Box<dyn Error>> {
		use image::{ save_buffer, ColorType::Rgba8 };
		use std::path::Path;

		let buffer:DrawBuffer = self.draw();
		let rgba_data:Vec<u8> = buffer.data.iter().flat_map(|color| [((color & 0xFF0000) >> 16) as u8, ((color & 0xFF00) >> 8) as u8, (color & 0xFF) as u8, ((color & 0xFF000000) >> 24) as u8]).collect::<Vec<u8>>();
		match save_buffer(Path::new(path), &rgba_data[..], buffer.width as u32, buffer.height as u32, Rgba8) {
			Ok(_) => Ok(()),
			Err(err) => Err(err.into())
		}
	}

	

	/* SPECIFIC SCENARIO METHODS */

	/// If the drawable is a 3D scene, fetch it.
	fn as_scene(&mut self) -> Result<&mut crate::elements::Scene, Box<dyn Error>> {
		Err(format!("{} cannot be interpreted as a scene.", self.name()).into())
	}
}