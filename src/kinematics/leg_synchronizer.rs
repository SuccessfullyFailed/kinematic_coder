use super::robot_skeleton::{ RobotSkeleton, RobotSkeletonLeg };
use glass_panel::tridimensional::model::VertexMath;
use std::f32::consts::PI;
use std::time::Instant;


static mut STATIC_SYNCHRONIZER_INSTANCE:Option<LegSynchronizer> = None;
pub struct LegSynchronizer {
	last_update:Instant,
	progress:f32,
	smallest_step:f32,
	leg_timing_offsets:Vec<(usize, f32)>,

	speed:f32,
	strafe:f32
}
impl LegSynchronizer {

	/* CONSTRUCTOR METHODS */

	/// Create a new synchronizer.
	pub fn create() {

		// Parse skeleton.
		let skeleton:&RobotSkeleton = RobotSkeleton::get();
		let skeleton_legs:Vec<(usize, &RobotSkeletonLeg)> = skeleton.legs_indexed();

		// Find the smallest step the robot can make.
		let smallest_step:f32 = skeleton_legs.iter().map(|(_, leg)| *leg.step_distance()).reduce(|a, b| a.min(b)).unwrap();

		// Create the synchronizer.
		let leg_synchronizer = LegSynchronizer {
			last_update: Instant::now(),
			progress: 0.0,
			smallest_step,
			leg_timing_offsets: Self::find_best_delays(&skeleton_legs),
			
			speed: 100.0,
			strafe: 0.0
		};

		// Move to static instance.
		unsafe { STATIC_SYNCHRONIZER_INSTANCE = Some(leg_synchronizer); }
	}

