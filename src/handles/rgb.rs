use regex::Regex;
use regex::Captures;
use color::Color;

// rgb -- rgba(81%,89%,12%,1),rgb(81,89,12),rgba(81%,89%,12%,0.3) etc..
// return -- [0.81,0.89,0.12,1],[0.81,0.89,0.12] [0.81,0.89,0.12,0.3] etc...
pub fn handle_rgb<'a>(color: &'a Color) -> Result<Captures<'a>, String> {
	let re = Regex::new(r"rgba?\((?P<r>\d{1,3}\.?\d*%?),(?P<g>\d{1,3}\.?\d*%?),(?P<b>\d{1,3}\.?\d*%?),?(?P<alpha>\d{1,3}\.?\d*%?)?\)").unwrap();
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		Ok(value)
	} else {
		Err(format!("[color-convert] noting to match: {}", color.to_str()))
	}
}

pub fn to_hsl(color: &Color) -> Result<Vec<f32>, String> {
	let cap = handle_rgb(&color)?;
	println!("{:?}", cap);

	Ok(vec![3f32])
}