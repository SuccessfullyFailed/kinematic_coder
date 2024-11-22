fn main() -> Result<(), Box<dyn std::error::Error>> {
	use kinematic_flow::{ storage, ui::Window, robot_configuration::RobotConfig };
	use std::{ thread::sleep, time::{ Duration, Instant } };
	use dynamic_data_storage::StorageManager;

	// Prepare project storage.
	if let Err(error) = storage::init_storage() {
		panic!("Could not initialize storage: {error}");
	}
	let strg:&mut StorageManager = StorageManager::get_mut();

	// Create the main window.
	let fps:f32 = strg.quick("main_window_framerate", 60.0);
	let window_size:[u16; 2] = [strg.quick("main_window_width", 800), strg.quick("main_window_height", 600)];
	let window:&mut Window = Window::create([window_size[0] as usize, window_size[1] as usize], fps);
	window.show_project_loading_tooltip();

	// Main loop.
	let interval:Duration = Duration::from_millis((1000.0 / fps).floor() as u64);
 	while window.exists() {
		let frame_start:Instant = Instant::now();

		// Update UI window.
		window.display();

		// Await interval.
		let processing_time:Duration = Instant::now() - frame_start;
		if processing_time < interval {
			sleep(interval - processing_time);
		}
	}

	// Save progress to auto-save.
	let _ = storage::store_project_at(storage::AUTO_SAVE_PROJECT_NAME);
	RobotConfig::save();

	Ok(())
}