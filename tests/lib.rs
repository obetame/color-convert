extern crate color_convert;

#[cfg(test)]
mod test {
	use color_convert::color::{Color};

	#[test]
	fn test_lib() {
		let mut color = Color::init("#c8c8c8ff", false, false, false);

		assert_eq!("rgb(200,200,200)", color.to_rgb().unwrap());
		assert_eq!("RGB(200,200,200)", color.to_upper(true).to_rgb().unwrap());
		assert_eq!("RGBA(200,200,200,1)", color.to_alpha(true).to_rgb().unwrap());
		assert_eq!("RGBA(200,200,255,0.78)", color.to_android(true).to_rgb().unwrap());
	}
}