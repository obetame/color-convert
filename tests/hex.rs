extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
	use color_convert::color::Color;
	use color_convert::handles::hex;
	use common::init_color;

	#[test]
	fn test_hex_handle() {
		let setting = Color::new("#80ffffff", false, true, false);
		let setting1 = Color::new("#c8c8c8", false, false, false);
		let setting2 = Color::new("#ddd", false, false, false);
		let setting4 = Color::new("#80ffffff", false, false, false);

//		let hex_vec: Result<Vec<&str>, String> = hex::handle_hex_value(&hex, &setting);
		let hex_vec = hex::handle_hex_value( &setting).unwrap();
		let hex_vec1 = hex::handle_hex_value( &setting1).unwrap();
		let hex_vec2 = hex::handle_hex_value( &setting2).unwrap();
		let hex_vec3 = hex::handle_hex_value(&setting4).unwrap();

		assert_eq!(vec!["f", "f", "f", "f", "f", "f", "8", "0"], hex_vec);
		assert_eq!(vec!["c", "8", "c", "8", "c", "8"], hex_vec1);
		assert_eq!(vec!["d", "d", "d", "d", "d", "d"], hex_vec2);
		assert_eq!(vec![ "8", "0", "f", "f", "f", "f", "f", "f"], hex_vec3);
	}

	#[test]
	fn test_hex2rgb() {
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			vec!["rgb(128,255,255)", "RGB(128,255,255)", "rgb(255,255,255)", "rgba(128,255,255,1.00)", "RGB(255,255,255)", "RGBA(128,255,255,1.00)", "rgba(255,255,255,0.50)", "RGBA(255,255,255,0.50)"],
			vec!["rgb(200,200,200)", "RGB(200,200,200)", "rgb(200,200,200)", "rgba(200,200,200,1)", "RGB(200,200,200)", "RGBA(200,200,200,1)", "rgba(200,200,200,1)", "RGBA(200,200,200,1)"],
			vec!["rgb(221,221,221)", "RGB(221,221,221)", "rgb(221,221,221)", "rgba(221,221,221,1)", "RGB(221,221,221)", "RGBA(221,221,221,1)", "rgba(221,221,221,1)", "RGBA(221,221,221,1)"],
			vec!["rgb(255,255,255)", "RGB(255,255,255)", "rgb(255,255,128)", "rgba(255,255,255,0.50)", "RGB(255,255,128)", "RGBA(255,255,255,0.50)", "rgba(255,255,128,1.00)", "RGBA(255,255,128,1.00)"]
		];

		let test_color = init_color(hex_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			println!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_rgb().unwrap());
//				print!("{:?}, ", color.to_rgb().unwrap());
			}
//			println!("]");
		}
	}

	#[test]
	fn test_hex2hex() {
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			vec!["#80ffff", "#80FFFF", "#ffffff", "#80ffffff", "#FFFFFF", "#80FFFFFF", "#80ffffff", "#80FFFFFF"],
			vec!["#c8c8c8", "#C8C8C8", "#c8c8c8", "#c8c8c8ff", "#C8C8C8", "#C8C8C8FF", "#ffc8c8c8", "#FFC8C8C8"],
			vec!["#dddddd", "#DDDDDD", "#dddddd", "#ddddddff", "#DDDDDD", "#DDDDDDFF", "#ffdddddd", "#FFDDDDDD"],
			vec!["#ffffff", "#FFFFFF", "#ffff80", "#ffffff80", "#FFFF80", "#FFFFFF80", "#ffffff80", "#FFFFFF80"]
		];

		let test_color = init_color(hex_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hex().unwrap());
