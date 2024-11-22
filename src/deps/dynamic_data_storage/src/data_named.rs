use super::GeneralDataType;
use std::error::Error;

pub struct NamedData {
	name:String,
	data:Vec<u8>,
}
impl NamedData {

	/// Create a new instance.
	pub fn new<T:GeneralDataType>(name:&str, value:&T) -> NamedData {
		NamedData {
			name: name.to_string(),
			data: value.to_bytes(),
		}
	}
	
	/// Get the name of the data.
	pub fn name(&self) -> &str {
		&self.name
	}
	
	/// Get the value from the data.
	pub fn value<T:GeneralDataType>(&self) -> Result<T, Box<dyn Error>> {
		T::from_bytes(&self.data)
	}
	
	/// Get the value from the data.
	pub fn get_value<T:GeneralDataType>(&self) -> Result<T, Box<dyn Error>> {
		self.value::<T>()
	}

	/// Set a new value.
	pub fn set_value<T:GeneralDataType>(&mut self, value:&T) {
		self.data = value.to_bytes();
	}
}