// use std::ascii::AsciiExt;

pub mod common {
	use config::color_mode;
	use config::cc_config::Setting;

	// get color mode
	// "rgb(1,1,1)" -> ColorMode::RGB("rgb(1,1,1)")
	pub fn get_color_mode(color: &str) -> color_mode::ColorMode {
		let upper_color = color.to_ascii_uppercase();

		if upper_color.contains("RGBA") {
			color_mode::ColorMode::RGBA(color)
		} else if upper_color.contains("RGB") {
			color_mode::ColorMode::RGB(color)
		} else if upper_color.contains("HSLA") {
			color_mode::ColorMode::HSLA(color)
		} else if upper_color.contains("HSL") {
			color_mode::ColorMode::HSL(color)
		} else if upper_color.contains("CMYK") {
			color_mode::ColorMode::CMYK(color)
		} else if upper_color.contains("HSV") {
			color_mode::ColorMode::HSV(color)
		} else {
			color_mode::ColorMode::HEX(color)
		}
	}

	// Get hex's transparency value and convert to decimal
	// ['f','f','f','f','f','f','f','f'] -> 1.0
	pub fn get_hex_alpha_value(color: &Vec<&str>, setting: &Setting) -> f32 {
		println!("{:?}", color);
		2.0
	}
}