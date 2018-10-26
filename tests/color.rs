extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::color::Color;
	use color_convert::color::ColorMode;
	
	#[test]
	fn test_color() {
		let color = Color::new("rgb(1,1,1)", false, false, false);
		assert_eq!(color.mode, ColorMode::new("rgb(1,1,1)"));
		assert_eq!(color.to_upper, false);
		assert_eq!(color.to_android, false);
		assert_eq!(color.to_alpha, false);
	}

	#[test]
	fn test_utils_get_color_mode() {
		let data = ColorMode::new("rgba(1,2,3,.9)");
		let data1 = ColorMode::new("hsla(1,2,3,.9)");

		assert_eq!(ColorMode::RGBA("rgba(1,2,3,.9)"), data);
		assert_eq!(ColorMode::HSLA("hsla(1,2,3,.9)"), data1);

		assert_ne!(ColorMode::RGB("rgba(1,2,3,.9)"), data);
		assert_ne!(ColorMode::HSL("hsla(1,2,3,.9)"), data1);
	}
}