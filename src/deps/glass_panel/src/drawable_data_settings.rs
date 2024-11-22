use crate::DrawableDataSettingDataType;

/*
	A settings object should never be able to be accessed directly.
	By calling specific functions in the settings object, the settings object can decide if the cache is outdated without manually having to do so somewhere else in the code.
*/
#[derive(Clone)]
pub struct DrawableDataSetting {
	pub name:String,
	pub value:Vec<u8>
}
impl DrawableDataSetting {

	/// Create a new setting instance.
	pub fn new<T: DrawableDataSettingDataType>(name:&str, value:T) -> DrawableDataSetting {
		DrawableDataSetting {
			name: String::from(name),
			value: value.to_bytes()
		}
	}
}


#[derive(Clone)]
pub struct DrawableDataSettings {
	entries:Vec<DrawableDataSetting>
}
impl DrawableDataSettings {

	/// Create a new list of settings.
	pub fn new<T: DrawableDataSettingDataType>(data:Vec<(&str, T)>) -> DrawableDataSettings {
		DrawableDataSettings {
			entries: data.iter().map(|entry| DrawableDataSetting::new::<Vec<u8>>(entry.0, entry.1.to_bytes())).collect::<Vec<DrawableDataSetting>>()
		}
	}

	/// List all settings.
	pub fn list<T: DrawableDataSettingDataType>(&self) -> Vec<(String, T)> {
		self.entries.iter().map(|e| (String::from(&e.name), T::from_bytes(&e.value[..]).unwrap())).collect::<Vec<(String, T)>>()
	}

	/// Get the value of a setting.
	pub fn get<T: DrawableDataSettingDataType>(&self, name:&str) -> Option<T> {
		let found_entries:Vec<&DrawableDataSetting> = self.entries.iter().filter(|entry| entry.name == name).collect::<Vec<&DrawableDataSetting>>();
		if !found_entries.is_empty() {
			Some(T::from_bytes(&found_entries[0].value[..]).unwrap())
		} else {
			None
		}
	}

	/// Get the value of a setting or a default value.
	pub fn get_or<T: DrawableDataSettingDataType>(&self, name:&str, default_value:T) -> T {
		match self.get::<T>(name) {
			Some(value) => value,
			None => default_value
		}
	}

	/// Set the value of a setting.
	pub fn set<T: DrawableDataSettingDataType>(&mut self, name:&str, value:T) {
		let mut found_entries:Vec<&mut DrawableDataSetting> = self.entries.iter_mut().filter(|entry| entry.name == name).collect::<Vec<&mut DrawableDataSetting>>();
		if !found_entries.is_empty() {
			found_entries[0].value = value.to_bytes();
		} else {
			self.entries.push(DrawableDataSetting::new(name, value.to_bytes()));
		}
	}
}