	/// Get the static config.
	pub fn get() -> &'static mut LegSynchronizer {
		match unsafe { &mut STATIC_SYNCHRONIZER_INSTANCE } {
			Some(inst) => inst,
			None => {
				LegSynchronizer::create();
				LegSynchronizer::get()
			}
		}
	}

	/// Find the best set of delays based on a skeleton.
	fn find_best_delays(skeleton_legs:&[(usize, &RobotSkeletonLeg)]) -> Vec<(usize, f32)> {
		
		// Calculate two groups of legs that are stable without another.
		let min_legs_on_ground:usize = 3;
		let delay_precision:f32 = 0.25;
		let mut leg_delays:Vec<f32> = vec![0.0; skeleton_legs.len()];

		// Find the set of delays with the biggest field of stability.
		let mut best_angledata_set:(f32, Vec<f32>) = (f32::MAX, vec![0.0; skeleton_legs.len()]);
		let mut reached_end:bool = false;
		while !reached_end {

			// Increment delays in brute-force pattern.
			for depth in 0..leg_delays.len() {
				leg_delays[depth] += delay_precision;
				if leg_delays[depth] >= 1.0 {
					leg_delays[depth] -= 1.0;
					let next_depth:usize = depth + 1;
					if next_depth >= leg_delays.len() {
						reached_end = true;
						break;
					}
				} else {
					break;
				}
			}
			if reached_end { break; }

			// Check if the robot can stand at each moment in time.
			let mut valid_set:bool = true;
			let mut stability_positions_over_time:Vec<[f32; 3]> = Vec::new();
			for progress_index in 0..(1.0 / delay_precision) as usize {
				let progress:f32 = progress_index as f32 * delay_precision;

				// Get the position of each leg.
				let leg_positions:Vec<[f32; 3]> = skeleton_legs.iter().map(|(_, leg)| leg.position().displaced(&Self::target_offset_for(leg, progress, *leg.step_distance()))).collect::<Vec<[f32; 3]>>();
				let legs_on_ground:Vec<usize> = (0..skeleton_legs.len()).filter(|leg_index| (progress + leg_delays[*leg_index]) % 1.0 >= 0.5).collect::<Vec<usize>>();
				if legs_on_ground.len() >= min_legs_on_ground {
				
					// Sort leg indexes by angle relative to robot body.
					let mut leg_angles:Vec<f32> = legs_on_ground.iter().map(|index| leg_positions[*index]).map(|position| position[0].atan2(position[1])).collect::<Vec<f32>>();
					leg_angles.sort_by(|a, b| (a.partial_cmp(b).unwrap()));

					// Keep track of the average circumference for this set of delays.
					let ground_leg_positions:Vec<[f32; 3]> = legs_on_ground.iter().map(|leg_index| leg_positions[*leg_index]).collect::<Vec<[f32; 3]>>();
					let stability_position:[f32; 3] =  (0..3).map(|axis| ground_leg_positions.iter().map(|pos| pos[axis]).sum::<f32>() / ground_leg_positions.len() as f32).collect::<Vec<f32>>().try_into().unwrap();
					stability_positions_over_time.push(stability_position);
				}

				// If there are too few legs on the ground, invalidate this delay set.
				else {
					valid_set = false;
					break;
				}
			}

			// If this set of delays seems valid, check if the current circumference is better than the currently kept best.
			if valid_set {
				let stability_offsets:Vec<f32> = stability_positions_over_time.iter().map(|p| (p[0].abs().powi(2) + p[1].abs().powi(2) + p[2].abs().powi(2)).sqrt()).collect::<Vec<f32>>();
				let max_stability_offset:f32 = stability_offsets.iter().copied().reduce(|a, b| a.max(b)).unwrap();
				if max_stability_offset < best_angledata_set.0 {
					best_angledata_set = (max_stability_offset, leg_delays.clone());
				}
			}
		}

		// Return an index and delay for each leg.
		skeleton_legs.iter().map(|(index, _)| (*index, best_angledata_set.1[*index])).collect::<Vec<(usize, f32)>>()
	}



	/* USAGE METHODS */

	/// Get the realtime progress for each leg.
	pub fn realtime_progress(&mut self) -> Vec<(usize, f32)> {

		// Calculate progress increase.
		let time_passed:f32 = self.last_update.elapsed().as_millis() as f32 / 1000.0;
		let progress_passed:f32 = time_passed / 100.0 * self.speed;
		self.progress = (self.progress + progress_passed) % 1.0;
		self.last_update = Instant::now();

		// Return progress per leg.
		self.leg_timing_offsets.iter().map(|(leg_index, progress_offset)| (*leg_index, self.progress + progress_offset)).collect::<Vec<(usize, f32)>>()
	}

	/// Get the realtime target position for each leg relative to the start of the step.
	pub fn realtime_target_offsets(&mut self) -> Vec<(usize, [f32; 3])> {
		let mut target_positions:Vec<(usize, [f32; 3])> = Vec::new();
		for (leg_index, _) in RobotSkeleton::get().legs_indexed() {

			// Get this leg's skeleton and progress.
			if let Some(leg) = &RobotSkeleton::get().legs()[leg_index] {
				if let Some(progress) = self.realtime_progress().iter().find(|(index, _)| index == &leg_index).map(|(_, progress)| progress) {

					// Get the offset for this leg.
					target_positions.push((leg_index, Self::target_offset_for(leg, *progress, self.smallest_step)));
				}
			}
		}
		target_positions
	}

	/// Get a target position for a specific leg at a specific point in progress.
	fn target_offset_for(leg:&RobotSkeletonLeg, progress:f32, step_distance:f32) -> [f32; 3] {

		// Prepare variables.
		let step_height:f32 = *leg.step_height();

		// Find out location at this progress.
		let mut offset:[f32; 3] = [0.0; 3];
		offset[1] = (((progress + 0.25) * PI * 2.0).sin() * step_distance - step_distance) * -0.5;
		offset[2] = (progress * PI * 2.0).sin().max(0.0) * step_height * 0.5;

		// Return result.
		offset
	}



	/* PROPERTY GETTER METHODS */

	/// Return a reference to the smallest_step.
	pub fn smallest_step(&self) -> &f32 {
		&self.smallest_step
	}

	/// Return a reference to the speed.
	pub fn speed(&self) -> &f32 {
		&self.speed
	}

	/// Return a mutable reference to the speed.
	pub fn speed_mut(&mut self) -> &mut f32 {
		&mut self.speed
	}

	/// Return a reference to the strafe.
	pub fn strafe(&self) -> &f32 {
		&self.strafe
	}

	/// Return a mutable reference to the strafe.
	pub fn strafe_mut(&mut self) -> &mut f32 {
		&mut self.strafe
	}
}