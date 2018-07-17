extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::config::cc_config;
	
	#[test]
	fn test_config() {
		let config_struct = cc_config::Setting::new("rgb", false, false, false);
		assert!(config_struct.convert_mode == "rgb");
		assert!(config_struct.capitization == false);
		assert!(config_struct.is_android == false);
		assert!(config_struct.loss_transparent == false);
	}
}