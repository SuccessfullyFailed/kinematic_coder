#[allow(dead_code)]
pub static TEST_CUBE_LOCATION:&str = "data/unittesting/default_cube.obj";
#[allow(dead_code)]
pub static TEST_TEXTURE_LOCATION:&str = "data/unittesting/default_texture.png";
#[allow(dead_code)]
pub static TEST_FONT_LOCATION:&str = "data/unittesting/Roboto.ttf";
#[allow(dead_code)]
pub static TEST_SCENE_SIZE:[usize; 2] = [200, 100];
#[allow(dead_code)]
pub static TEST_CUBE_SIZE:usize = 40;


// Gives each TestImage an ID to stop TestImages from existing in the same file.
#[allow(dead_code)]
static mut TEST_IMG_INDEX:usize = 0;

// Struct that gives the path to a test image and ensures any existing image will be removed before and after usage.
pub struct TestImage {
	path:String
}
#[allow(dead_code)]
impl TestImage {

	/// Create a TestImg instance.
	pub fn get() -> TestImage {
		unsafe {
			TEST_IMG_INDEX += 1; // Prevents async tests to mess up access.
			let img:TestImage = TestImage { path: format!("{}/test_img_{}.png", std::env::current_dir().unwrap().display(), TEST_IMG_INDEX) };
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
			std::fs::remove_file(self.path()).expect("Could not remove test img file");
		}
	}

	/// Get the path of the image.
	pub fn path(&self) -> String {
		String::from(&self.path)
	}
}
impl Drop for TestImage {
	fn drop(&mut self) {
		self.remove();
	}
}