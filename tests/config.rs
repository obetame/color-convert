extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::config::Setting;
	
	#[test]
	fn test_config() {
		let config_struct = Setting::new("rgb", false, false, false);
		assert_eq!(config_struct.convert_mode, "rgb");
		assert_eq!(config_struct.is_capitization, false);
		assert_eq!(config_struct.is_android, false);
		assert_eq!(config_struct.loss_transparent, false);
	}
}