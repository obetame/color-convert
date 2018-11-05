extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::color::{Color};
	//	use color_convert::handles::map;
	use color_convert::handles::rgb;
//	use color_convert::utils;

//	#[test]
//	fn test_rgb_handle() {
//		let color = Color::init("rgba");
//		let cap = rgb::handle_rgb(&color);
//
//		if let Captures(value) = cap {
//			println!("{:?}", value);
//		} else {
//			println!("noting");
//		}
//	}

	#[test]
	fn test_rgb_tohsl() {
		let color = Color::init("rgb( 81% , 89% , 10%)");
		let _color1 = Color::init("rgb(81%,89%,10%,0.5)");
		let rgb_vec = rgb::to_hsl(&color);

		if let Err(error) = rgb_vec {
			println!("error: {}", error);
		} else {
			println!("ok: {:?}", rgb_vec);
		}
	}
}