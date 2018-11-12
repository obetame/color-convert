extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
	use color_convert::color::{Color};
	use color_convert::handles::rgb;
	use common::init_color;

	#[test]
	fn test_handle_rgb() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)", "rgba(81,89,12,50%)"];
		let rgb_result = vec!["0.3176,0.89,0.1,", "0.81,0.89,0.1,0.5,", "0.3176,0.349,0.0471,0.3,", "0.3176,0.349,0.0471,0.5,"];

		for n in 0..4 {
			let color = Color::init(rgb_vec[n]);
			let v = rgb::handle_rgb(&color).unwrap();

			let mut result = String::from("");
			for i in v {
				result.push_str(&i.to_string());
				result.push(',');
			}
			assert_eq!(rgb_result[n], result);
		}
	}

	#[test]
	fn test_rgb_tohex() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["cmyk(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", ],
			vec!["cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", ],
			vec!["cmyk(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", ],
		];

		let test_color = init_color(rgb_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(rgb_result[i][index], color.to_cmyk().unwrap());
//				print!("{:?}, ", color.to_cmyk().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_rgb_tohsl() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["hsl(103.47,79.79%,49.5%)", "HSL(103.47,79.79%,49.5%)", "hsl(103.47,79.79%,49.5%)", "hsla(103.47,79.79%,49.5%,1)", "HSL(103.47,79.79%,49.5%)", "HSLA(103.47,79.79%,49.5%,1)", "hsla(103.47,79.79%,49.5%,1)", "HSLA(103.47,79.79%,49.5%,1)", ],
			vec!["hsl(66.08,79.79%,49.5%)", "HSL(66.08,79.79%,49.5%)", "hsl(66.08,79.79%,49.5%)", "hsla(66.08,79.79%,49.5%,0.5)", "HSL(66.08,79.79%,49.5%)", "HSLA(66.08,79.79%,49.5%,0.5)", "hsla(66.08,79.79%,49.5%,0.5)", "HSLA(66.08,79.79%,49.5%,0.5)", ],
			vec!["hsl(66.24,76.21%,19.80%)", "HSL(66.24,76.21%,19.80%)", "hsl(66.24,76.21%,19.80%)", "hsla(66.24,76.21%,19.80%,0.3)", "HSL(66.24,76.21%,19.80%)", "HSLA(66.24,76.21%,19.80%,0.3)", "hsla(66.24,76.21%,19.80%,0.3)", "HSLA(66.24,76.21%,19.80%,0.3)", ],
		];

		let test_color = init_color(rgb_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(rgb_result[i][index], color.to_hsl().unwrap());
//				print!("{:?}, ", color.to_hsl().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_rgb_tocmykl() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["cmyk(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "CMYK(0.64,0,0.89,0.11)", ],
			vec!["cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", ],
			vec!["cmyk(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "CMYK(0.09,0,0.87,0.65)", ],
		];

		let test_color = init_color(rgb_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(rgb_result[i][index], color.to_cmyk().unwrap());
//				print!("{:?}, ", color.to_cmyk().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_rgb_tohsv() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["hsv(103.47,88.76%,89%)", "HSV(103.47,88.76%,89%)", "hsv(103.47,88.76%,89%)", "hsv(103.47,88.76%,89%)", "HSV(103.47,88.76%,89%)", "HSV(103.47,88.76%,89%)", "hsv(103.47,88.76%,89%)", "HSV(103.47,88.76%,89%)", ],
			vec!["hsv(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", "hsv(66.08,88.76%,89%)", "hsv(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", "hsv(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", ],
			vec!["hsv(66.24,86.5%,34.9%)", "HSV(66.24,86.5%,34.9%)", "hsv(66.24,86.5%,34.9%)", "hsv(66.24,86.5%,34.9%)", "HSV(66.24,86.5%,34.9%)", "HSV(66.24,86.5%,34.9%)", "hsv(66.24,86.5%,34.9%)", "HSV(66.24,86.5%,34.9%)", ],
		];

		let test_color = init_color(rgb_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(rgb_result[i][index], color.to_hsv().unwrap());
//				print!("{:?}, ", color.to_hsv().unwrap());
			}
//			println!("],");
		}
	}
}