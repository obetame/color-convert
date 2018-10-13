// use std::ascii::AsciiExt;
use config::ColorMode;
use config::Setting;
use handles::map;

// get color mode
// "rgb(1,1,1)" -> ColorMode::RGB("rgb(1,1,1)")
pub fn get_color_mode(color: &str) -> ColorMode {
	let upper_color = color.to_ascii_uppercase();

	if upper_color.contains("RGBA") {
		ColorMode::RGBA(color)
	} else if upper_color.contains("RGB") {
		ColorMode::RGB(color)
	} else if upper_color.contains("HSLA") {
		ColorMode::HSLA(color)
	} else if upper_color.contains("HSL") {
		ColorMode::HSL(color)
	} else if upper_color.contains("CMYK") {
		ColorMode::CMYK(color)
	} else if upper_color.contains("HSV") {
		ColorMode::HSV(color)
	} else {
		ColorMode::HEX(color)
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

	let a: usize = map::map_hex(&*value[1].to_uppercase());
	let b: usize = map::map_hex(&*value[0].to_uppercase());

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
				Err(_error) => panic!("alpha value is not a number, unable to convert to f32"),
			}
		},
		_ => panic!("{} value is not formatted correctly.")
	};

	result
}

// transparency value convert to hexadecimal
// 0.5 -> "80"
pub fn handel_alpha_to_hexadecimal(alpha: f32) -> String {
	if alpha == 1f32 || alpha > 1f32 {
		return String::from("FF");
	}

	let to_decimal = alpha * 256f32;
	let b = (to_decimal % 16f32) as usize;
	let a = (to_decimal / 16f32 % 16f32) as usize;

	// let value: &'a str = &*format!("{}{}", map::map_rgb(a), map::map_rgb(b));
	map::map_rgb(&a).to_owned() + map::map_rgb(&b)
}