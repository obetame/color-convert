// color convert setting
pub mod cc_config {
	#[derive(Debug)]
	pub struct Setting {
		pub convert_mode: String,
		pub capitization: bool,
		pub is_android: bool,
		pub loss_transparent: bool
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
	// #[derive(Eq)]
	pub enum ColorMode<'a> {
		RGBA(&'a str),
		RGB(&'a str),
		HSLA(&'a str),
		HSL(&'a str),
		CMYK(&'a str),
		HSV(&'a str),
		HEX(&'a str)
	}

	impl<'a> PartialEq for ColorMode<'a> {
		fn eq(&self, other: &ColorMode) -> bool {
			match (self, other) {
				(ColorMode::RGBA(i), ColorMode::RGBA(j)) => i == j,
				(ColorMode::RGB(i), ColorMode::RGB(j)) => i == j,
				(ColorMode::HSLA(i), ColorMode::HSLA(j)) => i == j,
				(ColorMode::HSL(i), ColorMode::HSL(j)) => i == j,
				(ColorMode::CMYK(i), ColorMode::CMYK(j)) => i == j,
				(ColorMode::HSV(i), ColorMode::HSV(j)) => i == j,
				(ColorMode::HEX(i), ColorMode::HEX(j)) => i == j,
				_ => false
			}
		}
	}
}