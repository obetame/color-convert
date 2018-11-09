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
}