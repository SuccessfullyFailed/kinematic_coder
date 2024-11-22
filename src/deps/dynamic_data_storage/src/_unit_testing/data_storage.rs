/* As DataStorage is a SingleTon, make sure all tests run synchronically using #[serial] and hard reset the StorageManager before each test to make sure no data leaks into other tests. */

#[cfg(test)]
mod test {
	use crate::{StorageManager, _unit_testing::TestStoragePath};
	use serial_test::serial;

	static CAT_NAME:&str = "main";
	static VAR_NAME:&str = "test_var";
	
	

	#[test]
	#[serial]
	pub fn test_storage_store_and_load_memory() {

		// Create storage manager.
		StorageManager::hard_reset();
		let test_storage_path:TestStoragePath = TestStoragePath::get();
		let storage_manager:&mut StorageManager = StorageManager::get_mut();
		storage_manager.set_categories(vec![(CAT_NAME, &test_storage_path.path())]).unwrap();

		// Store value.
		let storage_value:u8 = 192;
		storage_manager.set_value::<u8>(CAT_NAME, VAR_NAME, &storage_value).unwrap();

		// Load value.
		let validation_value:u8 = storage_manager.get_value::<u8>(CAT_NAME, VAR_NAME).unwrap();

		// Compare.
		assert_eq!(storage_value, validation_value);
	}

	#[test]
	#[serial]
	pub fn test_storage_store_and_load_disk() {

		// Create storage manager.
		StorageManager::hard_reset();
		let test_storage_path:TestStoragePath = TestStoragePath::get();
		let path:String = test_storage_path.path();
		let storage_value:u8 = 192;

		// Store var.
		{
			let storage_manager:&mut StorageManager = StorageManager::get_mut();
			storage_manager.set_categories(vec![(CAT_NAME, &path)]).unwrap();
			storage_manager.set_value::<u8>(CAT_NAME, VAR_NAME, &storage_value).unwrap();
		}

		// Load var.
		{
			let storage_manager:&mut StorageManager = StorageManager::get_mut();
			storage_manager.add_category_from_file(&path).unwrap();
			let validation_value:u8 = storage_manager.get_value::<u8>(CAT_NAME, VAR_NAME).unwrap();

			assert_eq!(storage_value, validation_value);
		}
	}

	#[test]
	#[serial]
	#[should_panic]
	pub fn test_storage_nonexistent_category() {

		// Create storage manager.
		StorageManager::hard_reset();
		let test_storage_path:TestStoragePath = TestStoragePath::get();
		let storage_manager:&mut StorageManager = StorageManager::get_mut();
		storage_manager.set_categories(vec![(CAT_NAME, &test_storage_path.path())]).unwrap();

		// Store value.
		let value:u8 = 0;
		storage_manager.set_value::<u8>("FAKE_CAT_NAME", VAR_NAME, &value).unwrap();
	}

	#[test]
	#[serial]
	#[should_panic]
	pub fn test_storage_nonexistent_file() {
		StorageManager::hard_reset();
		let test_storage_path:TestStoragePath = TestStoragePath::get();
		let storage_manager:&mut StorageManager = StorageManager::get_mut();
		storage_manager.add_category_from_file(&test_storage_path.path()).unwrap();
	}

	#[test]
	#[serial]
	pub fn test_storage_multiple_instances() {
		StorageManager::hard_reset();
		let test_storage_path:TestStoragePath = TestStoragePath::get();

		// Store value.
		let storage_value:u8 = 192;
		{
			let storage_manager_a:&mut StorageManager = StorageManager::get_mut();
			storage_manager_a.set_categories(vec![(CAT_NAME, &test_storage_path.path())]).unwrap();
			storage_manager_a.set_value::<u8>(CAT_NAME, VAR_NAME, &storage_value).unwrap();
		}

		// Load value.
		{
			let storage_manager_b:&StorageManager = StorageManager::get();
			let validation_value:u8 = storage_manager_b.get_value::<u8>(CAT_NAME, VAR_NAME).unwrap();

			// Compare.
			assert_eq!(storage_value, validation_value);
		}
	}

	#[test]
	#[serial]
	pub fn test_storage_store_and_load_from_dir() {
		let categories:u16 = 2;
		let vars_per_category:u16 = 16;

		// Create storage manager.
		StorageManager::hard_reset();
		let storage_manager:&mut StorageManager = StorageManager::get_mut();
		
		// Create categories.
		let mut storage_paths:Vec<TestStoragePath> = Vec::new();
		for category_index in 0..categories {
			let category_name:String = format!("category_{category_index}");
			let test_storage_path:TestStoragePath = TestStoragePath::get();
			storage_manager.add_category(&category_name, &test_storage_path.path());
			storage_paths.push(test_storage_path);

			// Create variables.
			for variable_index in 0..vars_per_category {
				let var_name:String = format!("category_{category_index}_variable_{variable_index}");
				storage_manager.set_value::<u16>(&category_name, &var_name, &(variable_index * category_index)).unwrap();
			}
		}

		// Reset storage manager and read from dir.
		StorageManager::hard_reset();
		assert!(storage_manager.find_category("category_1").is_none());
		storage_manager.add_categories_from_dir("./", true);

		// Validate categories.
		for category_index in 0..categories {
			let category_name:String = format!("category_{category_index}");
			assert!(storage_manager.find_category(&category_name).is_some());

			// Validate variables.
			for variable_index in 0..vars_per_category {
				let var_name:String = format!("category_{category_index}_variable_{variable_index}");
				assert_eq!(storage_manager.get_value::<u16>(&category_name, &var_name).unwrap(), variable_index * category_index);
			}
		}
	}

	#[test]
	#[serial]
	pub fn test_storage_remove_file() {
		use std::path::Path;

		// Create StorageManager.
		StorageManager::hard_reset();
		let test_storage_path:TestStoragePath = TestStoragePath::get();
		let storage_manager:&mut StorageManager = StorageManager::get_mut();
		storage_manager.set_categories(vec![(CAT_NAME, &test_storage_path.path())]).unwrap();

		// Create and validate file.
		storage_manager.set_value::<u8>(CAT_NAME, VAR_NAME, &0).unwrap();
		assert!(Path::new(&test_storage_path.path()).exists());

		// Remove and validate file.
		storage_manager.remove_category(CAT_NAME).unwrap();
		assert!(!Path::new(&test_storage_path.path()).exists());
	}
}