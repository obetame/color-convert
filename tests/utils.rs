extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::utils::common;
	
	#[test]
	fn test_utils_get_color_mode() {
		let data = common::get_color_mode("rgba(1,2,3,.9)");
		let data1 = common::get_color_mode("hsla(1,2,3,.9)");
		println!("{:?}, {:?}", data, data1);
	}
}