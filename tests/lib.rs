extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert;

	#[test]
	fn rgb_to_hex() {
		let _config_struct = color_convert::convert("rgb(0,0,0)", "hex");
	}
}