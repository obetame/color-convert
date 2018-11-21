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
			let color = Color::new(rgb_vec[n]);
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
		let rgb_vec = vec!["rgb( 81% , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["#cee219", "#CEE219", "#cee219", "#cee219ff", "#CEE219", "#CEE219FF", "#ffcee219", "#FFCEE219", ],
			vec!["#cee219", "#CEE219", "#cee219", "#cee21980", "#CEE219", "#CEE21980", "#80cee219", "#80CEE219", ],
			vec!["#50580c", "#50580C", "#50580c", "#50580c4c", "#50580C", "#50580C4C", "#4c50580c", "#4C50580C", ],
		];

		let test_color = init_color(rgb_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(rgb_result[i][index], color.to_hex().unwrap());
//				print!("{:?}, ", color.to_hex().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_rgb_tohsl() {
		let rgb_vec = vec!["rgb( 81% , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["hsl(66.08,79.79%,49.5%)", "HSL(66.08,79.79%,49.5%)", "hsl(66.08,79.79%,49.5%)", "hsla(66.08,79.79%,49.5%,1)", "HSL(66.08,79.79%,49.5%)", "HSLA(66.08,79.79%,49.5%,1)", "hsla(66.08,79.79%,49.5%,1)", "HSLA(66.08,79.79%,49.5%,1)", ],
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
		let rgb_vec = vec!["rgb( 81% , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "CMYK(0.09,0,0.89,0.11)", ],
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
		let rgb_vec = vec!["rgb( 81% , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["hsv(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", "hsv(66.08,88.76%,89%)", "hsv(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", "hsv(66.08,88.76%,89%)", "HSV(66.08,88.76%,89%)", ],
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

	#[test]
	fn test_rgb_torgb() {
		let rgb_vec = vec!["rgb( 81% , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			vec!["rgb(81%,89%,10%)", "RGB(81%,89%,10%)", "rgb(81%,89%,10%)", "rgba(81%,89%,10%,1)", "RGB(81%,89%,10%)", "RGBA(81%,89%,10%,1)", "rgba(81%,89%,10%,1)", "RGBA(81%,89%,10%,1)", ],
			vec!["rgb(81%,89%,10%)", "RGB(81%,89%,10%)", "rgb(81%,89%,10%)", "rgba(81%,89%,10%,0.5)", "RGB(81%,89%,10%)", "RGBA(81%,89%,10%,0.5)", "rgba(81%,89%,10%,0.5)", "RGBA(81%,89%,10%,0.5)", ],
			vec!["rgb(32%,35%,5%)", "RGB(32%,35%,5%)", "rgb(32%,35%,5%)", "rgba(32%,35%,5%,0.3)", "RGB(32%,35%,5%)", "RGBA(32%,35%,5%,0.3)", "rgba(32%,35%,5%,0.3)", "RGBA(32%,35%,5%,0.3)", ],
		];

		let test_color = init_color(rgb_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(rgb_result[i][index], color.to_rgb().unwrap());
//				print!("{:?}, ", color.to_rgb().unwrap());
			}
//			println!("],");
		}
	}
}