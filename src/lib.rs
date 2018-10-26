pub mod color;
pub mod handles;
pub mod utils;

pub use color::*;

pub fn convert(color: &str, to_mode: &str) -> String {
	let _color_string = color.to_string();
	let _to_mode = to_mode.to_string();

	let to_color = String::from("zhouyuexie");
	to_color
}