use regex::Regex;
//use regex::Captures;
use color::{Color, Error};
use handles::map;
use utils;

// rgb -> rgba(81%,89%,12%,1),rgb(81,89,12),rgba(81%,89%,12%,0.3) etc..
// return -> [0.81,0.89,0.12,1],[0.81,0.89,0.12],[0.81,0.89,0.12,0.3] etc...
pub fn handle_rgb(color: &Color) -> Result<Vec<f32>, Error> {
	let re = Regex::new(r"(?i)rgba?\(\s*(?P<r>\d{1,3}\.?\d*%?)\s*,\s*(?P<g>\d{1,3}\.?\d*%?)\s*,\s*(?P<b>\d{1,3}\.?\d*%?)\s*,?\s*(?P<alpha>\.?\d{1,3}\.?\d*%?)?\s*\)").expect("Parse rgb value error");
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		let r = utils::convert_rgb_value_to_number(&value["r"]);
		let g = utils::convert_rgb_value_to_number(&value["g"]);
		let b = utils::convert_rgb_value_to_number(&value["b"]);

		let match_alpha = value.get(4);
		if let Some(value) = match_alpha {
			let a = utils::convert_alpha_value_to_number(value.as_str());
//			let a = value.as_str().parse::<f32>().unwrap();

			return Ok(vec![utils::round(r, 4), utils::round(g, 4), utils::round(b, 4), a]);
		}

		return Ok(vec![utils::round(r, 4), utils::round(g, 4), utils::round(b, 4)])
	}

	Err(Error::Format)
}

// rgb -> rgba(81%,89%,12%,1),rgb(81,89,12),rgba(81%,89%,12%,0.3) etc..
// return -> #cee21eff,#51590c,#CEE21E4C etc...
pub fn rgb2hex(color: &Color) -> Result<String, Error> {
	let cap = handle_rgb(color)?;
	let mut value: Vec<f32> = vec![];
	let mut hex = String::from("#");

	for n in &cap {
		value.push(n * 255f32);
	}

	for n in 0..3usize {
		if value[n] == 0f32 {
			hex.push_str("00");
			continue;
		}

		let low = (value[n] / 16f32) as usize;
		let high = (value[n] % 16f32) as usize;
		hex.push_str(map::map_rgb(&low));
		hex.push_str(map::map_rgb(&high));
	}

	if color.to_alpha {
		let mut alpha = 1f32;
		if cap.len() == 4 {
			alpha = cap[3];
		}
		let hexadecimal = &utils::handel_alpha_to_hexadecimal(alpha);
		if color.to_android {
			hex.insert_str(1, hexadecimal);
		} else {
			hex.push_str(hexadecimal);
		}
	}

	if color.to_upper {
		Ok(hex)
	} else {
		Ok(hex.to_lowercase())
	}
}

// rgb -> rgb( 81 , 89% , 10%), rgba(81%,89%,10%,0.5)
// return -> hsl(103.47,79.79%,49.5%), hsla(66.08,79.79%,49.5%,0.5)
pub fn rgb2hsl(color: &Color) -> Result<String, Error> {
	let cap: Vec<f32> = handle_rgb(&color)?;
	let rgb = &cap.clone()[0..3];

	let max = *rgb.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
	let min = *rgb.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

	let mut h = 0f32;
	let mut s = 0f32;
	let l = (max + min) / 2f32;

	if max == min {
		h = 0f32;
		s = 0f32;
	} else {
		let mid = max - min;
		if  l > 0.5f32 {
			s = mid / (2f32 - max - min);
		} else if l <= 0.5f32 && l > 0f32 {
			s = mid /(max + min);
		} else if l == 0f32 {
			s = 0f32;
		}

		if max == cap[0] && cap[1] >= cap[2] {
			h = 60f32 * (cap[1] - cap[2]) / mid;
		} else if max == cap[0] && cap[1] < cap[1] {
			h = 60f32 * (cap[1] - cap[2]) / mid + 360f32;
		} else if max == cap[1] || max == cap[2] {
			h = 60f32 * (cap[2] - cap[0]) / mid + 120f32;
		}
	}

	let mut hsl = format!("({},{},{}", utils::convert_f32_to_string(h), utils::convert_f32_to_percent(s), utils::convert_f32_to_percent(l));
	if color.to_alpha {
		hsl.push_str(",");
		if cap.len() == 4usize {
			hsl.push_str(&cap[3].to_string());
			hsl.push_str(")");
		} else {
			hsl.push_str("1)");
		}
		hsl.insert_str(0, "hsla");
	} else {
		hsl.push_str(")");
		hsl.insert_str(0, "hsl");
	}

	if color.to_upper {
		hsl = hsl.to_uppercase()
	}
	Ok(hsl)
}

