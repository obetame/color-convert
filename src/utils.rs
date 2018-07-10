// use std::ascii::AsciiExt;

pub mod common {
	use config::color_mode;

	pub fn get_color_mode(color: &str) -> color_mode::ColorMode {
		let upper_color = color.to_ascii_uppercase();

		if upper_color.contains("RGBA") {
			color_mode::ColorMode::RGBA(color.to_string())
		} else if upper_color.contains("RGB") {
			color_mode::ColorMode::RGB(color.to_string())
		} else if upper_color.contains("HSLA") {
			color_mode::ColorMode::HSLA(color.to_string())
		} else if upper_color.contains("HSL") {
			color_mode::ColorMode::HSL(color.to_string())
		} else if upper_color.contains("CMYK") {
			color_mode::ColorMode::CMYK(color.to_string())
		} else if upper_color.contains("HSV") {
			color_mode::ColorMode::HSV(color.to_string())
		} else {
			color_mode::ColorMode::HEX(color.to_string())
		}
	}
}