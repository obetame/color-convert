extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert;
	use color_convert::Color;

	#[test]
	fn rgb_to_hex() {
		let _hex = Color::new("rgb(0,0,0)", true, false, true);
		let _to_hex = color_convert::convert("rgb(0,0,0)", "hex");
		let _to_rgb = color_convert::convert("#fff", "rgb");
	}
}