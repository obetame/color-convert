extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
//	use color_convert::color::Color;
	use common::init_color;

	#[test]
	fn test_hsl2rgb() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)", "hsla(66.23,76.24%,19.8%,.5)"];
		let hex_result = vec![
			vec!["rgb(80.99,88.98,12)","RGB(80.99,88.98,12)","rgb(80.99,88.98,12)","rgba(80.99,88.98,12,1)","RGB(80.99,88.98,12)","RGBA(80.99,88.98,12,1)","rgba(80.99,88.98,12,1)","RGBA(80.99,88.98,12,1)",],
			vec!["rgb(80.99,88.98,12)","RGB(80.99,88.98,12)","rgb(80.99,88.98,12)","rgba(80.99,88.98,12,1)","RGB(80.99,88.98,12)","RGBA(80.99,88.98,12,1)","rgba(80.99,88.98,12,1)","RGBA(80.99,88.98,12,1)",],
			vec!["rgb(80.99,88.98,12)","RGB(80.99,88.98,12)","rgb(80.99,88.98,12)","rgba(80.99,88.98,12,0.5)","RGB(80.99,88.98,12)","RGBA(80.99,88.98,12,0.5)","rgba(80.99,88.98,12,0.5)","RGBA(80.99,88.98,12,0.5)",],
		];

		let test_color = init_color(hsl_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_rgb().unwrap());
//				print!("{:?},", color.to_rgb().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsl2hex() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)", "hsla(66.23,76.24%,19.8%,.5)"];
		let hex_result = vec![
			vec!["#50580c","#50580C","#50580c","#50580cff","#50580C","#50580CFF","#ff50580c","#FF50580C",],
			vec!["#50580c","#50580C","#50580c","#50580cff","#50580C","#50580CFF","#ff50580c","#FF50580C",],
			vec!["#50580c","#50580C","#50580c","#50580c80","#50580C","#50580C80","#8050580c","#8050580C",],
		];

		let test_color = init_color(hsl_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hex().unwrap());
//				print!("{:?},", color.to_hex().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsl2hsl() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)", "hsla(66.23,76.24%,19.8%,.5)"];
		let hex_result = vec![
			vec!["hsl(66.23,76.24%,19.8%)","HSL(66.23,76.24%,19.8%)","hsl(66.23,76.24%,19.8%)","hsla(66.23,76.24%,19.8%,1)","HSL(66.23,76.24%,19.8%)","HSLA(66.23,76.24%,19.8%,1)","hsla(66.23,76.24%,19.8%,1)","HSLA(66.23,76.24%,19.8%,1)",],
			vec!["hsl(66.23,76.24%,19.8%)","HSL(66.23,76.24%,19.8%)","hsl(66.23,76.24%,19.8%)","hsla(66.23,76.24%,19.8%,1)","HSL(66.23,76.24%,19.8%)","HSLA(66.23,76.24%,19.8%,1)","hsla(66.23,76.24%,19.8%,1)","HSLA(66.23,76.24%,19.8%,1)",],
			vec!["hsl(66.23,76.24%,19.8%)","HSL(66.23,76.24%,19.8%)","hsl(66.23,76.24%,19.8%)","hsla(66.23,76.24%,19.8%,0.5)","HSL(66.23,76.24%,19.8%)","HSLA(66.23,76.24%,19.8%,0.5)","hsla(66.23,76.24%,19.8%,0.5)","HSLA(66.23,76.24%,19.8%,0.5)",],
		];

		let test_color = init_color(hsl_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hsl().unwrap());
//				print!("{:?},", color.to_hsl().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsl2cmyk() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)", "hsla(66.23,76.24%,19.8%,.5)"];
		let hex_result = vec![
			vec!["cmyk(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)","cmyk(0.09,0,0.87,0.65)","cmyk(0.68,0.65,0.95,0)","CMYK(0.09,0,0.87,0.65)","CMYK(0.68,0.65,0.95,0)","cmyk(0.68,0.65,0.95,0)","CMYK(0.68,0.65,0.95,0)",],
			vec!["cmyk(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)","cmyk(0.09,0,0.87,0.65)","cmyk(0.68,0.65,0.95,0)","CMYK(0.09,0,0.87,0.65)","CMYK(0.68,0.65,0.95,0)","cmyk(0.68,0.65,0.95,0)","CMYK(0.68,0.65,0.95,0)",],
			vec!["cmyk(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)","cmyk(0.09,0,0.87,0.65)","cmyk(0.36,0.3,0.91,0.5)","CMYK(0.09,0,0.87,0.65)","CMYK(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)",],
		];

		let test_color = init_color(hsl_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_cmyk().unwrap());
//				print!("{:?},", color.to_cmyk().unwrap());
			}
//			println!("],");
		}
	}

	#[test]
	fn test_hsl2hvv() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)", "hsla(66.23,76.24%,19.8%,.5)"];
		let hex_result = vec![
			vec!["hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)",],
			vec!["hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)",],
			vec!["hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)","hsv(66.22,86.5%,34.89%)","HSV(66.22,86.5%,34.89%)",],
		];

		let test_color = init_color(hsl_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
//			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hsv().unwrap());
//				print!("{:?},", color.to_hsv().unwrap());
			}
//			println!("],");
		}
	}
}