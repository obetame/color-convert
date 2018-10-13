extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::utils;
	use color_convert::config::ColorMode;
	use color_convert::config::Setting;
	use color_convert::handles::hex;
	use std::panic;
	
	#[test]
	fn test_utils_get_color_mode() {
		let data = utils::get_color_mode("rgba(1,2,3,.9)");
		let data1 = utils::get_color_mode("hsla(1,2,3,.9)");
		
		assert_eq!(ColorMode::RGBA("rgba(1,2,3,.9)"), data);
		assert_eq!(ColorMode::HSLA("hsla(1,2,3,.9)"), data1);
		
		assert_ne!(ColorMode::RGB("rgba(1,2,3,.9)"), data);
		assert_ne!(ColorMode::HSL("hsla(1,2,3,.9)"), data1);
	}

	#[test]
	fn test_utils_get_hex_alpha_value() {
		let hex = "#80ffffff";
		let hex1 = "#ffffff80";
		let setting = Setting::new("rgb", false, true, false);
		let setting1 = Setting::new("rgb", false, false, false);
		let hex_vec: Vec<&str> = hex::handle_hex_value(&hex1, &setting1);
		let hex_vec1: Vec<&str> = hex::handle_hex_value(&hex, &setting);

		let data = utils::get_hex_alpha_value(&hex_vec, &setting);
		let data1 = utils::get_hex_alpha_value(&hex_vec1, &setting1);

		assert_eq!(1f32, data);
		assert_eq!(0.502f32, data1);
	}

	#[test]
	fn test_utils_get_rgba_alpha_value() {
		let data = utils::get_rgba_alpha_value("rgba(1,1,1,.5)");
		let data1 = utils::get_rgba_alpha_value("rgba(1,1,1,0.5)");
		let data2 = utils::get_rgba_alpha_value("rgb(1,1,1)");

		assert_eq!(data, 0.5);
		assert_eq!(data1, 0.5);
		assert_eq!(data2, 1f32);
	}

	#[test]
	fn test_utils_get_rgba_alpha_value_fail() {
		panic::set_hook(Box::new(|_info| {
			println!("catch error args")
		}));

		let result = panic::catch_unwind(|| {
			let _data1 = utils::get_rgba_alpha_value("rgba(1,1)");
			let _data = utils::get_rgba_alpha_value("rgba(1,1,1,b)");
		});
		assert!(result.is_err());
	}
}