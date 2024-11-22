use rust_build_buddy::BuildBuddyContext;
use std::{ io::Write, fs::{ read_to_string, File } };

static AUTO_GENERATED_TAG:&str = "/* AUTOGENERATED */";
static mut DATA_TYPE_FILE:String = String::new();

pub fn main() {
	unsafe { DATA_TYPE_FILE = format!("{}/src/data_type.rs", std::env::current_dir().unwrap().to_string_lossy()); }
	let mut build_buddy:BuildBuddyContext = BuildBuddyContext::new();

	// Find out build script target functionality.
	let build_script_target:String = std::env::var("BUILD_SCRIPT_TARGETS").unwrap_or(String::from("all"));

	// If not targets, do not do anything.
	if build_script_target.contains("none") { return; }

	// Auto-update dependencies.
	build_buddy.auto_update_libs();

	// Automatically fix parts of the code.
	if build_script_target.contains("all") || build_script_target.contains("auto_build") {
		build_buddy.generate_auto_public();
		auto_implement_data_types();
	}

	// Run custom quality checks.
	if build_script_target.contains("all") || build_script_target.contains("quality_validation") {
		build_buddy.validate_unit_test_location(&|path:&str| path.contains("src/_unit_testing/"));
		build_buddy.validate_code_quality_all();
	}
}

/// Automatically update general datatypes in src/data_type.rs.
fn auto_implement_data_types() {
	let mut code_lines:Vec<String> = Vec::new();

	// Create datasets per type.
	let data_sets = vec![
		("UNSIGNED INT", "u$bit_size", vec![8, 16, 32, 64, 128]),
		("SIGNED INT", "i$bit_size", vec![8, 16, 32, 64, 128]),
		("FLOATING POINT NUMBER", "f$bit_size", vec![32, 64])
	];

	// Loop through datasets.
	for data_set in &data_sets {

		// Banner.
		code_lines.push(format!("/* {} METHODS */", data_set.0));
		code_lines.push(String::new());

		// Implementations.
		for bit_size in &data_set.2 {
			code_lines.push(generate_datatype_implementation(data_set.1, bit_size / 8));
		}

		// Spacing for next banner.
		code_lines.push(String::from("\n\n"));
	}

	// Update file.
	let current_contents:String = read_to_string(unsafe { &DATA_TYPE_FILE }).unwrap().replace('\r', "");
	let components:Vec<&str> = current_contents.split(AUTO_GENERATED_TAG).collect::<Vec<&str>>();
	let new_contents:String = format!("{}\n\n\n{}\n\n\n\n{}", components[0].trim(), AUTO_GENERATED_TAG, code_lines.join("\n"));
	if current_contents != new_contents {
		File::create(unsafe { &DATA_TYPE_FILE }).unwrap().write_all(new_contents.as_bytes()).expect("Could not write to datatype file");
	}
}

/// Generate the code for one datatype implementation.
fn generate_datatype_implementation(name:&str, byte_size:usize) -> String {
r#"impl GeneralDataType for $name {
	fn from_bytes_inner(bytes:&[u8]) -> Result<Self, Box<dyn Error>> { Ok($name::from_le_bytes([$from_implementation])) }
	fn to_bytes(&self) -> Vec<u8> { self.to_le_bytes().to_vec() }
	fn byte_size() -> usize { $byte_size }
}"#
	.replace("$name", name)
	.replace("$byte_size", &byte_size.to_string())
	.replace("$bit_size", &(byte_size * 8).to_string())
	.replace("$from_implementation", &(0..byte_size).map(|index| format!("bytes[{index}]")).collect::<Vec<String>>().join(", "))
}