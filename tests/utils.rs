extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::utils;
	use color_convert::color::Color;
	use color_convert::handles::hex;
	use std::panic;

	#[test]
	fn test_utils_get_hex_alpha_value() {
		let hex = "#80ffffff";
		let hex1 = "#ffffff80";
		let setting = Color::new("rgb", false, true, false);
		let setting1 = Color::new("rgb", false, false, false);
		let hex_vec: Vec<&str> = hex::handle_hex_value(&hex1, &setting1).unwrap();
		let hex_vec1: Vec<&str> = hex::handle_hex_value(&hex, &setting).unwrap();

		let data = utils::get_hex_alpha_value(&hex_vec, &setting);
		let data1 = utils::get_hex_alpha_value(&hex_vec1, &setting1);

		assert_eq!(1f32, data);
		assert_eq!(0.502f32, data1);
	}

	#[test]
	fn test_utils_get_rgba_alpha_value() {
		let data = utils::get_rgba_alpha_value("rgba(1,1,1,.5)").unwrap();
		let data1 = utils::get_rgba_alpha_value("rgba(1,1,1,0.5)").unwrap();
		let data2 = utils::get_rgba_alpha_value("rgb(1,1,1)").unwrap();

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
			let _data1 = utils::get_rgba_alpha_value("rgba(1,1)").unwrap();
			let _data = utils::get_rgba_alpha_value("rgba(1,1,1,b)").unwrap();
		});
		assert!(result.is_err());
	}

	#[test]
	fn test_handel_alpha_to_hexadecimal() {
		let alpha = utils::get_rgba_alpha_value("rgba(1,1,1,.5)").unwrap();

		let hex_alpha = utils::handel_alpha_to_hexadecimal(alpha);

		assert_eq!(hex_alpha, String::from("80"));
	}
}