use regex::{Regex, Captures};
use color::{Color, Error};
use utils;

pub fn handle_hsv(hsv: &str) -> Option<Captures> {
	let re = Regex::new(r"(?i)hsv\(\s*(?P<h>\d{1,3}\.?\d*)\s*,\s*(?P<s>\d{1,3}\.?\d*%?)\s*,\s*(?P<v>\d{1,3}\.?\d*%?)\s*\)").expect("Parse hsl value error");
	let cap = re.captures(&hsv);

	cap
}

pub fn hsv2rgb(color: &Color) -> Result<String, Error> {
	let cap = handle_hsv(color.to_str());

	if let Some(value) = cap {
		let mut h = utils::convert_alpha_value_to_number(&value["h"]);
		let s = utils::convert_alpha_value_to_number(&value["s"]);
		let v = utils::convert_alpha_value_to_number(&value["v"]);

		if h >= 360f32 {
			h -= 360f32;
		}

		let c = v * s;
		let x = c * (1f32 - ((h / 60f32) % 2f32 - 1f32).abs());
		let m = v - c;

		let mut r = 0f32;
		let mut g = 0f32;
		let mut b = 0f32;

		if h >= 0f32 && h < 60f32 {
			r = c;
			g = x;
			b = 0f32;
		} else if h >= 60f32 && h < 120f32 {
			r = x;
			g = c;
			b = 0f32;
		} else if h >= 120f32 && h < 180f32 {
			r = 0f32;
			g = c;
			b = c;
		} else if h >= 180f32 && h < 240f32 {
			r = 0f32;
			g = x;
			b = c;
		} else if h >= 240f32 && h < 300f32 {
			r = x;
			g = 0f32;
			b = c;
		} else if h >= 300f32 && h < 360f32 {
			r = c;
			g = 0f32;
			b = x;
		}

		r = utils::round((r + m) * 255f32, 2);
		g = utils::round((g + m) * 255f32, 2);
		b = utils::round((b + m) * 255f32, 2);

		let rgb = if color.to_alpha {
			format!("rgba({},{},{},1)", r, g, b)
		} else {
			format!("rgb({},{},{})", r, g, b)
		};

		return Ok(if color.to_upper {rgb.to_uppercase()} else {rgb});
	}

	Err(Error::Format)
}

pub fn hsv2hex(color: &Color) -> Result<String, Error> {
	let rgb = hsv2rgb(&color)?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_hex()?)
}

pub fn hsv2hsl(color: &Color) -> Result<String, Error> {
	let rgb = hsv2rgb(&color)?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_hsl()?)
}

pub fn hsv2cmyk(color: &Color) -> Result<String, Error> {
	let rgb = hsv2rgb(&color)?; // check format
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_cmyk()?)
}

pub fn hsv2hsv(color: &Color) -> Result<String, Error> {
	let hsv = color.to_str();
	let cap = handle_hsv(hsv);

	if let Some(_) = cap {
		let hsv = hsv.to_owned();
		return Ok(if color.to_upper {hsv.to_uppercase()} else {hsv});
	}

	Err(Error::Format)
}