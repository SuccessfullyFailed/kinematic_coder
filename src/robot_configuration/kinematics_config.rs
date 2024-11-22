use dynamic_data_storage::GeneralDataType;
use std::error::Error;

#[derive(Clone)]
pub struct KinematicsConfig {
	leg_endpoint:[f32; 3],
	step_position:[f32; 3],
	step_distance:f32,
	step_distance_override:Option<f32>,
	step_height:f32
}
impl KinematicsConfig {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new instance.
	pub fn new(leg_endpoint:[f32; 3], step_position:[f32; 3], step_distance:f32, step_height:f32) -> KinematicsConfig {
		KinematicsConfig {
			leg_endpoint,
			step_position,
			step_distance,
			step_distance_override: None,
			step_height
		}
	}

	/// Create an empty instance.
	pub fn empty() -> KinematicsConfig {
		KinematicsConfig::new([0.0; 3], [0.0; 3], 0.0, 0.0)
	}



	/* PROPERTY GETTER METHODS */

	/// Return a reference to the leg_endpoint.
	pub fn leg_endpoint(&self) -> &[f32; 3] {
		&self.leg_endpoint
	}

	/// Return a mutable reference to the leg_endpoint.
	pub fn leg_endpoint_mut(&mut self) -> &mut [f32; 3] {
		&mut self.leg_endpoint
	}

	/// Return a reference to the step_position.
	pub fn step_position(&self) -> &[f32; 3] {
		&self.step_position
	}

	/// Return a mutable reference to the step_position.
	pub fn step_position_mut(&mut self) -> &mut [f32; 3] {
		&mut self.step_position
	}

	/// Return a reference to the step_distance.
	pub fn step_distance(&self) -> &f32 {
		match &self.step_distance_override {
			Some(distance) => distance,
			None => &self.step_distance
		}
	}

	/// Return a mutable reference to the step_distance.
	pub fn step_distance_mut(&mut self) -> &mut f32 {
		&mut self.step_distance
	}

	/// Return a mutable version of the override step distance.
	pub(crate) fn step_distance_override(&mut self) -> &mut Option<f32> {
		&mut self.step_distance_override
	}

	/// Return a reference to the step_height.
	pub fn step_height(&self) -> &f32 {
		&self.step_height
	}

	/// Return a mutable reference to the step_height.
	pub fn step_height_mut(&mut self) -> &mut f32 {
		&mut self.step_height
	}
}

impl GeneralDataType for KinematicsConfig {

	/// Create a value of the implemented type from these bytes while removing the bytes required from the bytes list. Useful for parsing more advances structs.
	fn from_bytes_consume(bytes:&mut Vec<u8>) -> Result<Self, Box<dyn Error>> {
		Ok(KinematicsConfig::new(
			[
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?
			],
			[
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?
			],
			f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
			f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?
		))
	}

	/// Create a value of the implemented type from these bytes.
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn Error>> {
		Self::from_bytes_consume(&mut bytes.to_vec())
	}

	/// Create a list of bytes from the value.
	fn to_bytes(&self) -> Vec<u8> {
		let mut bytes:Vec<u8> = Vec::new();
		bytes.extend_from_slice(&self.leg_endpoint[0].to_bytes());
		bytes.extend_from_slice(&self.leg_endpoint[1].to_bytes());
		bytes.extend_from_slice(&self.leg_endpoint[2].to_bytes());
		bytes.extend_from_slice(&self.step_position[0].to_bytes());
		bytes.extend_from_slice(&self.step_position[1].to_bytes());
		bytes.extend_from_slice(&self.step_position[2].to_bytes());
		bytes.extend_from_slice(&self.step_distance.to_bytes());
		bytes.extend_from_slice(&self.step_height.to_bytes());
		bytes
	}

	/// Get the byte size of this type.
	fn byte_size() -> usize {
		32
	}
}