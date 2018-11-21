extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
	use common::init_color;

	#[test]
	fn test_hsv2rgb() {
		let hsv_vec = vec!["hsv(180,100%,50%)", "hsv(400,100%,50%)"];
		let test_result = vec![
			vec!["rgb(0,127.5,127.5)", "RGB(0,127.5,127.5)", "rgb(0,127.5,127.5)", "rgba(0,127.5,127.5,1)", "RGB(0,127.5,127.5)", "RGBA(0,127.5,127.5,1)", "rgba(0,127.5,127.5,1)", "RGBA(0,127.5,127.5,1)", ],
			vec!["rgb(127.5,85,0)", "RGB(127.5,85,0)", "rgb(127.5,85,0)", "rgba(127.5,85,0,1)", "RGB(127.5,85,0)", "RGBA(127.5,85,0,1)", "rgba(127.5,85,0,1)", "RGBA(127.5,85,0,1)", ],
		];

		let test_color = init_color(hsv_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(test_result[i][index], color.to_rgb().unwrap());
//				print!("{:?}, ", color.to_rgb().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsv2hex() {
		let hsv_vec = vec!["hsv(180,100%,50%)", "hsv(400,100%,50%)"];
		let test_result = vec![
			vec!["#007f7f", "#007F7F", "#007f7f", "#007f7fff", "#007F7F", "#007F7FFF", "#ff007f7f", "#FF007F7F", ],
			vec!["#7f5400", "#7F5400", "#7f5400", "#7f5400ff", "#7F5400", "#7F5400FF", "#ff7f5400", "#FF7F5400", ],
		];

		let test_color = init_color(hsv_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(test_result[i][index], color.to_hex().unwrap());
//				print!("{:?}, ", color.to_hex().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsv2cmyk() {
		let hsv_vec = vec!["hsv(180,100%,50%)", "hsv(400,100%,50%)"];
		let test_result = vec![
			vec!["cmyk(1,0,0,0.5)", "CMYK(1,0,0,0.5)", "cmyk(1,0,0,0.5)", "cmyk(1,0.5,0.5,0)", "CMYK(1,0,0,0.5)", "CMYK(1,0.5,0.5,0)", "cmyk(1,0.5,0.5,0)", "CMYK(1,0.5,0.5,0)", ],
			vec!["cmyk(0,0.33,1,0.5)", "CMYK(0,0.33,1,0.5)", "cmyk(0,0.33,1,0.5)", "cmyk(0.5,0.67,1,0)", "CMYK(0,0.33,1,0.5)", "CMYK(0.5,0.67,1,0)", "cmyk(0.5,0.67,1,0)", "CMYK(0.5,0.67,1,0)", ],
		];

		let test_color = init_color(hsv_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(test_result[i][index], color.to_cmyk().unwrap());
//				print!("{:?}, ", color.to_cmyk().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsv2hsl() {
		let hsv_vec = vec!["hsv(180,100%,50%)", "hsv(400,100%,50%)"];
		let test_result = vec![
			vec!["hsl(180.00,100%,25%)", "HSL(180.00,100%,25%)", "hsl(180.00,100%,25%)", "hsla(180.00,100%,25%,1)", "HSL(180.00,100%,25%)", "HSLA(180.00,100%,25%,1)", "hsla(180.00,100%,25%,1)", "HSLA(180.00,100%,25%,1)", ],
			vec!["hsl(40.00,100%,25%)", "HSL(40.00,100%,25%)", "hsl(40.00,100%,25%)", "hsla(40.00,100%,25%,1)", "HSL(40.00,100%,25%)", "HSLA(40.00,100%,25%,1)", "hsla(40.00,100%,25%,1)", "HSLA(40.00,100%,25%,1)", ],
		];

		let test_color = init_color(hsv_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(test_result[i][index], color.to_hsl().unwrap());
//				print!("{:?}, ", color.to_hsl().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsv2hsv() {
		let hsv_vec = vec!["hsv(180,100%,50%)", "hsv(400,100%,50%)"];
		let test_result = vec![
			vec!["hsv(180,100%,50%)", "HSV(180,100%,50%)", "hsv(180,100%,50%)", "hsv(180,100%,50%)", "HSV(180,100%,50%)", "HSV(180,100%,50%)", "hsv(180,100%,50%)", "HSV(180,100%,50%)", ],
			vec!["hsv(400,100%,50%)", "HSV(400,100%,50%)", "hsv(400,100%,50%)", "hsv(400,100%,50%)", "HSV(400,100%,50%)", "HSV(400,100%,50%)", "hsv(400,100%,50%)", "HSV(400,100%,50%)", ],
		];

		let test_color = init_color(hsv_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(test_result[i][index], color.to_hsv().unwrap());
//				print!("{:?}, ", color.to_hsv().unwrap());
			}
//			println!("],");
		}
	}
}