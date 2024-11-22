use dynamic_data_storage::GeneralDataType;
use std::error::Error;
use super::LegConfig;



static mut STATIC_CONFIG_INSTANCE:Option<RobotConfig> = None;
pub struct RobotConfig {
	body:Option<String>,
	legs:Vec<Vec<LegConfig>>
}
impl RobotConfig {

	/* CREATING, LOADING AND SAVING METHODS */

	/// Create an empty config and store it in the static instance.
	pub fn create() {
		unsafe { STATIC_CONFIG_INSTANCE = Some(RobotConfig { body: None, legs: Vec::new() }) }
	}

	/// Create a config from the current project files.
	pub fn load() {
		unsafe {
			STATIC_CONFIG_INSTANCE = Some(
				match dynamic_data_storage::StorageManager::get_mut().get_value::<RobotConfig>("project_robot_config", "robot_config") {
					Ok(config) => config,
					Err(_) => RobotConfig { body: None, legs: Vec::new() }
				}
			);
			RobotConfig::get().update_ui();
		}
	}

	/// Save the robot config to the assigned file.
	pub fn save() {
		let _ = dynamic_data_storage::StorageManager::get_mut().set_value::<RobotConfig>("project_robot_config", "robot_config", RobotConfig::get());
	}



	/* GENERAL USAGE METHODS */

	/// Get the static config.
	pub fn get() -> &'static RobotConfig {
		match unsafe { &STATIC_CONFIG_INSTANCE } {
			Some(inst) => inst,
			None => {
				RobotConfig::create();
				RobotConfig::get()
			}
		}
	}

	/// Get the static config mutable.
	pub fn get_mut() -> &'static mut RobotConfig {
		match unsafe { &mut STATIC_CONFIG_INSTANCE } {
			Some(inst) => inst,
			None => {
				RobotConfig::create();
				RobotConfig::get_mut()
			}
		}
	}

	

	/* PROPERTY GETTER METHODS */

	/// Get the body configuration.
	pub fn body(&self) -> &Option<String> {
		&self.body
	}

	/// Get the body configuration.
	pub fn body_mut(&mut self) -> &mut Option<String> {
		&mut self.body
	}

	/// Get the legs' configuration.
	pub fn legs(&self) -> &Vec<Vec<LegConfig>> {
		&self.legs
	}

	/// Get the legs' configuration mutable.
	pub fn legs_mut(&mut self) -> &mut Vec<Vec<LegConfig>> {
		&mut self.legs
	}

	/// Get a joint.
	pub fn get_joint(&self, leg_index:usize, joint_index:usize) -> Option<&LegConfig> {
		if leg_index < self.legs.len() && joint_index < self.legs[leg_index].len() {
			Some(&self.legs[leg_index][joint_index])
		} else {
			None
		}
	}

	/// Get a joint mutable.
	pub fn get_joint_mut(&mut self, leg_index:usize, joint_index:usize) -> Option<&mut LegConfig> {
		if leg_index < self.legs.len() && joint_index < self.legs[leg_index].len() {
			Some(&mut self.legs[leg_index][joint_index])
		} else {
			None
		}
	}



	/* MODIFIER METHODS */

	/// Add a leg.
	pub fn add_leg(&mut self, path:&str) {
		self.add_joint(self.legs.len(), path);
	}

	/// Add a joint.
	pub fn add_joint(&mut self, leg_index:usize, path:&str) {
		while leg_index >= self.legs.len() {
			self.legs.push(Vec::new());
		}
		self.set_joint(leg_index, self.legs[leg_index].len(), path);
	}

	/// Set a joint.
	pub fn set_joint(&mut self, leg_index:usize, joint_index:usize, path:&str) {
		while leg_index >= self.legs.len() {
			self.legs.push(Vec::new());
		}
		while joint_index >= self.legs[leg_index].len() {
			self.legs[leg_index].push(LegConfig::empty());
		}
		*self.legs[leg_index][joint_index].obj_mut() = path.to_string();
		self.update_ui();
	}

	/// Remove a leg.
	pub fn remove_leg(&mut self, leg_index:usize) {
		if self.legs.len() > leg_index {
			self.legs.remove(leg_index);
		}
		self.update_ui();
	}

	/// Remove a leg.
	pub fn remove_joint(&mut self, leg_index:usize, joint_index:usize) {
		if self.legs.len() > leg_index && self.legs[leg_index].len() > joint_index {
			self.legs[leg_index].remove(joint_index);
			if self.legs[leg_index].is_empty() {
				self.legs.remove(leg_index);
			}
		}
		self.update_ui();
	}



	/* UI UPDATING METHODS */

	/// Update the main UI based on the new configuration.
	pub fn update_ui(&self) {

		// TODO: This does not really belong here.
		crate::ui::Window::get().update_robot_config_synchronized();
	}
}



use super::OBJ_PATH_MAX_LEN as OBJ_LEN;

impl GeneralDataType for RobotConfig {

	/// Create a value of the implemented type from these bytes while removing the bytes required from the bytes list. Useful for parsing more advances structs.
	fn from_bytes_consume(bytes:&mut Vec<u8>) -> Result<Self, Box<dyn Error>> {
		let body:Option<String> = if bytes.remove(0) == 0 { None } else { Some(String::from_bytes(&bytes.drain(..OBJ_LEN + 1).collect::<Vec<u8>>())?) };
		let mut legs:Vec<Vec<LegConfig>> = Vec::new();
		let leg_count:usize = u16::from_bytes(&bytes.drain(..2).collect::<Vec<u8>>())? as usize;
		for leg_index in 0..leg_count {
			legs.push(Vec::new());
			let joint_count:usize = u16::from_bytes(&bytes.drain(..2).collect::<Vec<u8>>())? as usize;
			for _ in 0..joint_count {
				legs[leg_index].push(LegConfig::from_bytes_consume(bytes)?);
			}
		}
		Ok(RobotConfig {
			body,
			legs
		})
	}

	/// Create a value of the implemented type from these bytes.
	fn from_bytes(bytes:&[u8]) -> Result<Self, Box<dyn Error>> {
		Self::from_bytes_consume(&mut bytes.to_vec())
	}

	/// Create a value of the implemented type from these bytes.
	fn from_bytes_inner(_:&[u8]) -> Result<Self, Box<dyn Error>> {
		panic!("Should not get LegConfig from bytes due to not having a set byte size, please use from_bytes_consume");
	}

	/// Create a list of bytes from the value.
	fn to_bytes(&self) -> Vec<u8> {

		// Validate obj path len.
		let obj_len:usize = self.body.as_ref().map(|body| body.len()).unwrap_or(0);
		if obj_len > OBJ_LEN {
			panic!("Could not store leg config, name '{}' is too long.", self.body.as_ref().unwrap());
		}

		// Create bytes.
		let mut bytes:Vec<u8> = Vec::new();
		bytes.extend_from_slice(&match &self.body() { Some(body) => [vec![1], body.to_bytes().to_vec(), vec![0u8; OBJ_LEN - obj_len]].iter().flatten().copied().collect::<Vec<u8>>(), None => vec![0] });
		bytes.extend_from_slice(&(self.legs.len() as u16).to_bytes());
		for leg in &self.legs {
			bytes.extend_from_slice(&(leg.len() as u16).to_bytes());
			for joint in leg {
				bytes.extend_from_slice(&joint.to_bytes());
			}
		}
		bytes
	}

	/// Get the byte size of this type.
	fn byte_size() -> usize {
		0
	}
}