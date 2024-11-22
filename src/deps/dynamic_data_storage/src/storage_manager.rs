use super::{ DataCategory, GeneralDataType };
use std::error::Error;

pub static STORAGE_FILE_EXTENSION:&str = "ddstrg";

// A simpleton struct that provides and stores data. Determines where and how to store it by itself.
static mut STATIC_DATA:StorageManager = StorageManager { categories: Vec::new() };
pub struct StorageManager {
	categories:Vec<DataCategory>
}
impl StorageManager {

	/* GETTER METHODS */

	/// Get a reference to the static manager.
	pub fn get() -> &'static StorageManager {
		unsafe { &STATIC_DATA }
	}

	/// Get a mutable reference to the static manager.
	pub fn get_mut() -> &'static mut StorageManager {
		unsafe { &mut STATIC_DATA }
	}

	/// Hard-reset data.
	#[allow(unused)]
	pub fn hard_reset() {
		unsafe { STATIC_DATA = StorageManager { categories: Vec::new() }; }
	}



	/* CATEGORY METHODS */

	/// Find a specific category and return a reference to it.
	pub fn find_category(&self, name:&str) -> Option<&DataCategory> {
		self.categories.iter().find(|category| category.name() == name)
	}

	/// Find a specific category and return a mutable reference to it.
	pub fn find_category_mut(&mut self, name:&str) -> Option<&mut DataCategory> {
		self.categories.iter_mut().find(|category| category.name() == name)
	}

	/// Set all categories.
	pub fn set_categories(&mut self, category_data:Vec<(&str, &str)>) -> Result<(), Box<dyn Error>> {

		// Add non-existing categories and update the sources of existing categories.
		for (name, source) in &category_data {
			match self.find_category_mut(name) {
				Some(category) => category.set_source(source),
				None => self.add_category(name, source)
			}
		}

		// Remove existing categories not included in the given dataset.
		let required_names:Vec<&str> = category_data.iter().map(|category| category.0).collect::<Vec<&str>>();
		let current_category_names:Vec<String> = self.categories.iter().map(|category| category.name().to_string()).collect::<Vec<String>>();
		let categories_to_remove:Vec<&String> = current_category_names.iter().filter(|category| !required_names.contains(&category.as_str())).collect::<Vec<&String>>();
		for category in categories_to_remove {
			self.remove_category(category)?;
		}

		Ok(())
	}

	/// Add a new category to the instance.
	pub fn add_category(&mut self, name:&str, source:&str) {
		if self.find_category(name).is_none() {
			self.categories.push(DataCategory::new(name, source));
		}
	}

	/// Add multiple categories.
	pub fn add_categories(&mut self, category_data:Vec<(&str, &str)>) {
		for (name, source) in category_data {
			self.add_category(name, source);
		}
	}

	/// Add a category from a file.
	pub fn add_category_from_file(&mut self, file:&str) -> Result<(), Box<dyn Error>> {
		self.categories.push(DataCategory::load_from_file(file)?);
		Ok(())
	}

	/// Add a category from a file.
	pub fn add_categories_from_dir(&mut self, dir:&str, recurse:bool) {
		use std::fs::{read_dir, metadata};

		// Loop through files.
		if let Ok(results) = read_dir(dir) {
			for dir_entry in results.flatten() {

				// Extract path and wether it is a dir or not.
				let path:String = dir_entry.path().to_str().unwrap().to_owned();
				let is_dir:bool = match metadata(&path) {
					Ok(md) => md.is_dir(),
					Err(_) => false
				};
				
				// Parse dir.
				if is_dir && recurse {
					self.add_categories_from_dir(&path, recurse);
				}

				// Parse file. Unwanted files are filtered out automatically.
				let _ = self.add_category_from_file(&path);
			}
		}
	}

	/// Remove a category.
	pub fn remove_category(&mut self, category_name:&str) -> Result<(), Box<dyn Error>> {
		if let Some(index) = self.categories.iter().position(|category| category.name() == category_name) {
			self.categories[index].remove_file()?;
			self.categories.remove(index);
			Ok(())
		} else {
			Err(format!("Could not find category {category_name} to remove").into())
		}
	}



	/* VALUE METHODS */

	/// An extremely shortened way to get a value or use a default value. DOES NOT STORE DEFAULT VALUE.
	pub fn quick<T:GeneralDataType>(&mut self, name:&str, default:T) -> T {
		if let Ok(value) = self.find_value::<T>(name) {
			value
		} else {
			default
		}
	}

	/// Get a value from any of the categories.
	pub fn find_value<T:GeneralDataType>(&self, variable_name:&str) -> Result<T, Box<dyn Error>> {
		for category in &self.categories {
			if let Ok(value) = category.get_value::<T>(variable_name) {
				return Ok(value);
			}
		}
		Err(format!("Could not find value '{variable_name}' in any category in storage").into())
	}

	/// Get a value from a specific category.
	pub fn get_value<T:GeneralDataType>(&self, category_name:&str, variable_name:&str) -> Result<T, Box<dyn Error>> {
		match self.find_category(category_name)	{
			Some(category) => match category.get_value::<T>(variable_name) {
				Ok(value) => Ok(value),
				Err(error) => Err(error)
			},
			None => Err(format!("Could not find category '{category_name}' in storage").into())
		}
	}

	/// Get a value from a specific category or set a default (will be stored).
	pub fn get_value_or<T:GeneralDataType>(&mut self, category_name:&str, variable_name:&str, default_value:T) -> T {
		match self.get_value::<T>(category_name, variable_name) {
			Ok(val) => val,
			Err(_) => {
				if let Err(error) = self.set_value(category_name, variable_name, &default_value) {
					eprintln!("{error}");
				}
				default_value
			}
		}
	}

	/// Set a value from a specific category.
	pub fn set_value<T:GeneralDataType>(&mut self, category_name:&str, variable_name:&str, value:&T) -> Result<(), Box<dyn Error>> {
		match self.find_category_mut(category_name)	{
			Some(category) => category.set_value(variable_name, value),
			None => Err(format!("Could not find category '{category_name}' in storage").into())
		}
	}
}