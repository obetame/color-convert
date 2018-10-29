extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::color::Color;
	//	use color_convert::handles::map;
	use color_convert::handles::rgb;
//	use color_convert::utils;

	#[test]
	fn test_rgb_handle() {
		let color = Color::init("rgba(81%,89%,12%,1)");
		let cap = rgb::handle_rgb(&color);
		println!("{:?}", cap);
	}
}