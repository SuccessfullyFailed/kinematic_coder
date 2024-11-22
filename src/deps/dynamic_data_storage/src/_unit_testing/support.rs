use crate::STORAGE_FILE_EXTENSION;

// Gives each TestImage an ID to stop TestStoragePath from existing in the same file.
#[allow(dead_code)]
static mut TEST_STORAGE_INDEX:usize = 0;

// Struct that gives the path to a test storage and ensures any existing file will be removed before and after usage.
pub struct TestStoragePath {
	path:String
}
#[allow(dead_code)]
impl TestStoragePath {

	/// Create a TestImg instance.
	pub fn get() -> TestStoragePath {
		unsafe {
			TEST_STORAGE_INDEX += 1; // Prevents async tests to mess up access.
			let img:TestStoragePath = TestStoragePath { path: format!("storage_{}.{}", TEST_STORAGE_INDEX, STORAGE_FILE_EXTENSION) };
			img.remove();
			img
		}
	}

	/// Check if the file exists.
	pub fn exists(&self) -> bool {
		std::path::Path::new(&self.path).exists()
	}

	/// Remove the physical file if it exists.
	pub fn remove(&self) {
		if self.exists() {
			std::fs::remove_file(self.path()).expect("Could not remove test storage file");
		}
	}

	/// Get the path of the image.
	pub fn path(&self) -> String {
		String::from(&self.path)
	}
}
impl Drop for TestStoragePath {
	fn drop(&mut self) {
		self.remove();
	}
}