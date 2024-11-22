use glass_panel::{ ListenerType, elements::Id, tridimensional::{ model::{ Mesh, VertexMath, materials::SimpleColorMaterial }, Entity, Scene }, Drawable };
use crate::robot_configuration::{ KinematicsConfig, LegConfig, MotorConfig, RobotConfig };
use std::{ error::Error, rc::Rc };
use super::super::Window;

impl Window {

	/// Create the main 3D scene of the UI.
	pub(crate) fn create_ui_main_scene(&self) -> Id {
		use glass_panel::tridimensional::lenses::OrthographicLens;

		let mut scene:Scene = Scene::new(self.setting("scene_width"), self.setting("window_height"), &OrthographicLens::new(), vec![]);

		// Move camera to logical location.
		scene.camera().set_position(self.scene_camera_position());
		scene.camera().set_rotation(self.scene_camera_rotation());

		// Add camera movement listeners.
		scene.add_listener("MainSceneCameraRotation", ListenerType::DragLeft, Rc::new(|mouse_offset, _, _, _| {
			
			// Calculate new rotation.
			let rotation:&mut [f32; 3] = Window::get().scene_camera_rotation_mut();
			rotation[0] = (rotation[0] + mouse_offset[1] as f32).max(-90.0).min(90.0);
			rotation[2] = (rotation[2] + mouse_offset[0] as f32) % 360.0;

			// Calculate new position.
			let new_position:[f32; 3] = [0.0, *Window::get().scene_camera_distance(), 0.0].rotated(&[rotation[0].to_radians(), 0.0, rotation[2].to_radians()], &None).negative();
			*Window::get().scene_camera_position_mut() = new_position;

			// Apply rotation and position to camera before rendering.
			Window::get().execute_post_listener(&||{ 
				let camera_rotation:[f32; 3] = *Window::get().scene_camera_rotation();
				let camera_position:[f32; 3] = *Window::get().scene_camera_position();
				let camera:&mut Entity = Window::get().get_scene_mut().camera();
				camera.set_rotation(&camera_rotation);
				camera.set_position(&camera_position);
			});
		}));

		// Return scene.
		Id::new("MainScene", vec![&scene])
	}


	
	/* ROBOT CONFIG DISPLAY METHODS */

	/// Update the robot components in the 3D scene after all listeners have finished.
	pub(crate) fn update_robot_config_in_scene_synchronized(&self) {
		Window::get().execute_post_listener(&||
			if let Err(error) = Window::get().update_robot_config_in_scene_now() {
				eprintln!("{error}");
			}
		);
	}

	/// Update the robot components in the 3D scene.
	pub(crate) fn update_robot_config_in_scene_now(&mut self) -> Result<(), Box<dyn Error>> {

		// Create local copies of the robot config to work around not borrowing self twice.
		let robot_config:&mut RobotConfig = RobotConfig::get_mut();
		let scene:&mut Scene = self.get_scene_mut();

		// Create / update body.
		if let Some(path) = robot_config.body() {
			match scene.entity_by_name_mut("RobotBody") {
				Some(body) => body.set_obj(path)?,
				None => scene.add_entity(Entity::from_obj("RobotBody", path)?)
			}
		} else {
			scene.remove_entity_by_name("RobotBody");
		}

		// Create / update legs.
		if let Some(body) = scene.entity_by_name_mut("RobotBody") {
			body.remove_children();

			// Build legs.
			for leg_index in 0..robot_config.legs().len() {
				let leg_config:&Vec<LegConfig> = &robot_config.legs()[leg_index];

				// Build joints for leg.
				for joint_index in 0..leg_config.len() {

					// Create the joint's entity.
					let joint:Entity = Self::create_joint_entity(body, leg_index, joint_index).unwrap();

					// Find parent to add the entity to.
					let mut parent:&mut Entity = body;
					if joint_index > 0 {
						let parent_joint_index:usize = joint_index - 1;
						parent = body.child_by_name_mut(&format!("RobotLeg{leg_index}Joint{parent_joint_index}")).unwrap();
					}

					// Add the entity to its parent.
					parent.add_child(joint);
				}
			}
		}

		scene.add_entity(Entity::new("DEBUG", Mesh::raw(vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [-10.0, 0.0, 0.0], [0.0, -10.0, 0.0], [0.0, 10.0, 0.0]], vec![[1,0,2],[3,0,4]])));

