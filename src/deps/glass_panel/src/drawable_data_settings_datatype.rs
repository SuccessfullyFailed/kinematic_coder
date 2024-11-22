pub trait DrawableDataSettingDataType: Sized {

	/// Validate the value can be built from this byte count, then move on to from_bytes_inner. Will result in an error of value of the implemented type.
	fn from_bytes(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> {

		// Check byte size.
		let target_byte_size:usize = Self::byte_size();
		if bytes.len() != target_byte_size {
			return Err(format!("DRAWABLE DATA SETTING TYPE ERROR: Invalid byte size. expected {} bytes, got {}.", target_byte_size, bytes.len()).into());
		}

		// Continue on to inner function.
		Self::from_bytes_inner(bytes)
	}

	/// Create a value of the implemented type from these bytes.
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>>;

	/// Create a list of bytes from the value.
	fn to_bytes(&self) -> Vec<u8>;

	/// Get the byte size of this type.
	fn byte_size() -> usize;
}



/* UNSIGNED INT */

impl DrawableDataSettingDataType for usize {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(usize::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) }
	fn byte_size() -> usize { 8 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for u64 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) }
	fn byte_size() -> usize { 8 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for u32 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) }
	fn byte_size() -> usize { 4 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for u16 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(u16::from_le_bytes([bytes[0], bytes[1]])) }
	fn byte_size() -> usize { 2 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for u8 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(u8::from_le_bytes([bytes[0]])) }
	fn byte_size() -> usize { 1 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}



/* SIGNED INT */

impl DrawableDataSettingDataType for isize {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(isize::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) }
	fn byte_size() -> usize { 8 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for i64 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(i64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) }
	fn byte_size() -> usize { 8 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for i32 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) }
	fn byte_size() -> usize { 4 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for i16 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(i16::from_le_bytes([bytes[0], bytes[1]])) }
	fn byte_size() -> usize { 2 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for i8 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(i8::from_le_bytes([bytes[0]])) }
	fn byte_size() -> usize { 1 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}



/* FLOATING POINT NUMBERS */

impl DrawableDataSettingDataType for f64 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(f64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) }
	fn byte_size() -> usize { 8 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}
impl DrawableDataSettingDataType for f32 {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) }
	fn byte_size() -> usize { 4 }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
}



/* MISCELLANEOUS */

impl DrawableDataSettingDataType for bool {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(bytes[0] == 1) }
	fn byte_size() -> usize { 1 }
	fn to_bytes(&self) -> Vec<u8> { vec![if *self { 1 } else { 0 }] }
}
impl DrawableDataSettingDataType for String {
	fn from_bytes(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(bytes.iter().map(|byte| *byte as char).collect::<String>()) }
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(String::from_utf8_lossy(bytes).to_string()) }
	fn byte_size() -> usize { 0 }
	fn to_bytes(&self) -> Vec<u8> { self.chars().map(|character| character as u8).collect::<Vec<u8>>() }
}
impl DrawableDataSettingDataType for Vec<u8> {
	fn from_bytes(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(bytes.to_vec()) }
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn std::error::Error>> { Ok(bytes.to_vec()) }
	fn byte_size() -> usize { 0 }
	fn to_bytes(&self) -> Vec<u8> { self[..].to_vec() }
}