use crate::{ Drawable, DrawableDataSettingDataType, DrawableDataSettings, Listener, ListenerCallback, ListenerType };
use std::time::{ Duration, Instant };

/* Whenever something mutable is accessed, assume the cache is outdated. */

static CACHE_AGE:Duration = Duration::from_millis(2500);
pub struct DrawableData {
	children:Vec<Box<dyn Drawable>>,
	settings:DrawableDataSettings,
	named_listeners:Vec<(String, Listener)>,

	/* Cache */
	cache_outdated:bool,
	absolute_position:[usize; 4],
	cache_cache_last_updated:Instant
}
impl DrawableData {

	/// Create a new instance.
	pub fn new<T: DrawableDataSettingDataType>(settings:Vec<(&str, T)>, children:Vec<&dyn Drawable>) -> DrawableData {
		DrawableData {
			children: children.iter().map(|child| child.boxify()).collect::<Vec<Box<dyn Drawable>>>(),
			settings: DrawableDataSettings::new::<T>(settings),
			named_listeners: Vec::new(),

			cache_outdated: true,
			absolute_position: [0; 4],
			cache_cache_last_updated: Instant::now()
		}
	}



	/* CHILDREN METHODS */

	/// Get a reference to the sets children.
	pub fn children(&self) -> &Vec<Box<dyn Drawable>> {
		&self.children
	}

	/// Get a reference to the sets children.
	pub fn children_mut(&mut self) -> &mut Vec<Box<dyn Drawable>> {
		self.cache_outdated = true;
		&mut self.children
	}

	/// Get the children in the data set as reference.
	pub fn children_unboxed(&self) -> Vec<&dyn Drawable> {
		self.children().iter().map(|boxed_child| &(**boxed_child)).collect::<Vec<&dyn Drawable>>()
	}

	/// Add a child.
	pub fn add_child(&mut self, child:&dyn Drawable) {
		self.children_mut().push(child.boxify());
		self.cache_outdated = true;
	}



	/* SETTING METHODS */

	/// Get the value of a setting from the dataset.
	pub fn get_setting_value<T: DrawableDataSettingDataType>(&self, name:&str) -> Option<T> {
		self.settings.get(name)
	}

	/// Get the value of a setting from the dataset or return a default.
	pub fn get_setting_value_or<T: DrawableDataSettingDataType>(&self, name:&str, default:T) -> T {
		match self.get_setting_value(name) {
			Some(value) => value,
			None => default
		}
	}

	/// Set the value of a setting from the dataset.
	pub fn set_setting_value<T: DrawableDataSettingDataType>(&mut self, name:&str, value:T) {
		if let Some(current_value) = self.get_setting_value::<T>(name) {
			if current_value.to_bytes() == value.to_bytes() {
				return;
			}
		}
		self.settings.set(name, value);
		self.cache_outdated = true;
	}

	/// Set the value of a setting from the dataset.
	pub fn set_settings_values<T: DrawableDataSettingDataType>(&mut self, data:Vec<(&str, T)>) {
		for (name, value) in data {
			self.set_setting_value::<T>(name, value);
		}
	}



	/* LISTENER METHODS */

	pub fn has_listeners(&self) -> bool {
		!self.named_listeners.is_empty() || self.children().iter().map(|child| child.has_listeners()).filter(|has| *has).count() > 0
	}

	/// Add a listener to this element.
	pub fn add_listener(&mut self, name:&str, listener_type:ListenerType, handler:ListenerCallback) {
		self.named_listeners.push((String::from(name), Listener::new(listener_type, handler)));
	}

	/// Remove all listeners with this name.
	pub fn remove_listener(&mut self, name:&str) {
		for index in (0..self.named_listeners.len()).rev() {
			if self.named_listeners[index].0 == name {
				self.named_listeners.remove(index);
			}
		}
	}

	/// Update and handle all listeners.
	pub fn update_listeners(&mut self, mouse_position:&[usize; 2], mouse_down:&[bool; 2], initial:&[bool; 2]) {
		
		// Create a temporary clone to be able to give self to the listener.
		for listener_index in 0..self.named_listeners.len() {
			let (name, mut listener) = self.named_listeners[listener_index].clone();
			listener.update(mouse_position, &self.absolute_position.clone(), mouse_down, initial, &name, self);
			self.named_listeners[listener_index].1 = listener;
		}
	}



	/* CACHE METHODS */

	/// Update the entire cache.
	pub fn update(&mut self, absolute_position:&[usize; 4]) {
		if self.requires_update() {
			self.absolute_position = [absolute_position[0], absolute_position[1], absolute_position[2], absolute_position[3]];
			self.children_mut().iter_mut().for_each(|child| { child.update(&[absolute_position[0], absolute_position[1]]); });
			self.cache_cache_last_updated = Instant::now();
			self.cache_outdated = false;
		}
	}

	/// Manually trigger an update.
	pub fn trigger_update(&mut self) {
		self.cache_outdated = true;
	}

	/// Check if the cache requires an update.
	pub fn requires_update(&self) -> bool {
		self.cache_outdated || self.cache_cache_last_updated.elapsed().as_millis() > CACHE_AGE.as_millis() || self.children().iter().map(|child| child.data_set().requires_update()).filter(|b| *b).count() > 0
	}
	
	/// Get the absolute position.
	pub fn absolute_position(&self) -> &[usize; 4] {
		&self.absolute_position
	}
}
impl Clone for DrawableData {
	fn clone(&self) -> DrawableData {
		DrawableData {
			children: self.children.iter().map(|boxed_child| boxed_child.boxify()).collect::<Vec<Box<dyn Drawable>>>(),
			settings: self.settings.clone(),
			named_listeners: self.named_listeners.clone(),

			cache_outdated: true,
			absolute_position: [self.absolute_position[0], self.absolute_position[1], self.absolute_position[2], self.absolute_position[3]],
			cache_cache_last_updated: self.cache_cache_last_updated
		}
	}
}