use regex::Regex;
use regex::Captures;
use color::{Color, Error};
use utils;

// rgb -- rgba(81%,89%,12%,1),rgb(81,89,12),rgba(81%,89%,12%,0.3) etc..
// return -- [0.81,0.89,0.12,1],[0.81,0.89,0.12] [0.81,0.89,0.12,0.3] etc...
pub fn handle_rgb<'a>(color: &'a Color) -> Result<Captures<'a>, Error> {
	let re = Regex::new(r"rgba?\(\s*(?P<r>\d{1,3}\.?\d*%?)\s*,\s*(?P<g>\d{1,3}\.?\d*%?)\s*,\s*(?P<b>\d{1,3}\.?\d*%?)\s*,?\s*(?P<alpha>\.?\d{1,3}\.?\d*%?)?\s*\)").unwrap();
	let cap = re.captures(color.to_str());

	if let Some(value) = cap {
		println!("{}", utils::convert_value_to_number(&value["r"]));
		return Ok(value)
	}

	Err(Error::Format)
}

pub fn to_hsl(color: &Color) -> Result<Vec<f32>, Error> {
	let cap = handle_rgb(&color)?;
	println!("{:?}", cap);

	Ok(vec![3f32])
}