extern crate regex;

pub mod color;
pub mod handles;
pub mod utils;

pub use color::*;

pub fn convert(color: &str, mode: &str) -> String {
	let _color_string = color.to_string();
	let _to_mode = mode.to_string();

	let to_color = String::from("zhouyuexie");
	to_color
}