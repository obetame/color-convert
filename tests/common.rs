extern crate color_convert;
use color_convert::color::Color;

// create all possible color config
pub fn init_color(hsl: Vec<&str>) -> Vec<Vec<Color>> {
	let mut test_color = vec![];
	for i in 0..hsl.len() {
		let mut result = vec![];
		result.push(Color::new(hsl[i], false, false, false));
		result.push(Color::new(hsl[i], true, false, false));
		result.push(Color::new(hsl[i], false, true, false));
		result.push(Color::new(hsl[i], false, false, true));
		result.push(Color::new(hsl[i], true, true, false));
		result.push(Color::new(hsl[i], true, false, true));
		result.push(Color::new(hsl[i], false, true, true));
		result.push(Color::new(hsl[i], true, true, true));

		test_color.push(result);
	}
	test_color
}