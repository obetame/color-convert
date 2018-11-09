extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::color::{Color};

	#[test]
	fn test_rgb_tohsl() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			"HSL(103.47,79.79%,49.5%)", "hsl(103.47,79.79%,49.5%)", "hsla(103.47,79.79%,49.5%,1)",
			"HSL(66.08,79.79%,49.5%)", "hsl(66.08,79.79%,49.5%)", "hsla(66.08,79.79%,49.5%,0.5)",
			"HSL(66.23,76.23%,19.80%)", "hsl(66.23,76.23%,19.80%)", "hsla(66.23,76.23%,19.80%,0.3)",
		];

		for (index, color) in rgb_vec.iter().enumerate() {
			for i in 0..3 {
				let mut color = Color::new(color, false, false, false);
				match i {
					0 => assert_eq!(color.upper(true).to_hsl().unwrap(), rgb_result[index * 3]),
					1 => assert_eq!(color.android(true).to_hsl().unwrap(), rgb_result[index * 3 + 1]),
					2 => assert_eq!(color.alpha(true).to_hsl().unwrap(), rgb_result[index * 3 + 2]),
					_ => println!("noting")
				}
			}
		}
	}

	#[test]
	fn test_rgb_tocmykl() {
		let rgb_vec = vec!["rgb( 81 , 89% , 10%)", "rgba(81%,89%,10%,0.5)", "rgba(81,89,12,0.3)"];
		let rgb_result = vec![
			"CMYK(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)", "cmyk(0.64,0,0.89,0.11)",
			"CMYK(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)", "cmyk(0.09,0,0.89,0.11)",
			"CMYK(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)", "cmyk(0.09,0,0.87,0.65)",
		];

		for (index, color) in rgb_vec.iter().enumerate() {
			for i in 0..3 {
				let mut color = Color::new(color, false, false, false);
				match i {
					0 => assert_eq!(color.upper(true).to_cmyk().unwrap(), rgb_result[index * 3]),
					1 => assert_eq!(color.android(true).to_cmyk().unwrap(), rgb_result[index * 3 + 1]),
					2 => assert_eq!(color.alpha(true).to_cmyk().unwrap(), rgb_result[index * 3 + 2]),
					_ => println!("noting")
				}
//				match i {
//					0 => println!("{}", color.upper(true).to_cmyk().unwrap()),
//					1 => println!("{}", color.android(true).to_cmyk().unwrap()),
//					2 => println!("{}", color.alpha(true).to_cmyk().unwrap()),
//					_ => println!("noting")
//				}
			}
		}
	}
}