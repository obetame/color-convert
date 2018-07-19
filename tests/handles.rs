extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::handles::map_name;
	use color_convert::config::cc_config::Setting;
	use color_convert::handles::handle;
	use color_convert::utils::common;

	#[test]
	fn test_map_color_name() {
		let color = "Sienna";
		let to_color = map_name::map_color_name(&color);
		assert_eq!(to_color, "#A0522D");
	}

	#[test]
	fn test_map_color_name_uper() {
		let color = "sienna";
		let to_color = map_name::map_name_to_name(&color);
		assert_eq!(to_color, "Sienna");
	}

	#[test]
	fn test_map_hex() {
		let hex = "B";
		let _to_char = map_name::map_hex(&hex);
		assert_eq!(_to_char, 11);
	}

	#[test]
	// #[should_panic(expected = "map_rgb not match match_number value")]
	fn test_map_rgb() {
		let number = 6;
		let _to_char = map_name::map_rgb(&number); // will panic
	}

	#[test]
	fn test_hex_handle() {
		let hex = "#80ffffff";
		let hex1 = "#c8c8c8";
		let hex2 = "#ddd";

		let setting = Setting::new("rgb", false, true, false);
		let setting1 = Setting::new("rgb", false, false, false);

		let hex_vec: Vec<&str> = handle::handle_hex_value(&hex, &setting);
		let hex_vec1: Vec<&str> = handle::handle_hex_value(&hex1, &setting);
		let hex_vec2: Vec<&str> = handle::handle_hex_value(&hex2, &setting);
		let hex_vec3: Vec<&str> = handle::handle_hex_value(&hex, &setting1);

		assert_eq!(vec!["f", "f", "f", "f", "f", "f", "8", "0"], hex_vec);
		assert_eq!(vec!["c", "8", "c", "8", "c", "8"], hex_vec1);
		assert_eq!(vec!["d", "d", "d", "d", "d", "d"], hex_vec2);
		assert_eq!(vec![ "8", "0", "f", "f", "f", "f", "f", "f"], hex_vec3);
	}

	#[test]
	fn test_handel_alpha_to_hexadecimal() {
		let alpha = common::get_rgba_alpha_value("rgba(1,1,1,.5)");

		let hex_alpha = handle::handel_alpha_to_hexadecimal(alpha);

		assert_eq!(hex_alpha, String::from("80"));
	}
}