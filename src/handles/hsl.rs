use regex::Regex;
use color::{Color, Error};
use utils;

fn handle_hsl(p: f32, q: f32, t: f32) -> f32 {
	let pi = p;
	let qi = q;
	let mut ti = t;

	if ti < 0f32 {
		ti += 1f32;
	}
	if ti > 1f32 {
		ti -= 1f32;
	}
	if ti < 0.166666666 {
		return pi + (qi - pi) * 6f32 * ti;
	}
	if ti < 0.5 {
		return qi;
	}
	if ti < 0.666666666 {
		return pi + (qi - pi) * (0.666666666 - ti) * 6f32;
	}

	pi
}

// hsl -> hsl(66.23,76.24%,19.8%); hsla(66.23,76.24%,19.8%,1) etc..
// return -> [0.81,0.89,0.12,1],[0.81,0.89,0.12],[0.81,0.89,0.12,0.3] etc...
pub fn hsl2rgb(color: &Color) -> Result<String, Error> {
	let re = Regex::new(r"(?i)hsla?\(\s*(?P<h>\d{1,3}\.?\d*%?)\s*,\s*(?P<s>\d{1,3}\.?\d*%?)\s*,\s*(?P<l>\d{1,3}\.?\d*%?)\s*,?\s*(?P<alpha>\.?\d{1,3}\.?\d*%?)?\s*\)").expect("Parse hsl value error");
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		let mut h = utils::convert_alpha_value_to_number(&value["h"]);
		let mut s = utils::convert_alpha_value_to_number(&value["s"]);
		let mut l = utils::convert_alpha_value_to_number(&value["l"]);
		let mut alpha = 1f32;

		let r;
		let g;
		let b;

		let match_alpha = value.get(4);
		if let Some(value) = match_alpha {
			alpha = utils::convert_alpha_value_to_number(value.as_str());
		}

		if h >= 360f32 {
			h -= 360f32;
		}
		h = h / 360f32;

		if s == 0f32 {
			r = l;
			g = l;
			b = l;
		} else {
			let q;
			let p;
			if l < 0.5f32 {
				q = l * (1f32 + s);
			} else {
				q = l + s - l * s;
			}
			p = 2f32 * l - q;
			r = utils::round(handle_hsl(p, q, h + 0.33333333) * 255f32, 2);
			g = utils::round(handle_hsl(p, q, h) * 255f32, 2);
			b = utils::round(handle_hsl(p, q, h - 0.33333333) * 255f32, 2);
		}

		let rgb;
		if color.alpha {
			rgb = format!("rgba({},{},{},{})", r, g, b, alpha);
		} else {
			rgb = format!("rgb({},{},{})", r, g, b);
		}

		return Ok(if color.upper {rgb.to_uppercase()} else {rgb});
	}

	Err(Error::Format)
}

pub fn hsl2hex(color: &Color) -> Result<String, Error> {
	let rgb = color.to_rgb()?;
	let new_color = &color.copy(&rgb);

	Ok(new_color.to_hex()?)
}

pub fn hsl2hsl(color: &Color) -> Result<String, Error> {
	let re = Regex::new(r"(?i)hsla?\(\s*(?P<h>\d{1,3}\.?\d*%?)\s*,\s*(?P<s>\d{1,3}\.?\d*%?)\s*,\s*(?P<l>\d{1,3}\.?\d*%?)\s*,?\s*(?P<alpha>\.?\d{1,3}\.?\d*%?)?\s*\)").expect("Parse hsl value error");
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		let mut alpha = 1f32;

		if let Some(value) = value.get(4) {
			alpha = utils::convert_alpha_value_to_number(value.as_str());
		}

		let hsl;
		if color.alpha {
			hsl = format!("hsla({},{},{},{})", &value["h"], &value["s"], &value["l"], alpha);
		} else {
			hsl = format!("hsl({},{},{})", &value["h"], &value["s"], &value["l"]);
		}

		return Ok(if color.upper {hsl.to_uppercase()} else {hsl});
	}

	Err(Error::Format)
}

pub fn hsl2cmyk(color: &Color) -> Result<String, Error> {
	let rgb = color.to_rgb()?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_cmyk()?)
}

pub fn hsl2hsv(color: &Color) -> Result<String, Error> {
	let rgb = color.to_rgb()?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_hsv()?)
}