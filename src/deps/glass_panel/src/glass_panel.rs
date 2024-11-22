use crate::Drawable;

static mut MOUSE_DOWN:[bool; 2] = [false; 2];

pub struct GlassPanel {
	position:[usize; 4],
	window:minifb::Window,
	source:Box<dyn Drawable>,
	post_listener_queue:Vec<&'static dyn Fn()>
}
impl GlassPanel {

	/* CONSTRUCTOR METHODS */

	/// Create a new instance from title, position and windowsettings.
	fn new(title:&str, position:[usize; 4], source:&dyn Drawable, window_settings:minifb::WindowOptions) -> GlassPanel {
		use crate::elements::Id;

		// Create the window.
		let minifb_window:minifb::Window = minifb::Window::new(title, position[2], position[3], window_settings).expect("Unable to create overlay");
		let mut win:GlassPanel = GlassPanel {
			position,
			window: minifb_window,
			source: Id::new("glass_panel_root", vec![source]).boxify(),
			post_listener_queue: Vec::new()
		};

		// Display the initial window.
		win.display();

		// Return window.
		win
	}

	/// Create a new window.
	pub fn window(title:&str, position:[usize; 4], source:&dyn Drawable) -> GlassPanel {
		GlassPanel::new(title, position, source, minifb::WindowOptions { borderless: false, transparency: false, none: false, resize: false, topmost: false, scale: minifb::Scale::X1, scale_mode: minifb::ScaleMode::AspectRatioStretch, title: true})
	}

	/// Set the maximum amount of fps for the window.
	pub fn set_max_fps(&mut self, fps:f32){
		self.window.limit_update_rate(Some(std::time::Duration::from_millis((1000.0 / fps) as u64)));
	}



	/* PROPERTY GETTER METHODS */

	/// Check if the window still exists.
	pub fn exist(&self) -> bool {
		self.window.is_open()
	}

	/// Get window position x.
	pub fn x(&self) -> usize {
		self.position[0]
	}

	/// Get window position y.
	pub fn y(&self) -> usize {
		self.position[1]
	}

	/// Get window position width.
	pub fn w(&self) -> usize {
		self.position[2]
	}

	/// Get window position height.
	pub fn h(&self) -> usize {
		self.position[3]
	}

	/// Get users' mouse position.
	pub fn mouse_pos(&self) -> Option<[usize; 2]> {
		if let Some(position) = self.window.get_mouse_pos(minifb::MouseMode::Pass) {
			if position.0 < 0.0 || position.1 < 0.0 || position.0 > self.w() as f32 || position.1 > self.h() as f32 {
				None
			} else {
				Some([position.0 as usize, position.1 as usize])
			}
		} else {
			None
		}
	}

	/// Check if the user has the left or right mouse button down.
	pub fn mouse_down(&self) -> [bool; 2] {
		use minifb::MouseButton::{ Left, Right };
		
		[self.window.get_mouse_down(Left), self.window.get_mouse_down(Right)]
	}



	/* DISPLAY METHODS */

	/// Refresh/display the window.
	pub fn display(&mut self){
		
		// Update listeners.
		if self.source.has_listeners() {
			if let Some(mouse_pos) = self.mouse_pos(){
				let mouse_down:[bool; 2] = self.mouse_down();
				let initial:[bool; 2] = unsafe { [mouse_down[0] != MOUSE_DOWN[0], mouse_down[1] != MOUSE_DOWN[1]] };
				unsafe { MOUSE_DOWN = mouse_down };
				self.source.update_listeners(&mouse_pos, &mouse_down, &initial);
			}
		}

		// Execute all actions scheduled after listeners.
		let listeners:Vec<&dyn Fn()> = self.post_listener_queue.drain(..).collect::<Vec<&dyn Fn()>>();
		for listener in &listeners {
			listener();
		}

		// Update UI.
		if self.source.update(&[0, 0]) {
			self.force_display();
		} else {
			self.window.update();
		}
	}

	/// Force Refresh/display the window.
	pub fn force_display(&mut self){
		let source_pos:[usize; 4] = self.source.position();
		let _ = self.window.update_with_buffer(&self.source.draw().data[..], source_pos[2], source_pos[3]);
	}



	/* POST-LISTENER METHODS */

	/// Add a function to the post-listener queue.
	pub fn execute_post_listener(&mut self, action:&'static dyn Fn()) {
		self.post_listener_queue.push(action);
	}



	/* SOURCE METHODS */

	/// Get the source of the window.
	pub fn source(&self) -> &dyn Drawable {
		&*self.source
	}

	/// Get the mutable source of the window.
	pub fn source_mut(&mut self) -> &mut dyn Drawable {
		&mut *self.source
	}
}