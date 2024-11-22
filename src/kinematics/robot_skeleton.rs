use glass_panel::tridimensional::model::{ VertexMath, Mesh };
use crate::robot_configuration::RobotConfig;



static mut STATIC_SKELETON_INSTANCE:Option<RobotSkeleton> = None;
pub struct RobotSkeleton {
	legs:Vec<Option<RobotSkeletonLeg>>
}
impl RobotSkeleton {

	/* CONSTRUCTOR METHODS */

	/// Create a skeleton from the given robot config and store it in the static instance. This function might get complicated, prioritize code clarity over efficiency.
	pub fn create() {

		// Get current config.
		let robot_config:&RobotConfig = RobotConfig::get();

		// Loop through legs.
		let mut skeleton_legs:Vec<Option<RobotSkeletonLeg>> = Vec::new();
		for (leg_index, leg) in robot_config.legs().iter().enumerate() {
			let mut skeleton_leg:Option<RobotSkeletonLeg> = None;
			
			// Loop through joints.
			let mut position:[f32; 3] = [0.0; 3];
			let mut segment_start_joint:usize = 0;
			let mut segment_axis:u8 = 0;
			for (joint_index, joint) in leg.iter().enumerate() {

				// Modify position.
				position.displace(joint.position());

				// If joint has a motor, build a new segment from the start joint to this joint.
				if let Some(motor) = joint.motor() {
					match &mut skeleton_leg {
						None => skeleton_leg = Some(RobotSkeletonLeg::new(position.displaced(motor.position()))),
						Some(skeleton_leg) => skeleton_leg.add_segment(RobotSkeletonSegment::new(segment_axis, [segment_start_joint, joint_index], position.displaced(motor.position())))
					}

					// Mark this the start of the new segment.
					segment_start_joint = joint_index;
					position = motor.position().negative(); // The motor's position is relative to the mesh in the main structure, but not in the skeleton.
					segment_axis = *motor.rotation_axis();
				}

				// If this joint has kinematics configuration, add final changes to the leg.
				if let Some(kinematics_config) = joint.kinematics_config() {

					// Add last segment to leg.
					if let Some(skeleton_leg) = &mut skeleton_leg {
						let motor_offset:[f32; 3] = joint.motor().as_ref().map(|m| *m.position()).unwrap_or_default();
						let endpoint_offset:[f32; 3] = kinematics_config.leg_endpoint().displaced(&motor_offset);
						skeleton_leg.add_segment(RobotSkeletonSegment::new(segment_axis, [segment_start_joint, joint_index], position.displaced(&endpoint_offset)));
					}

					// Add endpoint to leg.
					if let Some(skeleton_leg) = &mut skeleton_leg {
						skeleton_leg.set_step_matrix(*kinematics_config.step_position(), *kinematics_config.step_distance(), *kinematics_config.step_height());
					}

					// Add the leg to the list. If no endpoint is found, it is not useful and should not be added.
					skeleton_legs.push(skeleton_leg);
					break;
				}
			}

			// Add empty space if no leg could be created.
			if skeleton_legs.len() <= leg_index {
				skeleton_legs.push(None);
			}
		}

		unsafe { STATIC_SKELETON_INSTANCE = Some(RobotSkeleton { legs: skeleton_legs }) }
	}

