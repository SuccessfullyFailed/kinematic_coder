use glass_panel::{ elements::*, Drawable, DrawableDataSettingDataType, DrawableDataSettings, GlassPanel };



static mut STATIC_UI_INSTANCE:Option<Window> = None; // Allows modifications in listeners by providing static access.
pub struct Window {
	window:GlassPanel,
	settings:DrawableDataSettings,
	font:Font,

	selected_joint:Option<[usize; 2]>,
	menu_tab_names:Vec<String>,
	active_tab:usize,

	// Used to temporarily store camera modifications to apply after a render rather than during.
	scene_camera_distance:f32,
	scene_camera_position:[f32; 3],
	scene_camera_rotation:[f32; 3],

	realtime_kinematics:bool
}
impl Window {

	/* CONSTRUCTOR METHODS */

	/// Create the UI.
	pub fn create(window_size:[usize; 2], window_fps:f32) -> &'static mut Window {

		if let Some(ui) = unsafe { &mut STATIC_UI_INSTANCE } {
			return ui;
		}

		// Create the window.
		let mut window:GlassPanel = GlassPanel::window("3D renderer display", [0, 0, window_size[0], window_size[1]], &Rectangle::new(0, 0, 0, vec![]));
		window.set_max_fps(window_fps);

		// Size calculations.
		let toolbar_height:usize = 30;
		let menu_width:usize = (window_size[0] / 3).min(200);
		let menu_height:usize = window_size[1] - toolbar_height;
		let menu_content_margin:usize = 3;
		let menu_content_width:usize = menu_width - menu_content_margin * 2;
		let tab_selector_width:usize = (20).min(menu_width);
		let menu_properties_content_width:usize = menu_content_width - tab_selector_width;
		let menu_properties_input_group_border_size:usize = 4;
		let menu_properties_input_spacer_height:usize = 20;
		let menu_properties_input_group_content_width:usize = menu_properties_content_width - 2 * menu_properties_input_group_border_size;
		let menu_properties_input_border_width:usize = 2;
		let scene_width:usize = window_size[0] - menu_width - 1;
		let menu_robot_config_tree_height:usize = menu_height / 3;
		let menu_robot_properties_menu_height:usize = menu_height - menu_robot_config_tree_height;

		// Create struct.
		let mut ui:Window = Window {
			window,
			settings: DrawableDataSettings::new::<Vec<u8>>(vec![
				("window_width", window_size[0].to_bytes()),
				("window_height", window_size[1].to_bytes()),

				("toolbar_height", toolbar_height.to_bytes()),
				("menu_width", menu_width.to_bytes()),
				("menu_height", menu_height.to_bytes()),
				("menu_tab_selector_width", tab_selector_width.to_bytes()),
				("menu_properties_content_width", menu_properties_content_width.to_bytes()),
				("menu_properties_input_spacer_height", menu_properties_input_spacer_height.to_bytes()),
				("menu_properties_input_group_border_size", menu_properties_input_group_border_size.to_bytes()),
				("menu_properties_input_group_width", menu_properties_input_group_content_width.to_bytes()),
				("menu_properties_input_border_width", menu_properties_input_border_width.to_bytes()),
				("scene_width", scene_width.to_bytes()),
				("menu_content_margin", menu_content_margin.to_bytes()),
				("menu_content_width", menu_content_width.to_bytes()),
				("menu_robot_config_tree_height", menu_robot_config_tree_height.to_bytes()),
				("menu_robot_properties_menu_height", menu_robot_properties_menu_height.to_bytes()),

				("color_background", 0xFF222222_u32.to_bytes()),
				("color_foreground", 0xFF303030_u32.to_bytes()),
				("color_highlight", 0xFF404040_u32.to_bytes()),
				("color_detail", 0xFFFFAA00_u32.to_bytes()),
				("color_text", 0xFFFFFFFF_u32.to_bytes()),
				("color_scrollbar_outer", 0xFF222222_u32.to_bytes()),
				("color_scrollbar_inner", 0xFF404040_u32.to_bytes()),
				("color_tooltip_shadow", 0x88000000_u32.to_bytes()),

				("default_font_height", 16_usize.to_bytes())
			]),
			font: Font::new("resource/Roboto.ttf", 0.1).unwrap(),
			selected_joint: None,
			menu_tab_names: ["Rendering", "Displacement", "Motor", "Kinematics", "Controller", "Programming"].iter().map(|name| name.to_string()).collect::<Vec<String>>(),
			active_tab: 0,
			
			scene_camera_distance: 300.0,
			scene_camera_position: [200.0, -200.0, 100.0],
			scene_camera_rotation: [22.5, 0.0, -45.0],

			realtime_kinematics: false
		};

