extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::color::{Color};

	#[test]
	fn test_rgb_tohsl() {
		let color = Color::init("rgb( 81 , 89% , 10%)");
		let color1 = Color::new("rgb(81%,89%,10%,0.5)", false, false, true);

		assert_eq!("hsl(103.47,79.79%,49.5%)", color.to_hsl().unwrap());
		assert_eq!("hsla(66.08,79.79%,49.5%,0.5)", color1.to_hsl().unwrap());
	}
}