extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::color::Color;
//	use color_convert::handles::map;
	use color_convert::handles::hex;
//	use color_convert::utils;

	#[test]
	fn test_hex_handle() {
		let hex = "#80ffffff";
		let hex1 = "#c8c8c8";
		let hex2 = "#ddd";

		let setting = Color::new("rgb", false, true, false);
		let setting1 = Color::new("rgb", false, false, false);

//		let hex_vec: Result<Vec<&str>, String> = hex::handle_hex_value(&hex, &setting);
		let hex_vec = hex::handle_hex_value(&hex, &setting).unwrap();
		let hex_vec1 = hex::handle_hex_value(&hex1, &setting).unwrap();
		let hex_vec2 = hex::handle_hex_value(&hex2, &setting).unwrap();
		let hex_vec3 = hex::handle_hex_value(&hex, &setting1).unwrap();

		assert_eq!(vec!["f", "f", "f", "f", "f", "f", "8", "0"], hex_vec);
		assert_eq!(vec!["c", "8", "c", "8", "c", "8"], hex_vec1);
		assert_eq!(vec!["d", "d", "d", "d", "d", "d"], hex_vec2);
		assert_eq!(vec![ "8", "0", "f", "f", "f", "f", "f", "f"], hex_vec3);
	}

	#[test]
	fn test_hex2rgb() {
		let hex = "#80ffffff";
		let hex1 = "#c8c8c8";
		let hex2 = "#ddd";

		let setting = Color::new("rgb", false, true, false);
		let setting1 = Color::new("rgb", false, false, false);

		let rgb1 = hex::hex2rgb(&hex, &setting).unwrap();
		let rgb2 = hex::hex2rgb(&hex1, &setting).unwrap();
		let rgb3 = hex::hex2rgb(&hex2, &setting1).unwrap();

		println!("{:?}", rgb1);
		println!("{:?}", rgb2);
		println!("{:?}", rgb3);
	}
}