#[cfg(test)]
mod test {
	use crate::DrawableDataSettingDataType;

	#[test]
	fn drawable_data_types_test_all() {

		// Unsigned integers.
		assert_eq!(28u8, u8::from_bytes(&28u8.to_bytes()[..]).expect("Could not convert bytes to u8"));
		assert_eq!(298u16, u16::from_bytes(&298u16.to_bytes()[..]).expect("Could not convert bytes to u16"));
		assert_eq!(29865u32, u32::from_bytes(&29865u32.to_bytes()[..]).expect("Could not convert bytes to u32"));
		assert_eq!(29865u64, u64::from_bytes(&29865u64.to_bytes()[..]).expect("Could not convert bytes to u64"));
		assert_eq!(29865usize, usize::from_bytes(&29865usize.to_bytes()[..]).expect("Could not convert bytes to usize"));

		// Signed integers.
		assert_eq!(28i8, i8::from_bytes(&28i8.to_bytes()[..]).expect("Could not convert bytes to i8"));
		assert_eq!(298i16, i16::from_bytes(&298i16.to_bytes()[..]).expect("Could not convert bytes to i16"));
		assert_eq!(29865i32, i32::from_bytes(&29865i32.to_bytes()[..]).expect("Could not convert bytes to i32"));
		assert_eq!(29865i64, i64::from_bytes(&29865i64.to_bytes()[..]).expect("Could not convert bytes to i64"));
		assert_eq!(29865isize, isize::from_bytes(&29865isize.to_bytes()[..]).expect("Could not convert bytes to isize"));

		// Floating point numbers.
		assert_eq!(90.0f32, f32::from_bytes(&90.0f32.to_bytes()[..]).expect("Could not convert bytes to f32"));
		assert_eq!(128.0, f64::from_bytes(&128.0f64.to_bytes()[..]).expect("Could not convert bytes to f64"));

		// Miscellaneous.
		assert_eq!(String::from("Baguette"), String::from_bytes(&String::from("Baguette").to_bytes()[..]).expect("Could not convert bytes to String"));
	}
}