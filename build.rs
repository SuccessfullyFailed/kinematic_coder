use rust_build_buddy::BuildBuddyContext;

pub fn main() {
	let mut build_buddy:BuildBuddyContext = BuildBuddyContext::new();

	// Find out build script target functionality.
	let build_script_target:String = std::env::var("BUILD_SCRIPT_TARGETS").unwrap_or(String::from("all"));

	// If not targets, do not do anything.
	if build_script_target.contains("none") { return; }

	// Automatically fix parts of the code.
	if build_script_target.contains("all") || build_script_target.contains("auto_build") {
		build_buddy.generate_auto_public();
		build_buddy.auto_update_libs();
	}

	// Run custom quality checks.
	if build_script_target.contains("all") || build_script_target.contains("quality_validation") {
		build_buddy.validate_unit_test_location(&|path:&str| path.contains("src/_unit_testing/"));
		build_buddy.validate_code_quality_all();
	}
}