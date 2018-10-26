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
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			"RGB(128,255,255)", "rgb(255,255,255)", "rgba(128,255,255,1.00)",
			"RGB(200,200,200)", "rgb(200,200,200)", "rgba(200,200,200,1)",
			"RGB(221,221,221)", "rgb(221,221,221)", "rgba(221,221,221,1)",
			"RGB(255,255,255)", "rgb(255,255,128)", "rgba(255,255,255,0.50)"
		];

		for (index, color) in hex_vec.iter().enumerate() {
			for i in 0..3 {
				let mut color = Color::new(color, false, false, false);
				match i {
					0 => assert_eq!(color.upper(true).to_rgb().unwrap(), hex_result[index * 3]),
					1 => assert_eq!(color.android(true).to_rgb().unwrap(), hex_result[index * 3 + 1]),
					2 => assert_eq!(color.alpha(true).to_rgb().unwrap(), hex_result[index * 3 + 2]),
					_ => println!("noting")
				}
			}
		}
	}

	#[test]
	fn test_hex2hex() {
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			"#80FFFF", "#ffffff", "#80ffffff",
			"#C8C8C8", "#c8c8c8", "#c8c8c8ff",
			"#DDDDDD", "#dddddd", "#ddddddff",
			"#FFFFFF", "#ffff80", "#ffffff80"
		];

		for (index, color) in hex_vec.iter().enumerate() {
			for i in 0..3 {
				let mut color = Color::new(color, false, false, false);
				match i {
					0 => assert_eq!(color.upper(true).to_hex().unwrap(), hex_result[index * 3]),
					1 => assert_eq!(color.android(true).to_hex().unwrap(), hex_result[index * 3 + 1]),
					2 => assert_eq!(color.alpha(true).to_hex().unwrap(), hex_result[index * 3 + 2]),
					_ => println!("noting")
				}
			}
		}
	}
}