use super::{ MotorConfig, KinematicsConfig };
use dynamic_data_storage::GeneralDataType;
use std::error::Error;

#[derive(Clone)]
pub struct LegConfig {
	obj:String,
	position:[f32; 3],
	rotation:[f32; 3],
	motor:Option<MotorConfig>,
	kinematics_config:Option<KinematicsConfig>
}
impl LegConfig {
	
	/* CONSTRUCTOR METHODS */

	/// Create a new instance.
	pub fn new(obj:&str) -> LegConfig {
		LegConfig {
			obj: obj.to_string(),
			position: [0.0; 3],
			rotation: [0.0; 3],
			motor: None,
			kinematics_config: None
		}
	}

	/// Create an empty instance.
	pub fn empty() -> LegConfig {
		LegConfig::new("")
	}



	/* PROPERTY GETTER METHODS */

	/// Return a reference to the obj.
	pub fn obj(&self) -> &String {
		&self.obj
	}

	/// Return a mutable reference to the obj.
	pub fn obj_mut(&mut self) -> &mut String {
		&mut self.obj
	}

	/// Return a reference to the position.
	pub fn position(&self) -> &[f32; 3] {
		&self.position
	}

	/// Return a mutable reference to the position.
	pub fn position_mut(&mut self) -> &mut [f32; 3] {
		&mut self.position
	}

	/// Return a reference to the rotation.
	pub fn rotation(&self) -> &[f32; 3] {
		&self.rotation
	}
	
	/// Return a mutable reference to the rotation.
	pub fn rotation_mut(&mut self) -> &mut [f32; 3] {
		&mut self.rotation
	}

	/// Return a reference to the motor.
	pub fn motor(&self) -> &Option<MotorConfig> {
		&self.motor
	}

	/// Return a mutable reference to the motor.
	pub fn motor_mut(&mut self) -> &mut Option<MotorConfig> {
		&mut self.motor
	}

	/// Return a reference to the kinematics_config.
	pub fn kinematics_config(&self) -> &Option<KinematicsConfig> {
		&self.kinematics_config
	}
	
	/// Return a mutable reference to the kinematics_config.
	pub fn kinematics_config_mut(&mut self) -> &mut Option<KinematicsConfig> {
		&mut self.kinematics_config
	}
}



use super::OBJ_PATH_MAX_LEN as OBJ_LEN;

impl GeneralDataType for LegConfig {

	/// Create a value of the implemented type from these bytes while removing the bytes required from the bytes list. Useful for parsing more advances structs.
	fn from_bytes_consume(bytes:&mut Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let obj_name:String = String::from_bytes(&bytes.drain(..OBJ_LEN + 1).collect::<Vec<u8>>())?;
		Ok(LegConfig {
			obj: obj_name,
			position: [
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?
			],
			rotation: [
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?,
				f32::from_bytes(&bytes.drain(..4).collect::<Vec<u8>>())?
			],
			motor: if bytes.remove(0) == 0 { None } else { Some(MotorConfig::from_bytes_consume(bytes)?) },
			kinematics_config: if bytes.remove(0) == 0 { None } else { Some(KinematicsConfig::from_bytes_consume(bytes)?) },
		})
	}

	/// Create a value of the implemented type from these bytes.
	fn from_bytes_inner(_:&[u8]) -> Result<Self, Box<dyn Error>> {
		panic!("Should not get LegConfig from bytes due to not having a set byte size, please use from_bytes_consume");
	}

	/// Create a list of bytes from the value.
	fn to_bytes(&self) -> Vec<u8> {

		// Validate obj path len.
		let obj_len:usize = self.obj.len();
		if obj_len > OBJ_LEN {
			panic!("Could not store leg config, name '{}' is too long.", &self.obj);
		}

		// Create bytes.
		let mut bytes:Vec<u8> = Vec::new();
		bytes.extend_from_slice(&[self.obj.to_bytes().to_vec(), vec![0u8; OBJ_LEN - obj_len]].iter().flatten().copied().collect::<Vec<u8>>());
		bytes.extend_from_slice(&self.position[0].to_bytes());
		bytes.extend_from_slice(&self.position[1].to_bytes());
		bytes.extend_from_slice(&self.position[2].to_bytes());
		bytes.extend_from_slice(&self.rotation[0].to_bytes());
		bytes.extend_from_slice(&self.rotation[1].to_bytes());
		bytes.extend_from_slice(&self.rotation[2].to_bytes());
		bytes.extend_from_slice(&match &self.motor {
			Some(motor) => [vec![1], motor.to_bytes()].iter().flatten().copied().collect::<Vec<u8>>(),
			None => vec![0]
		}[..]);
		bytes.extend_from_slice(&match &self.kinematics_config {
			Some(kinematics_config) => [vec![1], kinematics_config.to_bytes()].iter().flatten().copied().collect::<Vec<u8>>(),
			None => vec![0]
		}[..]);
		bytes
	}

	/// Get the byte size of this type.
	fn byte_size() -> usize {
		0
	}
}