	/// Get the static config.
	pub fn get() -> &'static RobotSkeleton {
		match unsafe { &STATIC_SKELETON_INSTANCE } {
			Some(inst) => inst,
			None => {
				RobotSkeleton::create();
				RobotSkeleton::get()
			}
		}
	}



	/* PROPERTY GETTER METHODS */

	/// Get the legs.
	pub fn legs(&self) -> &Vec<Option<RobotSkeletonLeg>> {
		&self.legs
	}

	/// Get an indexed list of legs.
	pub fn legs_indexed(&self) -> Vec<(usize, &RobotSkeletonLeg)> {
		let mut list:Vec<(usize, &RobotSkeletonLeg)> = Vec::new();
		for (index, leg) in self.legs.iter().enumerate() {
			if let Some(leg) = leg {
				list.push((index, leg));
			}
		}
		list
	}



	/* USAGE METHODS */

	/// Draw the skeleton to a mesh. Mainly used for debugging.
	pub fn as_mesh(&self) -> Mesh {
		let mut vertices:Vec<[f32; 3]> = Vec::new();
		let mut faces:Vec<[usize; 3]> = Vec::new();

		// Lines from center to start of leg.
		vertices.push([0.0; 3]);
		for leg in self.legs.iter().flatten() {
			faces.push([0, 0, vertices.len()]);
			vertices.push(*leg.position());
		}

		// Lines per leg segment.
		for leg in self.legs.iter().flatten() {
			let mut position:[f32; 3] = *leg.position();
			for segment in leg.segments() {
				let new_position:[f32; 3] = (0..3).map(|axis| position[axis] + segment.endpoint()[axis]).collect::<Vec<f32>>().try_into().unwrap();
				faces.push([vertices.len(), vertices.len(), vertices.len() + 1]);
				vertices.push(position);
				vertices.push(new_position);
				position = new_position;
			}
		}

		// Create and return mesh.
		Mesh::raw(vertices, faces)
	}
}



pub struct RobotSkeletonLeg {
	position:[f32; 3],
	segments:Vec<RobotSkeletonSegment>,
	step_position:[f32; 3],
	step_distance:f32,
	step_height:f32
}
impl RobotSkeletonLeg {

	/* CONSTRUCTOR METHODS */

	/// Create a new segment.
	pub fn new(position:[f32; 3]) -> RobotSkeletonLeg {
		RobotSkeletonLeg {
			position,
			segments: Vec::new(),
			step_position: [0.0; 3],
			step_distance: 0.0,
			step_height: 0.0
		}
	}



	/* MODIFICATION METHODS */

	/// Add a new segment.
	pub fn add_segment(&mut self, segment:RobotSkeletonSegment) {
		self.segments.push(segment);
	}

	/// Set the step distance and height of the leg.
	pub fn set_step_matrix(&mut self, position:[f32; 3], distance:f32, height:f32) {
		self.step_position = position;
		self.step_distance = distance;
		self.step_height = height;
	}

	/// Get the total offset from the first segment to the endpoint.
	pub fn total_offset(&self) -> [f32; 3] {
		(0..3).map(|axis| 
			self.segments.iter().map(|segment| segment.endpoint()[axis]).sum()
		).collect::<Vec<f32>>().try_into().unwrap()
	}



	/* PROPERTY GETTER METHODS */

	/// Return a reference to the position.
	pub fn position(&self) -> &[f32; 3] {
		&self.position
	}

	/// Return a reference to the segments.
	pub fn segments(&self) -> &Vec<RobotSkeletonSegment> {
		&self.segments
	}

	/// Return a reference to the step_position.
	pub fn step_position(&self) -> &[f32; 3] {
		&self.step_position
	}

	/// Return a reference to the step_distance.
	pub fn step_distance(&self) -> &f32 {
		&self.step_distance
	}

	/// Return a reference to the step_height.
	pub fn step_height(&self) -> &f32 {
		&self.step_height
	}
}



pub struct RobotSkeletonSegment {
	axis:u8,
	joint_range:[usize; 2],
	endpoint:[f32; 3]
}
impl RobotSkeletonSegment {

	/// Create a new segment.
	pub fn new(axis:u8, joint_range:[usize; 2], endpoint:[f32; 3]) -> RobotSkeletonSegment {
		RobotSkeletonSegment {
			axis,
			joint_range,
			endpoint
		}
	}
	
	
	
	/* PROPERTY GETTER METHODS */

	/// Return a reference to the axis.
	pub fn axis(&self) -> &u8 {
		&self.axis
	}

	/// Return a reference to the joint_range.
	pub fn joint_range(&self) -> &[usize; 2] {
		&self.joint_range
	}

	/// Return a reference to the endpoint.
	pub fn endpoint(&self) -> &[f32; 3] {
		&self.endpoint
	}
}