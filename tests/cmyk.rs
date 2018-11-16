extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
	//	use color_convert::color::Color;
	use common::init_color;

	#[test]
	fn test_cmyk2rgb() {
		let hsl_vec = vec!["cmyk(0.09,0,0.87,0.65)", "cmyk(0.36,0.3,0.91,0.5)", "CMYK(0.36,0.3,0.91,0.5)"];
		let hex_result = vec![
			vec!["rgb(81,89,12)","RGB(81,89,12)","rgb(81,89,12)","rgba(81,89,12,1)","RGB(81,89,12)","RGBA(81,89,12,1)","rgba(81,89,12,1)","RGBA(81,89,12,1)",],
			vec!["rgb(82,89,11)","RGB(82,89,11)","rgb(82,89,11)","rgba(82,89,11,1)","RGB(82,89,11)","RGBA(82,89,11,1)","rgba(82,89,11,1)","RGBA(82,89,11,1)",],
			vec!["rgb(82,89,11)","RGB(82,89,11)","rgb(82,89,11)","rgba(82,89,11,1)","RGB(82,89,11)","RGBA(82,89,11,1)","rgba(82,89,11,1)","RGBA(82,89,11,1)",],
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
	fn test_cmyk2hex() {
		let hsl_vec = vec!["cmyk(0.09,0,0.87,0.65)", "cmyk(0.36,0.3,0.91,0.5)", "CMYK(0.36,0.3,0.91,0.5)"];
		let hex_result = vec![
			vec!["#50580c","#50580C","#50580c","#50580cff","#50580C","#50580CFF","#ff50580c","#FF50580C",],
			vec!["#52580a","#52580A","#52580a","#52580aff","#52580A","#52580AFF","#ff52580a","#FF52580A",],
			vec!["#52580a","#52580A","#52580a","#52580aff","#52580A","#52580AFF","#ff52580a","#FF52580A",],
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
	fn test_cmyk2hsl() {
		let hsl_vec = vec!["cmyk(0.09,0,0.87,0.65)", "cmyk(0.36,0.3,0.91,0.5)", "CMYK(0.36,0.3,0.91,0.5)"];
		let hex_result = vec![
			vec!["hsl(66.24,76.21%,19.80%)","HSL(66.24,76.21%,19.80%)","hsl(66.24,76.21%,19.80%)","hsla(66.24,76.21%,19.80%,1)","HSL(66.24,76.21%,19.80%)","HSLA(66.24,76.21%,19.80%,1)","hsla(66.24,76.21%,19.80%,1)","HSLA(66.24,76.21%,19.80%,1)",],
			vec!["hsl(65.37,78.01%,19.60%)","HSL(65.37,78.01%,19.60%)","hsl(65.37,78.01%,19.60%)","hsla(65.37,78.01%,19.60%,1)","HSL(65.37,78.01%,19.60%)","HSLA(65.37,78.01%,19.60%,1)","hsla(65.37,78.01%,19.60%,1)","HSLA(65.37,78.01%,19.60%,1)",],
			vec!["hsl(65.37,78.01%,19.60%)","HSL(65.37,78.01%,19.60%)","hsl(65.37,78.01%,19.60%)","hsla(65.37,78.01%,19.60%,1)","HSL(65.37,78.01%,19.60%)","HSLA(65.37,78.01%,19.60%,1)","hsla(65.37,78.01%,19.60%,1)","HSLA(65.37,78.01%,19.60%,1)",],
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
	fn test_cmyk2cmyk() {
		let hsl_vec = vec!["cmyk(0.09,0,0.87,0.65)", "cmyk(0.36,0.3,0.91,0.5)", "CMYK(0.36,0.3,0.91,0.5)"];
		let hex_result = vec![
			vec!["cmyk(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)","cmyk(0.09,0,0.87,0.65)","cmyk(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)","cmyk(0.09,0,0.87,0.65)","CMYK(0.09,0,0.87,0.65)",],
			vec!["cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)",],
			vec!["cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)","cmyk(0.36,0.3,0.91,0.5)","CMYK(0.36,0.3,0.91,0.5)",],
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
	fn test_cmyk2hsv() {
		let hsl_vec = vec!["cmyk(0.09,0,0.87,0.65)", "cmyk(0.36,0.3,0.91,0.5)", "CMYK(0.36,0.3,0.91,0.5)"];
		let hex_result = vec![
			vec!["hsv(66.24,86.5%,34.9%)","HSV(66.24,86.5%,34.9%)","hsv(66.24,86.5%,34.9%)","hsv(66.24,86.5%,34.9%)","HSV(66.24,86.5%,34.9%)","HSV(66.24,86.5%,34.9%)","hsv(66.24,86.5%,34.9%)","HSV(66.24,86.5%,34.9%)",],
			vec!["hsv(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)","hsv(65.37,87.65%,34.9%)","hsv(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)","hsv(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)",],
			vec!["hsv(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)","hsv(65.37,87.65%,34.9%)","hsv(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)","hsv(65.37,87.65%,34.9%)","HSV(65.37,87.65%,34.9%)",],
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