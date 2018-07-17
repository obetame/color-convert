// use std::ascii::AsciiExt;

pub mod common {
	use config::color_mode;
	use config::cc_config::Setting;
	use handles::map_name;

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
	// ['8','0','f','f','f','f','f','f'] is_android = true -> 0.5
	// ['f','f','f','f','f','f','8','0'] is_android = false -> 0.5
	pub fn get_hex_alpha_value(color: &Vec<&str>, setting: &Setting) -> f32 {
		let value: &[&str] = if setting.is_android {
			&color[..2]
		} else {
			&color[&color.len() - 2..]
		};

		let a: usize = map_name::map_hex(&*value[1].to_uppercase());
		let b: usize = map_name::map_hex(&*value[0].to_uppercase());

		((a + b * 16) as f32 / 255.0 * 10000.0).round() / 10000.0
	}

	// Get rgba transparency value and convert to hexadecimal
	// "rgba(1,1,1,.5)" -> 0.5
	pub fn get_rgba_alpha_value(color: &str) -> f32 {
		let vec_value: Vec<&str> = color.split(',').collect();
		
		let result = match vec_value.len() {
			3 => 1f32,
			4 => {
				let alpha: String = vec_value[3].replace(")", "");
				match alpha.parse::<f32>() {
					Ok(value) => value,
					Err(error) => panic!("{} value is not a number, unable to convert to f32"),
				}
			},
			_ => panic!("{} value is not formatted correctly.")
		};

		result
	}
}