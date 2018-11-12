extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
	use color_convert::color::Color;
	use common::init_color;

	#[test]
	fn test_hsl2rgb() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)"];
		let hex_result = vec![
			"RGB(80.99,88.98,12)", "rgb(80.99,88.98,12)", "rgba(80.99,88.98,12,1)",
			"RGB(80.99,88.98,12)", "rgb(80.99,88.98,12)", "rgba(80.99,88.98,12,1)",
		];

		for (index, color) in hsl_vec.iter().enumerate() {
			for i in 0..3 {
				let mut color = Color::new(color, false, false, false);
				match i {
					0 => assert_eq!(color.upper(true).to_rgb().unwrap(), hex_result[index * 3]),
					1 => assert_eq!(color.android(true).to_rgb().unwrap(), hex_result[index * 3 + 1]),
					2 => assert_eq!(color.alpha(true).to_rgb().unwrap(), hex_result[index * 3 + 2]),
					_ => println!("noting")
				}
//				match i {
//					0 => println!("{}", color.upper(true).to_rgb().unwrap()),
//					1 => println!("{}", color.android(true).to_rgb().unwrap()),
//					2 => println!("{}", color.alpha(true).to_rgb().unwrap()),
//					_ => println!("noting")
//				}
			}
		}
	}

	#[test]
	fn test_hsl2hex() {
		let hsl_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)"];
		let hex_result = vec![
			vec!["#50580c", "#50580C", "#50580c", "#50580cff", "#50580C", "#50580CFF", "#ff50580c", "#FF50580C"],
			vec!["#50580c", "#50580C", "#50580c", "#50580cff", "#50580C", "#50580CFF", "#ff50580c", "#FF50580C"]
		];

		let test_color = init_color(hsl_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
			for (index, color) in vec_color.iter().enumerate() {
				assert_eq!(hex_result[i][index], color.to_hex().unwrap());
			}
		}
	}
}