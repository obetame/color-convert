extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {
	use color_convert::color::{Color};
	use color_convert::handles::rgb;
	use common::init_color;

	#[test]
	fn test_hsv2rgb() {
		let hsv_vec = vec!["hsl(66.23,76.24%,19.8%)", "hsla(66.23,76.24%,19.8%,1)"];
//		let test_result = vec![];

		let test_color = init_color(hsv_vec);
		for (i, vec_color) in test_color.iter().enumerate() {
			print!("vec![");
			for (index, color) in vec_color.iter().enumerate() {
//				assert_eq!(rgb_result[i][index], color.to_hex().unwrap());
				print!("{:?}, ", color.to_rgb().unwrap());
			}
			println!("],");
		}
	}
}