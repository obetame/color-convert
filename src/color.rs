use handles::{hex, rgb, hsl, cmyk, hsv};
use std::fmt;
use std::error::Error as StdError;

// color define
#[derive(Debug)]
pub struct Color<'a> {
	pub mode: ColorMode<'a>,
	pub to_upper: bool,
	pub to_android: bool,
	pub to_alpha: bool
}
impl<'a> Color<'a> {
	// new one Color instance by all setting
	pub fn new(mode: &'a str, to_upper: bool, to_android: bool, to_alpha: bool) -> Self {
		Color {
			mode: ColorMode::new(mode),
			to_upper,
			to_android,
			to_alpha
		}
	}
	// new one color instance by default config
	pub fn init(mode: &'a str) -> Self {
		Color {
			mode: ColorMode::new(mode),
			to_upper: false,
			to_android: false,
			to_alpha: false
		}
	}
	// init new color from copy other colorValue
	pub fn copy(&self, mode: &'a str) -> Self {
		Color {
			mode: ColorMode::new(mode),
			to_upper: self.to_upper,
			to_android: self.to_android,
			to_alpha: self.to_alpha
		}
	}

	// config Color config [to_upper, to_android, to_alpha]
	pub fn upper(&mut self, to_upper: bool) -> &Self {
		self.to_upper = to_upper;
		self
	}
	pub fn android(&mut self, to_android: bool) -> &Self {
		self.to_android = to_android;
		self
	}
	pub fn alpha(&mut self, to_alpha: bool) -> &Self {
		self.to_alpha = to_alpha;
		self
	}
	pub fn to_str(&self) -> &str {
		match self.mode {
			ColorMode::HEX(color) => color,
			ColorMode::RGB(color) => color,
			ColorMode::RGBA(color) => color,
			ColorMode::HSL(color) => color,
			ColorMode::HSLA(color) => color,
			ColorMode::HSV(color) => color,
			ColorMode::CMYK(color) => color,
		}
	}

	pub fn to_rgb(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2rgb(self),
			ColorMode::RGB(_) => rgb::rgb2rgb(self),
			ColorMode::RGBA(_) => rgb::rgb2rgb(self),
			ColorMode::HSL(_) => hsl::hsl2rgb(self),
			ColorMode::HSLA(_) => hsl::hsl2rgb(self),
			ColorMode::CMYK(_) => cmyk::cmyk2rgb(self),
			ColorMode::HSV(_) => hsv::hsv2rgb(self),
		}
	}

	pub fn to_hex(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2hex(self),
			ColorMode::RGB(_) => rgb::rgb2hex(self),
			ColorMode::RGBA(_) => rgb::rgb2hex(self),
			ColorMode::HSL(_) => hsl::hsl2hex(self),
			ColorMode::HSLA(_) => hsl::hsl2hex(self),
			ColorMode::CMYK(_) => cmyk::cmyk2hex(self),
			ColorMode::HSV(_) => hsv::hsv2hex(self),
		}
	}

	pub fn to_hsl(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2hsl(self),
			ColorMode::RGB(_) => rgb::rgb2hsl(self),
			ColorMode::RGBA(_) => rgb::rgb2hsl(self),
			ColorMode::HSL(_) => hsl::hsl2hsl(self),
			ColorMode::HSLA(_) => hsl::hsl2hsl(self),
			ColorMode::CMYK(_) => cmyk::cmyk2hsl(self),
			ColorMode::HSV(_) => hsv::hsv2hsl(self),
		}
	}

	pub fn to_cmyk(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2cmyk(self),
			ColorMode::RGB(_) => rgb::rgb2cmyk(self),
			ColorMode::RGBA(_) => rgb::rgb2cmyk(self),
			ColorMode::HSL(_) => hsl::hsl2cmyk(self),
			ColorMode::HSLA(_) => hsl::hsl2cmyk(self),
			ColorMode::CMYK(_) => cmyk::cmyk2cmyk(self),
			ColorMode::HSV(_) => hsv::hsv2cmyk(self),
		}
	}

	pub fn to_hsv(&self) -> Result<String, Error> {
		match self.mode {
			ColorMode::HEX(_) => hex::hex2hsl(self),
			ColorMode::RGB(_) => rgb::rgb2hsv(self),
			ColorMode::RGBA(_) => rgb::rgb2hsv(self),
			ColorMode::HSL(_) => hsl::hsl2hsv(self),
			ColorMode::HSLA(_) => hsl::hsl2hsv(self),
			ColorMode::CMYK(_) => cmyk::cmyk2hsv(self),
			ColorMode::HSV(_) => hsv::hsv2hsv(self),
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

#[derive(Debug)]
pub enum Error {
	Format,
	NotMatch,
	RgbAlphaFormat
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Format => write!(f, "Format Error"),
			Error::NotMatch => write!(f, "NotMatch Color"),
			Error::RgbAlphaFormat => write!(f, "Rgba Alpha Format Error")
		}
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Format => "[color-convert] Color value is illegal.",
			Error::NotMatch => "[color-convert] No match color value.",
			Error::RgbAlphaFormat => "alpha value is not a number, unable to convert to f32"
		}
	}
}