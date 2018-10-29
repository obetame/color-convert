use regex::Regex;
use regex::Captures;
use color::Color;

// rgb -- rgba(81%,89%,12%,1),rgb(81,89,12),rgba(81%,89%,12%,0.3) etc..
// return -- [0.81,0.89,0.12,1],[0.81,0.89,0.12] [0.81,0.89,0.12,0.3] etc...
pub fn handle_rgb<'a>(color: &'a Color) -> Captures<'a> {
	let re = Regex::new(r"(\d{1,3}\.?\d*%?),(\d{1,3}\.?\d*%?),(\d{1,3}\.?\d*%?),?(\d{1,3}\.?\d*%?)?").unwrap();

	let cap: Captures = re.captures(color.to_str()).unwrap();
	cap
}