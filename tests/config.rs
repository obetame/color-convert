extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::config::cc_config;
	
	#[test]
	fn test_config() {
		let config_struct = cc_config::Setting::new("rgb", false, false, false);
		println!("{:?}", config_struct);
	}
}