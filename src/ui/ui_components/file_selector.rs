static mut FILE_SELECTOR_OPENED:bool = false;
enum Action { SelectFile, SelectDir, SaveFile }

/// Let the user define a new file on their computer. Will freeze the thread until file has been selected.
pub fn save_file(start_dir:Option<&str>, filters:Vec<(&str, &[&str])>) -> Option<String> {
	await_file_dialog(start_dir, Action::SaveFile, filters)
}

/// Let the user select a file on their computer. Will freeze the thread until file has been selected.
pub fn select_file(start_dir:Option<&str>, filters:Vec<(&str, &[&str])>) -> Option<String> {
	await_file_dialog(start_dir, Action::SelectFile, filters)
}

/// Let the user select a directory on their computer. Will freeze the thread until file has been selected.
pub fn select_dir(start_dir:Option<&str>) -> Option<String> {
	await_file_dialog(start_dir, Action::SelectDir, Vec::new())
}

/// Build a file dialog and wait for the user to select something.
fn await_file_dialog(start_dir:Option<&str>, action:Action, filters:Vec<(&str, &[&str])>) -> Option<String> {
	use std::path::PathBuf;
	use rfd::FileDialog;

	// Ensure there is no active prompt.
	if unsafe { FILE_SELECTOR_OPENED } { return None; }

	// Prepare dialog.
	let mut file_dialog:FileDialog = FileDialog::new();
	if let Some(dir) = start_dir {
		let mut dir:String = dir.to_string();
		if !dir.contains(':') {
			dir = std::env::current_dir().map(|path| path.display().to_string().replace('\\', "/")).unwrap_or_default() + "/" + &dir;
		}
		#[cfg(target_os = "windows")]{
			dir = dir.replace('/', "\\");
		}
		file_dialog = file_dialog.set_directory(dir);
	}
	for (name, filters) in &filters {
		file_dialog = file_dialog.add_filter(name.to_string(), filters);
	}

	// Open dialog.
	unsafe { FILE_SELECTOR_OPENED = true; }
	let result:Option<PathBuf> = match action {
		Action::SelectFile => file_dialog.pick_file(),
		Action::SelectDir => file_dialog.pick_folder(),
		Action::SaveFile => file_dialog.save_file()
	};
	let selected_file:Option<String> = result.map(|path_buffer| path_buffer.as_path().display().to_string().replace('\\', "/"));
	unsafe { FILE_SELECTOR_OPENED = false; }

	// Return the result.
	selected_file
}