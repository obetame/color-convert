extern crate color_convert;
use color_convert::color::Color;

// create all possible color config
pub fn init_color(color: Vec<&str>) -> Vec<Vec<Color>> {
	let mut test_color = vec![];
	for i in 0..color.len() {
		let mut result = vec![];
		result.push(Color::new(color[i], false, false, false));
		result.push(Color::new(color[i], true, false, false));
		result.push(Color::new(color[i], false, true, false));
		result.push(Color::new(color[i], false, false, true));
		result.push(Color::new(color[i], true, true, false));
		result.push(Color::new(color[i], true, false, true));
		result.push(Color::new(color[i], false, true, true));
		result.push(Color::new(color[i], true, true, true));

		test_color.push(result);
	}
	test_color
}