		Ok(())
	}



	/* ENTITY CREATION METHODS */

	/// Create an entity for a robot joint.
	fn create_joint_entity(body:&mut Entity, leg_index:usize, joint_index:usize) -> Option<Entity> {
		if let Some(joint_config) = RobotConfig::get().get_joint(leg_index, joint_index) {
			let joint_name:String = format!("RobotLeg{leg_index}Joint{joint_index}");

			// Create the pivot point for the leg.
			let mut pivot:Entity = Entity::new(&(joint_name.to_string() + "Pivot"), Mesh::raw(Vec::new(), Vec::new()));
			pivot.set_position(joint_config.position());
			pivot.set_rotation(joint_config.rotation());
			let mut motor:Entity = Entity::new(&(joint_name.to_string() + "Motor"), Mesh::raw(Vec::new(), Vec::new()));

			// Create an entity of the joint's obj and add it as a child to the pivot.
			if let Ok(mut joint) = Entity::from_obj(&joint_name, joint_config.obj()) {

				// If the joint has a motor, modify the pivot.
				if let Some(motor_config) = joint_config.motor() {
					Self::add_motor_display_to_pivot(&mut pivot, motor_config);
					
					// Keep mesh in place despite displacement of motor.
					let mut joint_offset:[f32; 3] = *motor_config.position();
					joint_offset = [-joint_offset[0], -joint_offset[1], -joint_offset[2]];
					joint.displace(&joint_offset);

					// Set rotation of motor based on the robot config.
					let mut rotation:[f32; 3] = [0.0; 3];
					rotation[*motor_config.rotation_axis() as usize] = *motor_config.current_rotation();
					motor.set_euler_rotation(&rotation);
				}
				motor.add_child(joint);

				// If the joint has kinematics configuration, add visuals for it.
				if let Some(kinematics_config) = joint_config.kinematics_config() {
					let endpoint_display:Entity = Self::create_kinematics_endpoint_display(leg_index, joint_index, kinematics_config);
					motor.add_child(endpoint_display);

					if let Some(child) = body.child_by_name_mut(&format!("RobotLeg{leg_index}Joint0Pivot")) {
						let step_display:Entity = Self::create_kinematics_step_display(leg_index, kinematics_config);
						child.add_child(step_display);
					}
				}
				
				// Return the entity.
				pivot.add_child(motor);
				return Some(pivot);
			}
		}
		None
	}

	/// Create an entity for a robot motor.
	fn add_motor_display_to_pivot(pivot:&mut Entity, motor_config:&MotorConfig) {

		// Size settings.
		let axis_display_dist:f32 = 25.0;
		let range_display_dist:f32 = 25.0;
		let end_hook_size:f32 = 12.0;
		let default_rotation_hook_size:f32 = 6.0;
		let end_hook_dist:f32 = range_display_dist - end_hook_size;

		// Calculate axis indexes.
		let rotation_axis:usize = (*motor_config.rotation_axis() % 3) as usize;
		let (local_x_axis, local_y_axis) = match rotation_axis {
			0 => (1, 2), //  X
			1 => (0, 2), //  Y
			2 => (0, 1), //  Z
			other => panic!("Cannot rotate robot over axis with index {other}")
		};

		// Create a mesh showing the axis of rotation.
		let mut axis_display_vertices:[[f32; 3]; 2] = [[0.0; 3]; 2];
		axis_display_vertices[0][rotation_axis] = -axis_display_dist;
		axis_display_vertices[1][rotation_axis] = axis_display_dist;
		let mut mesh:Mesh = Mesh::raw(vec![axis_display_vertices[0], [0.0; 3], axis_display_vertices[1]], vec![[0, 1, 2]]);

		// Add a mesh showing the range of rotation.
		let rotations_to_display:Vec<f32> = (motor_config.rotation_range()[0] as i16..motor_config.rotation_range()[1] as i16).map(|degrees| (degrees as f32).to_radians()).collect::<Vec<f32>>();
		if rotations_to_display.len() > 2 {

			// Create arch.
			let mut segment_coordinates:Vec<[f32; 2]> = rotations_to_display.iter().map(|radian| [radian.sin() * range_display_dist, radian.cos() * range_display_dist]).collect::<Vec<[f32; 2]>>();
			let mut segments_faces:Vec<[usize; 3]> = (0..rotations_to_display.len() - 3).map(|index| [index, index + 1, index + 2]).collect::<Vec<[usize; 3]>>();

			// Add end hooks to arch.
			let last_vertex_index:usize = rotations_to_display.len() - 1;
			segment_coordinates.push([rotations_to_display[0].sin() * end_hook_dist, rotations_to_display[0].cos() * end_hook_dist]);
			segment_coordinates.push([rotations_to_display[last_vertex_index].sin() * end_hook_dist, rotations_to_display[last_vertex_index].cos() * end_hook_dist]);
			segments_faces.push([0, 0, last_vertex_index + 1]);
			segments_faces.push([last_vertex_index, last_vertex_index, last_vertex_index + 2]);

			// Add default rotation hook to arch.
			let default_rotation_distances:[f32; 2] = [range_display_dist - default_rotation_hook_size, range_display_dist + default_rotation_hook_size];
			segment_coordinates.push([motor_config.default_rotation().to_radians().sin() * default_rotation_distances[0], motor_config.default_rotation().to_radians().cos() * default_rotation_distances[0]]);
			segment_coordinates.push([motor_config.default_rotation().to_radians().sin() * default_rotation_distances[1], motor_config.default_rotation().to_radians().cos() * default_rotation_distances[1]]);
			segments_faces.push([last_vertex_index + 3, last_vertex_index + 3, last_vertex_index + 4]);

			// Map 2D coordinates to 3D vertices.
			let segment_vertices:Vec<[f32; 3]> = segment_coordinates.iter().map(|coordinate| {
				let mut vertex:[f32; 3] = [0.0; 3];
				vertex[local_x_axis] = coordinate[0];
				vertex[local_y_axis] = coordinate[1];
				vertex
			}).collect::<Vec<[f32; 3]>>();

			// Create mesh.
			let mut range_display_mesh:Mesh = Mesh::raw(segment_vertices, segments_faces);
			if rotation_axis == 0 { range_display_mesh.rotate(&[90.0, 0.0, 0.0], &None); } // X rotations should be rotated to have 0 degrees be forwards.
			mesh.combine_with(&range_display_mesh);
		}

		// Set a specific material to the mesh.
		// TODO: Use material manager later.
		mesh.set_material(&SimpleColorMaterial::new(0, 0xFF0000FF, 0));

		// Calculate the complete rotation of the pivot.
		let mut pivot_rotation:[f32; 3] = [0.0; 3];
		pivot_rotation[rotation_axis] = *motor_config.current_rotation();
		if rotation_axis == 1 {
			pivot_rotation[rotation_axis] = -pivot_rotation[rotation_axis];
		}

		// Set the new mesh to the pivot.
		pivot.set_mesh(mesh);

		// Set the position and rotation of the pivot point.
		let mut pivot_displacement:[f32; 3] = *motor_config.position();
		let mut motor_rotation:[f32; 3] = [0.0; 3];
		motor_rotation[*motor_config.rotation_axis() as usize] = motor_config.current_rotation().to_radians();
		pivot_displacement.rotate(&motor_rotation, &None);
		pivot.displace(&pivot_displacement);

		// Adapt position of the children to the position of the pivot point.
		let children_displacement:[f32; 3] = motor_config.position().iter().map(|axis_value| -axis_value).collect::<Vec<f32>>().try_into().unwrap();
		for child in pivot.children_mut() {
			child.displace(&children_displacement);
		}
	}

	/// Create an entity for a kinematics endpoint for a joint.
	fn create_kinematics_endpoint_display(leg_index:usize, joint_index:usize, kinematics_config:&KinematicsConfig) -> Entity {

		// Size settings.
		let mesh_size:f32 = 10.0;

		// Create a mesh.
		let mut vertices:Vec<[f32; 3]> = vec![[0.0; 3]];
		let mut faces:Vec<[usize; 3]> = Vec::new();
		for axis in 0..3 {
			let mut start:[f32; 3] = [0.0; 3];
			let mut end:[f32; 3] = [0.0; 3];
			start[axis] = -mesh_size;
			end[axis] = mesh_size;
			vertices.push(start);
			vertices.push(end);
			faces.push([0, 1 + axis * 2, 2 + axis * 2]);
		}
		let mut mesh:Mesh = Mesh::raw(vertices, faces);
		
		// Set a specific material to the mesh.
		// TODO: Use material manager later.
		mesh.set_material(&SimpleColorMaterial::new(0, 0xFF00FF00, 0));

		// Create the entity.
		let mut entity:Entity = Entity::new(&format!("RobotLeg{leg_index}Joint{joint_index}KinematicsEndpoint"), mesh);
		entity.set_position(kinematics_config.leg_endpoint());
		entity
	}

	/// Create an entity for a kinematics endpoint for a joint.
	fn create_kinematics_step_display(leg_index:usize, kinematics_config:&KinematicsConfig) -> Entity {
		use crate::kinematics::LegSynchronizer;
		
		// Get required arguments.
		let distance:f32 = *kinematics_config.step_distance();
		let radius:f32 = distance / 2.0;
		let height:f32 = *kinematics_config.step_height();
		let height_multiplier:f32 = 1.0 / distance * height;
		let strafe_angle:f32 = *LegSynchronizer::get().strafe();

		// Create the arch mesh.
		let arch_vertices:Vec<[f32; 3]> = (-90..90).map(|rotation| (rotation as f32).to_radians()).map(|rotation| [0.0, radius + radius * rotation.sin(), radius * rotation.cos() * height_multiplier]).collect::<Vec<[f32; 3]>>();
		let arch_faces:Vec<[usize; 3]> = (0..arch_vertices.len() - 1).map(|index| [index, index, index + 1]).collect::<Vec<[usize; 3]>>();
		let mut arch_mesh:Mesh = Mesh::raw(arch_vertices, arch_faces);

		// Set a specific materials to the meshes.
		// TODO: Use material manager later.
		arch_mesh.set_material(&SimpleColorMaterial::new(0, 0xFF00AA88, 0));

		// Create the entity.
		let mut entity:Entity = Entity::new(&format!("RobotLeg{leg_index}KinematicsStep"), arch_mesh);
		entity.set_position(kinematics_config.step_position());
		entity.set_rotation(&[0.0, 0.0, strafe_angle]);
		entity
	}



	/* ENTITY UPDATE METHODS */
	
	/// Rotate all joints to their current rotation in the scene.
	pub fn update_scene_motors(&self) {

		// Loop through joints with motors.
		for (leg_index, leg) in RobotConfig::get().legs().iter().enumerate() {
			for (joint_index, joint) in leg.iter().enumerate() {
				if joint.motor().is_some() {
					self.update_scene_motor(leg_index, joint_index)
				}
			}
		}

	}

	/// Rotate a joint to its current rotation in the scene.
	pub fn update_scene_motor(&self, leg_index:usize, joint_index:usize) {

		// Get motor config.
		if let Some(joint_config) = RobotConfig::get_mut().get_joint_mut(leg_index, joint_index) {
			if let Some(motor) = joint_config.motor() {

				// Get motor entity.
				if let Some(motor_entity) = Window::get().get_scene_mut().entity_by_name_mut(&format!("RobotLeg{leg_index}Joint{joint_index}Motor")) {

					// Set rotation.
					let mut rotation:[f32; 3] = [0.0; 3];
					rotation[*motor.rotation_axis() as usize] = *motor.current_rotation();
					motor_entity.set_euler_rotation(&rotation);
				}
			}
		}
	}
}