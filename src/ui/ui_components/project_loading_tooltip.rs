use glass_panel::{ Drawable, ListenerType, elements::{ Rectangle, Col, Row, Id, VisibilityToggler, Centered, Border } };
use std::{rc::Rc, time::SystemTime};
use super::super::Window;
use crate::storage;

impl Window {
	
	/// Create the mesh replacement menu.
	pub(crate) fn create_project_loading_tooltip(&self) -> Id {
		let window_size:[usize; 2] = [self.setting::<usize>("window_width"), self.setting::<usize>("window_width")];

		Id::new("project_loading_tooltip", vec![
			&VisibilityToggler::new(false, vec![
				&Rectangle::new(window_size[0], window_size[1], self.setting::<u32>("color_tooltip_shadow"), vec![
					&Centered::new(true, true, window_size[0], window_size[1], vec![
						&Id::new("project_loading_tooltip_contents", vec![])
					])
				])
			])
		])
	}

	/// Show a mesh replacement menu at the given location.
	pub fn show_project_loading_tooltip(&mut self) {
		use dynamic_data_storage::STORAGE_FILE_EXTENSION;

		// Suppress main window listeners.
		self.window_mut().source_mut().child_by_name_mut("#main_ui").unwrap().suppress_listeners();

		// Find latest modified projects.
		let latest_projects:Vec<String> = Self::latest_modified_projects();

		// Calculate sizes.
		let vmin:usize = self.setting::<usize>("window_width").min(self.setting::<usize>("window_width"));
		let tile_size:usize = vmin / 8;
		let tile_border_size:usize = 5;
		let tile_bounds_size:usize = tile_size + tile_border_size * 2;

		// Create tiles.
		let auto_save:Rectangle = self.project_tile(tile_bounds_size * 2, tile_border_size, "Last session", None);
		let new:Rectangle = self.project_tile(tile_bounds_size, tile_border_size, "New", None);
		let open:Rectangle = self.project_tile(tile_bounds_size, tile_border_size, "Open", None);
		let latest:Vec<Rectangle> = (0..4).map(|index| {
			if latest_projects.len() > index {
				let title:String = latest_projects[index].split('/').last().unwrap().replace(&format!(".{}", STORAGE_FILE_EXTENSION), "");
				self.project_tile(tile_bounds_size, tile_border_size, &title, None)
			} else {
				Rectangle::new(0, 0, 0, vec![])
			}
		}).collect::<Vec<Rectangle>>();

		// Create full tooltip.
		let tiles = Row::new(vec![
			&auto_save,
			&Col::new(vec![
				&Row::new(vec![&new, &open, &latest[0]]),
				&Row::new(vec![&latest[1], &latest[2], &latest[3]])
			])
		]);
		let tiles_size = &tiles.position()[2..];
		let tooltip:Rectangle = Rectangle::new(tiles_size[0], tiles_size[1], 0, vec![
			&tiles
		]);

		// Set tooltip in window.
		self.window_mut().source_mut().child_by_name_mut("#project_loading_tooltip visibility_toggler").unwrap().data_set_mut().set_setting_value::<bool>("visible", true);
		self.window_mut().source_mut().child_by_name_mut("#project_loading_tooltip_contents").unwrap().set_children(vec![&tooltip]);

		// Add listener to remove tooltip when clicking background.
		self.window_mut().source_mut().child_by_name_mut("#project_loading_tooltip").unwrap().unsuppress_listeners();
		self.window_mut().source_mut().child_by_name_mut("#project_loading_tooltip").unwrap().add_listener("MeshReplacementToolTipRemoveToolTip", ListenerType::LeftDown, Rc::new(|_,_,_,_| Window::get().hide_project_loading_tooltip()));
	}

	/// Hide the mesh replacement tooltip.
	pub(crate) fn hide_project_loading_tooltip(&mut self) {

		// Schedule UI update.
		Window::get().execute_post_listener(&|| {
			let ui:&mut Window = Window::get();
			ui.window_mut().source_mut().child_by_name_mut("#project_loading_tooltip").unwrap().suppress_listeners();
			ui.window_mut().source_mut().child_by_name_mut("#project_loading_tooltip visibility_toggler").unwrap().data_set_mut().set_setting_value::<bool>("visible", false);
			ui.window_mut().source_mut().child_by_name_mut("#main_ui").unwrap().unsuppress_listeners();
		});
	}

	/// Create a single project tile.
	fn project_tile(&self, size:usize, border_size:usize, title:&str, thumbnail:Option<String>) -> Rectangle {
		use super::file_selector::select_dir;
		use std::thread;

		// Size settings.
		let color_background:u32 = self.setting("color_background");
		let color_tile:u32 = self.setting("color_foreground");
		let color_border:u32 = self.setting("color_highlight");
		let contents_size:usize = size - 2 * border_size;

		// Create tile.
		let mut tile:Rectangle = Rectangle::new(size, size, color_background, vec![
			&Border::new(border_size - 1, 0, vec![
				&Border::new(1, color_border, vec![
					&Rectangle::new(contents_size, contents_size, color_tile, vec![
						&Centered::new(thumbnail.is_none(), thumbnail.is_none(), contents_size, contents_size, vec![
							
							// Tile contents.
							&self.default_text(title)
						])
					])
				])
			])
		]);

		// Add listener.
		tile.data_set_mut().set_setting_value::<String>("project_name", title.to_string());
		tile.add_listener("OpenProject", ListenerType::LeftDown, Rc::new(|_, _, _, element_data| {
			let clicked_tile_title:String = element_data.get_setting_value::<String>("project_name").unwrap();
			match clicked_tile_title.as_str() {
				"New" => {}, // Automatically open fresh AutoSave project.
				"Open" => {
					thread::spawn(||{
						if let Some(project) = select_dir(Some(&storage::projects_dir())) {
							storage::select_project(&project).expect("Could not open project");
						}
					});
				},
				title => {
					let title: &str = if title == "Last session" { storage::AUTO_SAVE_PROJECT_NAME } else { title };
					storage::select_project(title).expect("Could not open project");
				}
			}
		}));

		// Return tile.
		tile
	}

	/// Create a list of available projects sorted by modification date from newest to oldest changes.
	fn latest_modified_projects() -> Vec<String> {
		use std::fs::{ read_dir, metadata };

		// Loop through project dirs.
		let projects_dir:String = storage::projects_dir();
		let mut latest_projects:Vec<(String, SystemTime)> = Vec::new();
		if let Ok(sub_dir_entries) = read_dir(projects_dir) {
			for sub_dir in sub_dir_entries.flatten() {
				if sub_dir.file_name() == "_AutoSave" { continue; }
				if let Ok(sub_dir_meta) = metadata(sub_dir.path()) {
					if !sub_dir_meta.is_dir() { continue; }

					// Loop through files in the directory.
					let mut latest_modified:SystemTime = sub_dir_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
					if let Ok(file_entries) = read_dir(sub_dir.path()) {
						for file in file_entries.flatten() {
							if let Ok(file_meta) = metadata(file.path()) {
								if file_meta.is_dir() { continue; }

								// Keep track of the last modified file.
								if let Ok(modification_time) = file_meta.modified() {
									if modification_time > latest_modified {
										latest_modified = modification_time;
									}
								}
							}
						}
					}

					// Keep list of available projects.
					latest_projects.push((sub_dir.path().display().to_string(), latest_modified));
				}
			}
		}

		// Sort projects by date.
		latest_projects.sort_by(|(_, time1), (_, time2)| time2.cmp(time1));

		latest_projects.iter().map(|project| project.0.replace('\\', "/")).collect::<Vec<String>>()
	}
}