// rgb -> rgb(81,89,12), rgba(81,89,12,1)
// return -> cmyk(0.09,0,0.865,0.651)
// https://www.rapidtables.com/convert/color/rgb-to-cmyk.html
pub fn rgb2cmyk(color: &Color) -> Result<String, Error> {
	let cap: Vec<f32> = handle_rgb(&color)?;
	let r = cap[0];
	let g = cap[1];
	let b = cap[2];
	let max = *cap.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

	let k = ((1f32 - max) * 100f32).round() / 100f32;
	if k == 1f32 {
		if color.to_upper {
			return Ok(String::from("CMYK(0,0,0,1)"));
		} else {
			return Ok(String::from("cmyk(0,0,0,1)"));
		}
	}

	let c = ((1f32 - r - k) / (1f32 - k) * 100f32).round() / 100f32;
	let m = ((1f32 - g - k) / (1f32 - k) * 100f32).round() / 100f32;
	let y = ((1f32 - b - k) / (1f32 - k) * 100f32).round() / 100f32;

	let mut cmyk = vec![c, m, y, k];
	for index in 0..4 {
		if cmyk[index] < 0f32 {
			cmyk[index] = 0f32;
		}
	}

	if color.to_upper {
		Ok(format!("CMYK({},{},{},{})", cmyk[0], cmyk[1], cmyk[2], cmyk[3]))
	} else {
		Ok(format!("cmyk({},{},{},{})", cmyk[0], cmyk[1], cmyk[2], cmyk[3]))
	}
}

// rgb -> rgb(81,89,12), rgba(81,89,12,1)
// return -> hsv(66,86.52%,34.9%)
// https://www.rapidtables.com/convert/color/rgb-to-cmyk.html
pub fn rgb2hsv(color: &Color) -> Result<String, Error> {
	let cap: Vec<f32> = handle_rgb(&color)?;
	let rgb = &cap.clone()[0..3];

	let r = rgb[0];
	let g = rgb[1];
	let b = rgb[2];
	let mut h = 0f32;
	let s;
	let v;

	let max = *rgb.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
	let min = *rgb.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
	let mid = max - min;

	if mid == 0f32 {
		h = 0f32;
	} else if max == r {
		h = 60f32 * (((g - b) / mid) % 6f32);
	} else if max == g {
		h = 60f32 * ((b - r) / mid + 2f32);
	} else if max == b {
		h = 60f32 * ((r - g) / mid + 4f32);
	}

	if max == 0f32 {
		s = 0f32;
	} else {
		s = (mid / max * 10000f32).round() / 100f32; // keep two decimals
	}

	v = (max * 10000f32).round() / 100f32;

	if color.to_upper {
		Ok(format!("HSV({},{}%,{}%)", (h * 100f32).round() / 100f32, s, v))
	} else {
		Ok(format!("hsv({},{}%,{}%)", (h * 100f32).round() / 100f32, s, v))
	}
}

// rgb -> rgb(81,89,12), rgba(81,89,12,1)
// return -> rgb(66,86.52%,34.9%)
pub fn rgb2rgb(color: &Color) -> Result<String, Error> {
	let cap: Vec<f32> = handle_rgb(&color)?;
	let mut alpha = 1f32;

	if cap.len() == 4 {
		alpha = cap[3];
	}

	let r = utils::decimal_to_percent(cap[0]);
	let g = utils::decimal_to_percent(cap[1]);
	let b = utils::decimal_to_percent(cap[2]);

	let rgb;
	if color.to_upper {
		if color.to_alpha {
			rgb = format!("RGBA({},{},{},{})", r, g, b, if alpha > 1f32 {1f32} else {alpha});
		} else {
			rgb = format!("RGB({},{},{})", r, g, b);
		}
	} else {
		if color.to_alpha {
			rgb = format!("rgba({},{},{},{})", r, g, b, if alpha > 1f32 {1f32} else {alpha});
		} else {
			rgb = format!("rgb({},{},{})", r, g, b);
		}
	}

	Ok(rgb)
}