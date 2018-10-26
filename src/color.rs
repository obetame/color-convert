use handles::hex;

// color define
#[derive(Debug)]
pub struct Color<'a> {
	pub to_mode: ColorMode<'a>,
	pub is_upper: bool,
	pub is_android: bool,
	pub is_alpha: bool
}
impl<'a> Color<'a> {
	// new one Color instance
	pub fn new(mode: &'a str, is_upper: bool, is_android: bool, is_alpha: bool) -> Self {
		Color {
			to_mode: ColorMode::new(mode),
			is_upper,
			is_android,
			is_alpha
		}
	}

	// config Color config [is_upper, is_android, is_alpha]
	pub fn upper(&mut self, is_upper: bool) -> &Self {
		self.is_upper = is_upper;
		self
	}
	pub fn android(&mut self, is_android: bool) -> &Self {
		self.is_android = is_android;
		self
	}
	pub fn alpha(&mut self, is_alpha: bool) -> &Self {
		self.is_alpha = is_alpha;
		self
	}

	pub fn to_rgb(&self) -> Result<String, &'static str> {
		match self.to_mode {
			ColorMode::HEX(color) => hex::hex2rgb(color, self),
			_ => Err("noting to match")
		}
	}
}

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

impl<'a> ColorMode<'a> {
	pub fn new(color: &'a str) -> ColorMode {
		let upper_color = color.to_ascii_uppercase();

		if upper_color.contains("RGBA") {
			ColorMode::RGBA(color)
		} else if upper_color.contains("RGB") {
			ColorMode::RGB(color)
		} else if upper_color.contains("HSLA") {
			ColorMode::HSLA(color)
		} else if upper_color.contains("HSL") {
			ColorMode::HSL(color)
		} else if upper_color.contains("CMYK") {
			ColorMode::CMYK(color)
		} else if upper_color.contains("HSV") {
			ColorMode::HSV(color)
		} else {
			ColorMode::HEX(color)
		}
	}
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