//				print!("{:?}, ", color.to_hex().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hex2hsl() {
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			vec!["hsl(180.00,100%,75.1%)", "HSL(180.00,100%,75.1%)", "hsl(0,0,100%)", "hsla(180.00,100%,75.1%,1)", "HSL(0,0,100%)", "HSLA(180.00,100%,75.1%,1)", "hsla(0,0,100%,0.5)", "HSLA(0,0,100%,0.5)", ],
			vec!["hsl(0,0,78.43%)", "HSL(0,0,78.43%)", "hsl(0,0,78.43%)", "hsla(0,0,78.43%,1)", "HSL(0,0,78.43%)", "HSLA(0,0,78.43%,1)", "hsla(0,0,78.43%,1)", "HSLA(0,0,78.43%,1)", ],
			vec!["hsl(0,0,86.67%)", "HSL(0,0,86.67%)", "hsl(0,0,86.67%)", "hsla(0,0,86.67%,1)", "HSL(0,0,86.67%)", "HSLA(0,0,86.67%,1)", "hsla(0,0,86.67%,1)", "HSLA(0,0,86.67%,1)", ],
			vec!["hsl(0,0,100%)", "HSL(0,0,100%)", "hsl(60.00,100%,75.1%)", "hsla(0,0,100%,0.5)", "HSL(60.00,100%,75.1%)", "HSLA(0,0,100%,0.5)", "hsla(60.00,100%,75.1%,1)", "HSLA(60.00,100%,75.1%,1)", ],
		];

		let test_color = init_color(hex_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hsl().unwrap());
//				print!("{:?}, ", color.to_hsl().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hex2cmykl() {
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			vec!["cmyk(0.5,0,0,0)", "CMYK(0.5,0,0,0)", "cmyk(0,0,0,0)", "cmyk(0.5,0,0,0)", "CMYK(0,0,0,0)", "CMYK(0.5,0,0,0)", "cmyk(0,0,0,0)", "CMYK(0,0,0,0)", ],
			vec!["cmyk(0,0,0,0.22)", "CMYK(0,0,0,0.22)", "cmyk(0,0,0,0.22)", "cmyk(0.22,0.22,0.22,0)", "CMYK(0,0,0,0.22)", "CMYK(0.22,0.22,0.22,0)", "cmyk(0.22,0.22,0.22,0)", "CMYK(0.22,0.22,0.22,0)", ],
			vec!["cmyk(0,0,0,0.13)", "CMYK(0,0,0,0.13)", "cmyk(0,0,0,0.13)", "cmyk(0.13,0.13,0.13,0)", "CMYK(0,0,0,0.13)", "CMYK(0.13,0.13,0.13,0)", "cmyk(0.13,0.13,0.13,0)", "CMYK(0.13,0.13,0.13,0)", ],
			vec!["cmyk(0,0,0,0)", "CMYK(0,0,0,0)", "cmyk(0,0,0.5,0)", "cmyk(0,0,0,0)", "CMYK(0,0,0.5,0)", "CMYK(0,0,0,0)", "cmyk(0,0,0.5,0)", "CMYK(0,0,0.5,0)", ],
		];

		let test_color = init_color(hex_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_cmyk().unwrap());
//				print!("{:?}, ", color.to_cmyk().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hex2hsv() {
		let hex_vec = vec!["#80ffffff", "#c8c8c8", "#ddd", "#ffffff80"];
		let hex_result = vec![
			vec!["hsl(180.00,100%,75.1%)", "HSL(180.00,100%,75.1%)", "hsl(0,0,100%)", "hsla(180.00,100%,75.1%,1)", "HSL(0,0,100%)", "HSLA(180.00,100%,75.1%,1)", "hsla(0,0,100%,0.5)", "HSLA(0,0,100%,0.5)", ],
			vec!["hsl(0,0,78.43%)", "HSL(0,0,78.43%)", "hsl(0,0,78.43%)", "hsla(0,0,78.43%,1)", "HSL(0,0,78.43%)", "HSLA(0,0,78.43%,1)", "hsla(0,0,78.43%,1)", "HSLA(0,0,78.43%,1)", ],
			vec!["hsl(0,0,86.67%)", "HSL(0,0,86.67%)", "hsl(0,0,86.67%)", "hsla(0,0,86.67%,1)", "HSL(0,0,86.67%)", "HSLA(0,0,86.67%,1)", "hsla(0,0,86.67%,1)", "HSLA(0,0,86.67%,1)", ],
			vec!["hsl(0,0,100%)", "HSL(0,0,100%)", "hsl(60.00,100%,75.1%)", "hsla(0,0,100%,0.5)", "HSL(60.00,100%,75.1%)", "HSLA(0,0,100%,0.5)", "hsla(60.00,100%,75.1%,1)", "HSLA(60.00,100%,75.1%,1)", ],
		];

		let test_color = init_color(hex_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hsv().unwrap());
//				print!("{:?}, ", color.to_hsv().unwrap());
			}
//			println!("],");
		}
	}
}