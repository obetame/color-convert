extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::utils::common;
	use color_convert::config::color_mode;
	use color_convert::config::cc_config::Setting;
	use color_convert::handles::handle;
	
	#[test]
	fn test_utils_get_color_mode() {
		let data = common::get_color_mode("rgba(1,2,3,.9)");
		let data1 = common::get_color_mode("hsla(1,2,3,.9)");
		
		assert_eq!(color_mode::ColorMode::RGBA("rgba(1,2,3,.9)"), data);
		assert_eq!(color_mode::ColorMode::HSLA("hsla(1,2,3,.9)"), data1);
		
		assert!(color_mode::ColorMode::RGB("rgba(1,2,3,.9)") != data);
		assert!(color_mode::ColorMode::HSL("hsla(1,2,3,.9)") != data1);
	}

	#[test]
	fn test_utils_get_hex_alpha_value() {
		let hex = "#80ffffff";
		let hex1 = "#ffffff80";
		let setting = Setting::new("rgb", false, true, false);
		let setting1 = Setting::new("rgb", false, false, false);
		let hex_vec: Vec<&str> = handle::handle_hex_value(&hex1, &setting1);
		let hex_vec1: Vec<&str> = handle::handle_hex_value(&hex, &setting);

		let data = common::get_hex_alpha_value(&hex_vec, &setting);
		let data1 = common::get_hex_alpha_value(&hex_vec1, &setting1);

		assert_eq!(1f32, data);
		assert_eq!(0.502f32, data1);
	}

	#[test]
	fn test_utils_get_rgba_alpha_value() {
		let data = common::get_rgba_alpha_value("rgba(1,1,1,.5)");
		let data1 = common::get_rgba_alpha_value("rgba(1,1,1,0.5)");
		let data2 = common::get_rgba_alpha_value("rgb(1,1,1)");

		assert_eq!(data, 0.5);
		assert_eq!(data1, 0.5);
		assert_eq!(data2, 1f32);
	}

	#[test]
	#[should_panic]
	fn test_utils_get_rgba_alpha_value_fail() {
		let data = common::get_rgba_alpha_value("rgba(1,1,1,b)");
		let data1 = common::get_rgba_alpha_value("rgba(1,1)");
	}
}