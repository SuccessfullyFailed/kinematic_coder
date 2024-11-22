use dynamic_data_storage::GeneralDataType;
use std::error::Error;

#[derive(Clone)]
pub struct MotorConfig {
	position:[f32; 3],
	rotation_axis:u8,
	default_rotation:f32,
	rotation_range:[f32; 2],
	current_rotation:f32
}
impl MotorConfig {

	/* CONSTRUCTOR METHODS */

	/// Create a new motor config.
	pub fn new(position:[f32; 3], rotation_axis:u8, default_rotation:f32, rotation_range:[f32; 2]) -> MotorConfig {
		MotorConfig {
			position,
			rotation_axis,
			default_rotation,
			rotation_range: [rotation_range[0].min(rotation_range[1]), rotation_range[0].max(rotation_range[1])],
			current_rotation: default_rotation
		}
	}

	/// Create a new empty motor config.
	pub fn empty() -> MotorConfig {
		MotorConfig::new([0.0; 3], 0, 0.0, [-45.0, 45.0])
	}
	


	/* PROPERTY GETTER METHODS */

	/// Return a reference to the position.
	pub fn position(&self) -> &[f32; 3] {
		&self.position
	}

	/// Return a mutable reference to the position.
	pub fn position_mut(&mut self) -> &mut [f32; 3] {
		&mut self.position
	}

	/// Return a reference to the rotation_axis.
	pub fn rotation_axis(&self) -> &u8 {
		&self.rotation_axis
	}

	/// Return a mutable reference to the rotation_axis.
	pub fn rotation_axis_mut(&mut self) -> &mut u8 {
		&mut self.rotation_axis
	}

	/// Return a reference to the default_rotation.
	pub fn default_rotation(&self) -> &f32 {
		&self.default_rotation
	}

	/// Return a mutable reference to the default_rotation.
	pub fn default_rotation_mut(&mut self) -> &mut f32 {
		&mut self.default_rotation
	}

	/// Return a reference to the rotation_range.
	pub fn rotation_range(&self) -> &[f32; 2] {
		&self.rotation_range
	}

	/// Return a mutable reference to the rotation_range.
	pub fn rotation_range_mut(&mut self) -> &mut [f32; 2] {
		&mut self.rotation_range
	}

	/// Return a reference to the current_rotation.
	pub fn current_rotation(&self) -> &f32 {
		&self.current_rotation
	}

	/// Return a mutable reference to the current_rotation.
	pub fn current_rotation_mut(&mut self) -> &mut f32 {
		&mut self.current_rotation
	}
}

impl GeneralDataType for MotorConfig {

	/// Create a value of the implemented type from these bytes.
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn Error>> {
		Ok(MotorConfig {
			position: [
				f32::from_bytes(&bytes[0..4])?,
				f32::from_bytes(&bytes[4..8])?,
				f32::from_bytes(&bytes[8..12])?
			],
			rotation_axis: u8::from_bytes(&bytes[12..13])?,
			default_rotation: f32::from_bytes(&bytes[13..17])?,
			rotation_range: [
				f32::from_bytes(&bytes[17..21])?,
				f32::from_bytes(&bytes[21..25])?
			],
			current_rotation: f32::from_bytes(&bytes[25..29])?
		})
	}

	/// Create a list of bytes from the value.
	fn to_bytes(&self) -> Vec<u8> {
		let mut bytes:Vec<u8> = Vec::new();
		bytes.extend_from_slice(&self.position[0].to_bytes());
		bytes.extend_from_slice(&self.position[1].to_bytes());
		bytes.extend_from_slice(&self.position[2].to_bytes());
		bytes.extend_from_slice(&(self.rotation_axis).to_bytes());
		bytes.extend_from_slice(&self.default_rotation.to_bytes());
		bytes.extend_from_slice(&self.rotation_range[0].to_bytes());
		bytes.extend_from_slice(&self.rotation_range[1].to_bytes());
		bytes.extend_from_slice(&self.current_rotation.to_bytes());
		bytes
	}

	/// Get the byte size of this type.
	fn byte_size() -> usize {
		29
	}
}