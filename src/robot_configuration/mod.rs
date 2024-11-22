pub static OBJ_PATH_MAX_LEN:usize = 256;

// auto-publib 

mod kinematics_config;
mod leg_config;
mod motor_config;
mod robot_config;

pub use kinematics_config::*;
pub use leg_config::*;
pub use motor_config::*;
pub use robot_config::*;