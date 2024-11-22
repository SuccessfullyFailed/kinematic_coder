use crate::{ GeneralDataType, STORAGE_FILE_EXTENSION };
use std::error::Error;
use super::NamedData;

static FILE_RECOGNITION_TAG:&str = "<<DynamicDataStorageFile>>";


pub struct DataCategory {
	name:String,
	source:String,
	data:Vec<NamedData>
}
impl DataCategory {

	/// Create a new instance.
	pub fn new(name:&str, source:&str) -> DataCategory {
		use std::path::Path;

		let full_source:String = if source.contains(STORAGE_FILE_EXTENSION) { source.to_string() } else { format!("{source}.{STORAGE_FILE_EXTENSION}") };

		// Try to load data from file if exist.
		if Path::new(&full_source).exists() {
			if let Ok(category) = Self::load_from_file(&full_source) {
				return category;
			}
		}

		// Create new category.
		DataCategory {
			name: name.to_string(),
			source: full_source,
			data: Vec::new()
		}
	}



	/* PROPERTY GETTER AND SETTER METHODS */

	/// Return a reference to the name.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// Return a reference to the source.
	pub fn source(&self) -> &String {
		&self.source
	}

	/// Update the source of the category. Does not read the data at the new path, but rather stores it there from now on.
	pub fn set_source(&mut self, source:&str) {
		self.source = if source.contains(STORAGE_FILE_EXTENSION) { source.to_string() } else { format!("{source}.{STORAGE_FILE_EXTENSION}") };
	}

	/// Return a reference to the data.
	pub fn data(&self) -> &Vec<NamedData> {
		&self.data
	}

	/// Return a mutable reference to the data.
	pub fn data_mut(&mut self) -> &mut Vec<NamedData> {
		&mut self.data
	}


	
	/* VALUE METHODS */

	/// Find a named data.
	pub fn find(&self, name:&str) -> Option<&NamedData> {
		self.data.iter().find(|named_data| named_data.name() == name)
	}

	/// Find a mutable named data.
	pub fn find_mut(&mut self, name:&str) -> Option<&mut NamedData> {
		self.data.iter_mut().find(|named_data| named_data.name() == name)
	}

	/// Get a specific variable.
	pub fn get_value<T:GeneralDataType>(&self, name:&str) -> Result<T, Box<dyn Error>> {
		match self.find(name) {
			Some(data) => data.get_value::<T>(),
			None => Err(format!("Could not find variable '{}' in category '{}'.", name, self.name).into())
		}
	}

	/// Set a specific variable.
	pub fn set_value<T:GeneralDataType>(&mut self, name:&str, value:&T) -> Result<(), Box<dyn Error>> {

		// Write value to memory.
		match self.find_mut(name) {
			Some(data) => data.set_value::<T>(value),
			None => self.data.push(NamedData::new::<T>(name, value))
		}

		// Write memory to file.
		self.store_to_file()
	}



	/* DISK STORAGE METHODS */

	/// Write own memory to file.
	pub fn store_to_file(&self) -> Result<(), Box<dyn Error>> {
		use std::{ io::Write, path::Path, fs::{ File, create_dir } };
		
		// Turn data into writable bytes.
		let mut output_bytes:Vec<u8> = format!("{}{}:", FILE_RECOGNITION_TAG, self.name).to_bytes();
		for variable in &self.data {
			let name:Vec<u8> = variable.name().to_string().to_bytes();
			let value:Vec<u8> = variable.value::<Vec<u8>>().unwrap();
			output_bytes.extend_from_slice(&(name.len() as u16).to_bytes());
			output_bytes.extend_from_slice(&name);
			output_bytes.extend_from_slice(&(value.len() as u16).to_bytes());
			output_bytes.extend_from_slice(&value);
		}
		
		// Make sure parent dir exists.
		let path_components:Vec<&str> = self.source.split('/').collect::<Vec<&str>>();
		for length in 1..path_components.len() {
			let dir:String = path_components[..length].join("/");
			if !Path::new(&dir).exists() {
				create_dir(dir)?;
			}
		}

		// Write string to file.
		let mut file:File = File::create(&self.source)?;
		file.write_all(&output_bytes[..])?;
		
		Ok(())
	}

	/// Read category from file.
	pub fn load_from_file(file:&str) -> Result<DataCategory, Box<dyn Error>> {
		use std::fs::read;

		// Validate path.
		if !file.ends_with(&format!(".{STORAGE_FILE_EXTENSION}")) {
			return Err(format!("Could not read '{file}' as source, file is not recognized as DynamicDataStorage file due to its extension.").into());
		}
		
		// Read file.
		let file_contents:Vec<u8> = read(file)?;

		// Validate file recognition tag.
		if String::from_bytes(&file_contents[0..FILE_RECOGNITION_TAG.len()])? != FILE_RECOGNITION_TAG {
			return Err(format!("Could not read '{file}' as source, file is not recognized as DynamicDataStorage file due to its contents.").into());
		}

		// Find out category name.
		let mut cursor:usize = FILE_RECOGNITION_TAG.len();
		while file_contents[cursor] != b':' && cursor < file_contents.len() {
			cursor += 1;
		}
		if cursor >= file_contents.len() {
			return Err(format!("Could not read '{file}' as source, no category name exists.").into());
		}
		let category_name:String = String::from_bytes(&file_contents[FILE_RECOGNITION_TAG.len()..cursor])?;
		let mut category:DataCategory = DataCategory { name: category_name, source: file.to_string(), data: Vec::new() };

		// Read variables.
		cursor += 2;
		while cursor < file_contents.len() {

			// Variable name.
			let name_len:usize = u16::from_bytes(&file_contents[cursor..cursor+2])? as usize;
			cursor += 2;
			let name:String = String::from_bytes(&file_contents[cursor..cursor+name_len])?;
			cursor += name_len;

			// Variable data.
			let value_len:usize = u16::from_bytes(&file_contents[cursor..cursor+2])? as usize;
			cursor += 2;
			let value:&[u8] = &file_contents[cursor..cursor+value_len];
			cursor += value_len;

			// Add to category.
			category.set_value::<Vec<u8>>(&name, &value.to_vec())?;
		}
		
		// Return category.
		Ok(category)
	}

	/// Remove the file.
	pub fn remove_file(&self) -> Result<(), Box<dyn Error>> {
		use std::{path::Path, fs::remove_file};

		let path:&Path = Path::new(&self.source);
		if path.exists() {
			match remove_file(path) {
				Ok(_) => Ok(()),
				Err(error) => Err(Box::new(error))
			}
		} else {
			Ok(())
		}
	}
}