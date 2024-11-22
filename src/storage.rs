use crate::robot_configuration::RobotConfig;
use dynamic_data_storage::StorageManager;
use std::error::Error;



/* PROJECT CREATING AND LOADING METHODS */

/// Create and set the initial storage of the project.
pub fn init_storage() -> Result<(), Box<dyn Error>> {
	set_project_name_and_path(AUTO_SAVE_PROJECT_NAME);
	StorageManager::hard_reset();
	set_storage_manager_catagories()?;
	RobotConfig::create();
	Ok(())
}

/// Select a project based on its name or path.
pub fn select_project(project_source:&str) -> Result<(), Box<dyn Error>> {
	set_project_name_and_path(project_source);
	StorageManager::hard_reset();
	set_storage_manager_catagories()?;
	RobotConfig::load();
	Ok(())
}

/// Store the current project at a new name/path.
pub fn store_project_at(project_source:&str) -> Result<(), Box<dyn Error>> {
	set_project_name_and_path(project_source);
	set_storage_manager_catagories()?;
	RobotConfig::save();
	Ok(())
}

/// Set the new project name and path based on a name/path.
fn set_project_name_and_path(source:&str) {
	unsafe {
		if source.contains('/') {
			PROJECT_NAME = source.split('/').last().unwrap().to_string();
			PROJECT_PATH = source.to_string();
		} else {
			PROJECT_NAME = source.to_string();
			PROJECT_PATH = projects_dir() + "/" + source;
		}
	}
}

/// Set the categories for the storage manager.
fn set_storage_manager_catagories() -> Result<(), Box<dyn Error>> {
	StorageManager::get_mut().set_categories(vec![
		("user_settings", &user_settings_file()),
		("project_settings", &project_settings_file()),
		("project_robot_config", &project_robot_config_file())
	])
}



/* MAIN PATH BUILDER METHODS */

/// The root directory to store user settings and configuration.
pub fn user_storage_dir() -> String {
	"user_data".to_string()
}

/// The file to store user settings globally used across all projects.
pub fn user_settings_file() -> String {
	format!("{}/settings", user_storage_dir())
}

/// Get the general projects dir.
fn default_projects_dir() -> String {
	format!("{}/projects", user_storage_dir())
}

/// Get the active projects dir.
pub fn projects_dir() -> String {
	default_projects_dir()
}



/* PROJECT PATH BUILDER METHODS */

pub static AUTO_SAVE_PROJECT_NAME:&str = "_AutoSave";
static mut PROJECT_NAME:String = String::new();
static mut PROJECT_PATH:String = String::new();

/// The directory that contains sub-directories for all user projects.
pub fn project_name() -> &'static str {
	unsafe { &PROJECT_NAME }
}

/// The directory to store a specific project in.
pub fn project_dir() -> String {
	unsafe { PROJECT_PATH.to_string() }
}

/// The file for the settings of the project.
pub fn project_settings_file() -> String {
	format!("{}/settings", project_dir())
}

/// The file for the configuration of the robot.
pub fn project_robot_config_file() -> String {
	format!("{}/robot", project_dir())
}