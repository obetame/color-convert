use color::Color;
use handles::map;

// hex -- #fff,#ffffff,#ffffff80,#80ffffff etc..
// return -- ['f','f','f','f','f','f'],['f','f','f','f','f','f','8','0'] etc...
pub fn handle_hex_value<'a>(hex: &'a str, setting: &Color) -> Result<Vec<&'a str>, &'static str> {
	let mut hex_vec: Vec<&'a str> = hex.split("").collect();
	hex_vec.retain(|&x| x != "" && x != "#");

	let mut return_vex: Vec<&'a str> = vec![];
	match hex_vec.len() {
		3 => {
			for item in &hex_vec {
				// let value: &'a str = &format!("{}{}", item, item);
				// let upper_item: &'a str = item.to_ascii_uppercase();
				return_vex.extend_from_slice(&[*item, *item]);
			}
		},
		6 => return_vex.extend(&hex_vec),
		8 => {
			if setting.is_android {
				return_vex.extend(&hex_vec[2..]);
				return_vex.extend(&hex_vec[0..2]);
			} else {
				return_vex.extend(&hex_vec);
			}
		},
		_ => return Err("[color-convert] hex value length must one of in [3, 6, 8]")
	}
	Ok(return_vex)
}

pub fn hex2rgb(hex: &str, setting: &Color) -> Result<String, &'static str> {
	let hex_vec = handle_hex_value(&hex, &setting)?;
	let mut rgb_string = String::new();
	let tool_array: [usize; 3] = [0, 2, 4];

	if hex_vec.len() == 8 {

	}

	for n in tool_array.iter() {
		let data = map::map_hex(hex_vec[*n]) * 16 + map::map_hex(hex_vec[*n + 1]);
		rgb_string.push_str(&data.to_string());
		rgb_string.push(',');
	}

	if setting.is_android {
		rgb_string.push_str("rgb(")
	}

	Ok(rgb_string)
}