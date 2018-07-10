extern crate color_convert;

#[cfg(test)]
mod tests {
	use color_convert::handles::map_name;

	#[test]
	fn test_map_color_name() {
		let color = "Sienna";
		let to_color = map_name::map_color_name(&color);
		println!("{}", to_color);
	}

	#[test]
	fn test_map_color_name_uper() {
		let color = "sienna";
		let to_color = map_name::map_name_to_name(&color);
		println!("{}", to_color);
	}

	#[test]
	fn test_map_hex() {
		let hex = 'B';
		let to_char = map_name::map_hex(&hex);
		println!("{}", to_char);
	}

	#[test]
	#[should_panic]
	fn test_map_rgb() {
		let number = 20;
		let to_char = map_name::map_rgb(number);
		println!("{}", to_char);
	}
}