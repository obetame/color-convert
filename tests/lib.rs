extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert;

	#[test]
	fn rgb_to_hex() {
		let _to_hex = color_convert::convert("rgb(0,0,0)", "hex");
		let _to_rgb = color_convert::convert("#fff", "rgb");
	}
}