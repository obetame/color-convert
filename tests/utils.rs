extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::utils::common;
	use color_convert::config::color_mode;
	
	#[test]
	fn test_utils_get_color_mode() {
		let data = common::get_color_mode("rgba(1,2,3,.9)");
		let data1 = common::get_color_mode("hsla(1,2,3,.9)");
		
		assert_eq!(color_mode::ColorMode::RGBA("rgba(1,2,3,.9)"), data);
		assert_eq!(color_mode::ColorMode::HSLA("hsla(1,2,3,.9)"), data1);
		
		assert!(color_mode::ColorMode::RGB("rgba(1,2,3,.9)") != data);
		assert!(color_mode::ColorMode::HSL("hsla(1,2,3,.9)") != data1);
	}
}