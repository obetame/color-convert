extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::config::cc_config;
	
	#[test]
	fn test_config() {
		let config_struct = cc_config::Setting::new("rgb", false, false, false);
		assert_eq!(config_struct.convert_mode, "rgb");
		assert_eq!(config_struct.capitization, false);
		assert_eq!(config_struct.is_android, false);
		assert_eq!(config_struct.loss_transparent, false);
	}
}