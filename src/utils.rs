// use std::ascii::AsciiExt;
//use config::ColorMode;
use color::{Color, Error};
use handles::map;
use handles::hex;

// Get hex's transparency value and convert to decimal
// ['8','0','f','f','f','f','f','f'] to_android = true -> 0.5
// ['f','f','f','f','f','f','8','0'] to_android = false -> 0.5
pub fn get_hex_alpha_value(color: &Color) -> f32 {
	let hex_vec = hex::handle_hex_value( &color).unwrap();
	let value: &[&str] = &hex_vec[&hex_vec.len() - 2..];

	let a: usize = map::map_hex(&*value[1].to_uppercase());
	let b: usize = map::map_hex(&*value[0].to_uppercase());

	((a + b * 16) as f32 / 255.0 * 10000.0).round() / 10000.0
}

// Get rgba transparency value and convert to hexadecimal
// "rgba(1,1,1,.5)" -> 0.5
pub fn get_rgba_alpha_value(color: &str) -> Result<f32, Error> {
	let vec_value: Vec<&str> = color.split(',').collect();

	let result = match vec_value.len() {
		3 => 1f32,
		4 => {
			let alpha: String = vec_value[3].replace(")", "");
			match alpha.parse::<f32>() {
				Ok(value) => value,
				Err(_error) => return Err(Error::RgbAlphaFormat),
			}
		},
		_ => return Err(Error::RgbAlphaFormat)
	};

	Ok(result)
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

// convert value_string to number
// "50%" -> 0.5; "0.5" -> 0.5; ".5" -> 0.5, "244" -> 244.0
pub fn convert_value_to_number(value: &str) ->f32 {
	let n = value.replace("%", "");
	let result = match n.parse::<f32>() {
		Ok(value) => value,
		Err(_error) => 0f32,
	};

	result
}
