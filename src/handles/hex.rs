use color::{Color, Error};
use handles::map;

// hex -- #fff,#ffffff,#ffffff80,#80ffffff etc..
// return -- ['f','f','f','f','f','f'],['f','f','f','f','f','f','8','0'] etc...
pub fn handle_hex_value<'a>(color: &'a Color) -> Result<Vec<&'a str>, Error> {
	let mut hex_vec: Vec<&'a str> = color.to_str().split("").collect();
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
			if color.to_android {
				return_vex.extend(&hex_vec[2..]);
				return_vex.extend(&hex_vec[0..2]);
			} else {
				return_vex.extend(&hex_vec);
			}
		},
		_ => return Err(Error::Format) // "[color-convert] hex value length must one of in [3, 6, 8]"
	}
	Ok(return_vex)
}

pub fn hex2rgb(color: &Color) -> Result<String, Error> {
	let hex_vec = handle_hex_value(&color)?;
	let mut rgb_string = String::new();
	let tool_array: [usize; 3] = [0, 2, 4];

	for n in tool_array.iter() {
		let data = map::map_hex(hex_vec[*n]) * 16 + map::map_hex(hex_vec[*n + 1]);
		rgb_string.push_str(&format!("{}", data));
		rgb_string.push(',');
	}

	if color.to_alpha {
		rgb_string.insert_str(0, "rgba(");
		if hex_vec.len() == 8 {
			let data = map::map_hex(hex_vec[6]) * 16 + map::map_hex(hex_vec[7]);
			rgb_string.push_str(&format!("{:.2})", data as f32 / 255f32));
		} else {
			rgb_string.push_str("1)");
		}
	} else {
		rgb_string.insert_str(0, "rgb(");
		rgb_string.pop();
		rgb_string.push_str(")");
	}

	if color.to_upper {
		Ok(rgb_string.to_uppercase())
	} else {
		Ok(rgb_string.to_lowercase())
	}
}

pub fn hex2hex(color: &Color) -> Result<String, Error> {
	let hex_vec = handle_hex_value(&color)?;
	let mut hex_string = hex_vec.join("");

	if color.to_alpha {
		if hex_vec.len() != 8 {
			if color.to_android {
				hex_string.insert_str(0, "ff");
			} else {
				hex_string.push_str("FF");
			}
		} else {
			if color.to_android {
				//hex_string.insert_str(0, &hex_string.pop().unwrap().to_string());
				hex_string = format!("{}{}", &hex_string[6..8], &hex_string[0..6]);
			}
		}
	} else if hex_vec.len() == 8 {
		hex_string = String::from(&hex_string[..6]);
	}

	hex_string.insert_str(0, "#");
	if color.to_upper {
		Ok(hex_string.to_uppercase())
	} else {
		Ok(hex_string.to_lowercase())
	}
}

pub fn hex2hsl(color: &Color) -> Result<String, Error> {
	let rgb_string = hex2rgb(&color)?;
	let color_rgb = color.copy(&rgb_string);

	Ok(color_rgb.to_hsl()?)
}