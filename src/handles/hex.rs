use config::Setting;
use handles::map;
// hex -- #fff,#ffffff,#ffffff80,#80ffffff etc..
// return -- ['f','f','f','f','f','f'],['f','f','f','f','f','f','8','0'] etc...
pub fn handle_hex_value<'a>(hex: &'a str, setting: &Setting) -> Vec<&'a str> {
	let mut hex_vec: Vec<&'a str> = hex.split("").collect();
	hex_vec.retain(|&x| x != "" && x != "#");

	let mut return_vex: Vec<&'a str> = vec![];
	match hex_vec.len() {
		3 => {
			for item in &hex_vec {
				// let value: &'a str = &format!("{}{}", item, item);
				// let upper_item: &'a str = item.to_ascii_uppercase();
				return_vex.push(item);
				return_vex.push(item);
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
		_ => panic!("[color-convert] hex value length must one of in [3, 6, 8]")
	}
	return_vex
}

// transparency value convert to hexadecimal
// 0.5 -> "80"
pub fn handel_alpha_to_hexadecimal(alpha: f32) -> String {
	if alpha == 1f32 || alpha > 1f32 {
		return String::from("FF");
	}

	let to_deciaml = alpha * 256f32;
	let b = (to_deciaml % 16f32) as usize;
	let a = (to_deciaml / 16f32 % 16f32) as usize;

	// let value: &'a str = &*format!("{}{}", map::map_rgb(a), map::map_rgb(b));
	map::map_rgb(&a).to_owned() + map::map_rgb(&b)
}