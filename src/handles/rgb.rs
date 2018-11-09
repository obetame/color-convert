use regex::Regex;
//use regex::Captures;
use color::{Color, Error};
use utils;

// rgb -> rgba(81%,89%,12%,1),rgb(81,89,12),rgba(81%,89%,12%,0.3) etc..
// return -> [0.81,0.89,0.12,1],[0.81,0.89,0.12] [0.81,0.89,0.12,0.3] etc...
pub fn handle_rgb(color: &Color) -> Result<Vec<f32>, Error> {
	let re = Regex::new(r"(?i)rgba?\(\s*(?P<r>\d{1,3}\.?\d*%?)\s*,\s*(?P<g>\d{1,3}\.?\d*%?)\s*,\s*(?P<b>\d{1,3}\.?\d*%?)\s*,?\s*(?P<alpha>\.?\d{1,3}\.?\d*%?)?\s*\)").unwrap();
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		let r = utils::convert_value_to_number(&value["r"]);
		let g = utils::convert_value_to_number(&value["g"]);
		let b = utils::convert_value_to_number(&value["b"]);

		let match_alpha = value.get(4);
		if let Some(value) = match_alpha {
			let a = value.as_str().parse::<f32>().unwrap();

			return Ok(vec![r, g, b, a]);
		}

		return Ok(vec![r, g, b])
	}

	Err(Error::Format)
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