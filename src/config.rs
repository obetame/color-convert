// color convert setting
pub mod cc_config {
	#[derive(Debug)]
	pub struct Setting {
		convert_mode: String,
		capitization: bool,
		is_android: bool,
		loss_transparent: bool
	}

	impl Setting {
		pub fn new(mode: &str, upper: bool, is_android: bool, loss_transparent: bool) -> Setting {
			Setting {
				convert_mode: mode.to_string(),
				capitization: upper,
				is_android: is_android,
				loss_transparent: loss_transparent
			}
		}
	}
}

pub mod color_mode {
	#[derive(Debug)]
	pub enum ColorMode {
		RGBA(String),
		RGB(String),
		HSLA(String),
		HSL(String),
		CMYK(String),
		HSV(String),
		HEX(String)
	}
}