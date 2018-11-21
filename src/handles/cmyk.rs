use regex::Regex;
use color::{Color, Error};
use utils;

pub fn cmyk2rgb(color: &Color) -> Result<String, Error> {
	let re = Regex::new(r"(?i)cmyk\(\s*(?P<c>\d{1,3}\.?\d*)\s*,\s*(?P<m>\d{1,3}\.?\d*)\s*,\s*(?P<y>\d{1,3}\.?\d*)\s*,?\s*(?P<k>\.?\d{1,3}\.?\d*)\s*\)").expect("Parse hsl value error");
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		let c = utils::convert_alpha_value_to_number(&value["c"]);
		let y = utils::convert_alpha_value_to_number(&value["y"]);
		let m = utils::convert_alpha_value_to_number(&value["m"]);
		let k = utils::convert_alpha_value_to_number(&value["k"]);

		let r = utils::round(255f32 * (1f32 - c) * (1f32 - k), 0);
		let g = utils::round(255f32 * (1f32 - m) * (1f32 - k), 0);
		let b = utils::round(255f32 * (1f32 - y) * (1f32 - k), 0);

		let rgb = if color.alpha {
			format!("rgba({},{},{},1)", r, g, b)
		} else {
			format!("rgb({},{},{})", r, g, b)
		};

		return Ok(if color.upper {rgb.to_uppercase()} else {rgb});
	}

	Err(Error::Format)
}

pub fn cmyk2hex(color: &Color) -> Result<String, Error> {
	let rgb = cmyk2rgb(&color)?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_hex()?)
}

pub fn cmyk2hsl(color: &Color) -> Result<String, Error> {
	let rgb = cmyk2rgb(&color)?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_hsl()?)
}

pub fn cmyk2cmyk(color: &Color) -> Result<String, Error> {
	let _ = cmyk2rgb(&color)?; // check format
	let cmyk = color.to_str();

	Ok(if color.upper {cmyk.to_uppercase()} else {cmyk.to_lowercase()})
}

pub fn cmyk2hsv(color: &Color) -> Result<String, Error> {
	let rgb = cmyk2rgb(&color)?;
	let rgb_color = color.copy(&rgb);

	Ok(rgb_color.to_hsv()?)
}