		// Populate and set as static.
		let ui_source = ui.create_ui_source(); // Leave type open to change in the return type of the function.
		ui.window.source_mut().set_children(vec![&ui_source]);
		ui.update_robot_config_now(); // Allows the creation methods to skip data remade in update function.
		ui.update_robot_properties_menu_now(); // Allows the creation methods to skip creating the tab and data.
		unsafe { STATIC_UI_INSTANCE = Some(ui); }

		// Return it.
		Self::get()
	}



	/* GENERAL USAGE METHODS */

	/// Check if it is possible to get the UI.
	pub fn instance_available() -> bool {
		unsafe { STATIC_UI_INSTANCE.is_some() }
	}

	/// Get the UI.
	pub fn get() -> &'static mut Window {
		match unsafe { &mut STATIC_UI_INSTANCE } {
			Some(inst) => inst,
			None => panic!("Could not get UI, UI does not seem to exist.")
		}
	}

	/// Wether or not the window still exists.
	pub fn exists(&self) -> bool {
		self.window.exist()
	}

	/// Update the window.
	pub fn display(&mut self) {
		self.window.display();
	}

	/// Schedule an action to happen after all listeners are finished.
	pub fn execute_post_listener(&mut self, action:&'static dyn Fn()) {
		self.window.execute_post_listener(action);
	}

	/// Get the name of the active tab.
	pub fn active_tab_name(&self) -> &str {
		&self.menu_tab_names[self.active_tab]
	}



	/* PROPERTY GETTER METHODS */

	/// Return a reference to the window.
	pub fn window(&self) -> &GlassPanel {
		&self.window
	}

	/// Return a mutable reference to the window.
	pub fn window_mut(&mut self) -> &mut GlassPanel {
		&mut self.window
	}

	/// Return a reference to the settings.
	pub fn settings(&self) -> &DrawableDataSettings {
		&self.settings
	}

	/// Return a mutable reference to the settings.
	pub fn settings_mut(&mut self) -> &mut DrawableDataSettings {
		&mut self.settings
	}

	/// Return a reference to the font.
	pub fn font(&self) -> &Font {
		&self.font
	}

	/// Return a mutable reference to the font.
	pub fn font_mut(&mut self) -> &mut Font {
		&mut self.font
	}

	/// Return a reference to the selected_joint.
	pub fn selected_joint(&self) -> &Option<[usize; 2]> {
		&self.selected_joint
	}

	/// Return a mutable reference to the selected_joint.
	pub fn selected_joint_mut(&mut self) -> &mut Option<[usize; 2]> {
		&mut self.selected_joint
	}

	/// Return a reference to the menu_tab_names.
	pub fn menu_tab_names(&self) -> &Vec<String> {
		&self.menu_tab_names
	}

	/// Return a mutable reference to the menu_tab_names.
	pub fn menu_tab_names_mut(&mut self) -> &mut Vec<String> {
		&mut self.menu_tab_names
	}

	/// Return a reference to the active_tab.
	pub fn active_tab(&self) -> &usize {
		&self.active_tab
	}
	
	/// Return a mutable reference to the active_tab.
	pub fn active_tab_mut(&mut self) -> &mut usize {
		&mut self.active_tab
	}


	
	/* MAIN SCENE PROPERTY GETTER METHODS */

	/// Return a reference to the scene_camera_distance.
	pub fn scene_camera_distance(&self) -> &f32 {
		&self.scene_camera_distance
	}

	/// Return a mutable reference to the scene_camera_distance.
	pub fn scene_camera_distance_mut(&mut self) -> &mut f32 {
		&mut self.scene_camera_distance
	}

	/// Return a reference to the scene_camera_position.
	pub fn scene_camera_position(&self) -> &[f32; 3] {
		&self.scene_camera_position
	}

	/// Return a mutable reference to the scene_camera_position.
	pub fn scene_camera_position_mut(&mut self) -> &mut [f32; 3] {
		&mut self.scene_camera_position
	}

	/// Return a reference to the scene_camera_rotation.
	pub fn scene_camera_rotation(&self) -> &[f32; 3] {
		&self.scene_camera_rotation
	}
	
	/// Return a mutable reference to the scene_camera_rotation.
	pub fn scene_camera_rotation_mut(&mut self) -> &mut [f32; 3] {
		&mut self.scene_camera_rotation
	}

	/// Return a reference to the realtime_kinematics.
	pub fn realtime_kinematics(&self) -> &bool {
		&self.realtime_kinematics
	}

	/// Return a mutable reference to the realtime_kinematics.
	pub fn realtime_kinematics_mut(&mut self) -> &mut bool {
		&mut self.realtime_kinematics
	}



	/* ELEMENT CREATOR HELPER METHODS */

	/// Get a specific setting.
	pub(super) fn setting<T:DrawableDataSettingDataType>(&self, name:&str) -> T {
		self.settings.get::<T>(name).unwrap()
	}

	/// Get a specific mutable element.
	pub(super) fn element_mut(&mut self, name:&str) -> &mut dyn Drawable {
		self.window.source_mut().child_by_name_mut(name).unwrap()
	}

	/// Create a text element.
	pub(super) fn default_text(&self, text:&str) -> Text {
		Text::new(&self.font, text, self.setting("default_font_height"), self.setting("color_text"))
	}

	/// Create a text element with a relative scale.
	pub(super) fn scaled_text(&self, text:&str, scale:f32) -> Text {
		Text::new(&self.font, text, (self.setting::<usize>("default_font_height") as f32 * scale) as usize, self.setting("color_text"))
	}

	/// Get a mutable reference to the 3D scene.
	pub(crate) fn get_scene_mut(&mut self) -> &mut Scene {
		self.window_mut().source_mut().child_by_name_mut("#MainScene tridimensional_scene").unwrap().as_scene().unwrap()
	}



	/* ELEMENT CREATOR METHODS */

	/// Create the main UI source.
	fn create_ui_source(&self) -> Rectangle {
		let window_size:[usize; 2] = [self.setting("window_width"), self.setting("window_height")];
		Rectangle::new(window_size[0], window_size[1], self.setting("color_background"), vec![

			// Main UI.
			&Id::new("main_ui", vec![
				&Col::new(vec![
					&self.create_toolbar(),
					&Row::new(vec![
						&self.create_ui_main_scene(),
						&Rectangle::new(1, window_size[0], self.setting("color_detail"), vec![]),
						&self.create_ui_main_menu()
					])
				])
			]),

			// Tooltips.
			&self.create_mesh_replacement_tooltip(),
			&self.create_project_loading_tooltip()
		])
	}

	/// Create the main menu of the UI.
	fn create_ui_main_menu(&self) -> Rectangle {
		Rectangle::new(self.setting("menu_width"), self.setting("window_height"), self.setting("color_background"), vec![
			&Border::new(self.setting("menu_content_margin"), 0, vec![

				// Top-down menu items.
				&Col::new(vec![
					&Id::new("robot_components_tree", vec![]), // Is built in update method.
					&self.create_properties_menu() // Is built in update method.
				])
			])
		])
	}
}