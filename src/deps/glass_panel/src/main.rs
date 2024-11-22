use glass_panel::{ elements::{ Id, Rectangle, Text, Font, Positioned }, tridimensional::{ lenses::OrthographicLens, model::{ materials::SimpleColorMaterial, Mesh }, Entity, Scene }, GlassPanel };
use std::{ thread::sleep, time::{ Duration, Instant } };


fn main() -> Result<(), Box<dyn std::error::Error>> {

	// Settings.
	let scene_size:[usize; 2] = [800, 600];
	let fps:f32 = 60.0;
	let main_font:Font = Font::new("data/unittesting/Roboto.ttf", 0.1)?;

	// Basic display of 3d scene.
	let cube_entity:Entity = Entity::from_obj("DisplayCube", "data/unittesting/default_cube.obj")?;
	let scene:Scene = Scene::new(scene_size[0], scene_size[1], &OrthographicLens::new(), vec![
		Entity::new("StackOfCubes", Mesh::empty()).with_children(vec![
			cube_entity.displaced(&[0.0; 3]),
			cube_entity.displaced(&[50.0, 0.0, 0.0]),
			cube_entity.displaced(&[25.0, 0.0, 40.0]).with_children(vec![
				Entity::new("MaterialDisplayCube", cube_entity.mesh().with_material(&SimpleColorMaterial::new(0xFFFF0000, 0xFF00FF00, 0x00000000))).displaced(&[20.0, 0.0, 20.0]).scaled(&[0.3; 3])
			])
		]).displaced(&[0.0, 120.0, 0.0])
	]);
	
	// Create window.
	let window_source:Rectangle = Rectangle::new(scene_size[0], scene_size[1], 0xFF222222, vec![
		&scene,
		&Positioned::new(5, 5, vec![
			&Id::new("FPS_counter", vec![
				&Text::new(&main_font, "No FPS data available", 30, 0xFFFF8800)
			])
		])
	]);
	let mut window:GlassPanel = GlassPanel::window("3D renderer display", [0, 0, scene_size[0], scene_size[1]], &window_source);
	window.set_max_fps(fps);

	// Main loop.
	let interval:Duration = Duration::from_millis((1000.0 / fps).floor() as u64);
 	while window.exist() {
		let frame_start:Instant = Instant::now();
		window.display();

		// Get the scene.
		if let Some(scene_element) = window.source_mut().child_by_name_mut("tridimensional_scene") {
			if let Ok(scene) = scene_element.as_scene() {

				// Rotate the stack of cubes.
				scene.entities_mut()[0].rotate(&[0.0, 0.0, 1.0]);
			}
		}

		// Await interval.
		let processing_time:Duration = Instant::now() - frame_start;
		if processing_time < interval {
			sleep(interval - processing_time);
		}

		// Update FPS counter.
		let fps:f32 = 1000.0 / (Instant::now() - frame_start).as_millis() as f32;
		if let Some(fps_element) = window.source_mut().child_by_name_mut("#FPS_counter text") {
			fps_element.data_set_mut().set_setting_value::<String>("text", format!("{}fps", fps.round()));
		}
	}

	Ok(())
}
