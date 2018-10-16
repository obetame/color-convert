extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::config::Setting;
	use color_convert::handles::map;
	use color_convert::handles::hex;
//	use color_convert::utils;

	#[test]
	fn test_map_color_name() {
		let color = "Sienna";
		let to_color = map::map_color_name(&color);
		assert_eq!(to_color, "#A0522D");
	}

	#[test]
	fn test_map_color_name_upper() {
		let color = "sienna";
		let to_color = map::map_name_to_name(&color);
		assert_eq!(to_color, "Sienna");
	}

	#[test]
	fn test_map_hex() {
		let hex = "B";
		let _to_char = map::map_hex(&hex);
		assert_eq!(_to_char, 11);
	}

	#[test]
	// #[should_panic(expected = "map_rgb not match match_number value")]
	fn test_map_rgb() {
		let number = 6;
		let _to_char = map::map_rgb(&number); // will panic
	}
}