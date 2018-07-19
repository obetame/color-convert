// use std::collections::HashMap;

// const COLOR_NAME: HashMap<&str, i32> = [
// 	("Norway", 100),
// 	("Denmark", 50),
// 	("Iceland", 10)
// ].iter().cloned().collect();

pub mod map_name {
	// color name convert to #ffffff
	pub fn map_color_name(color_name: &str) -> &str {
		let to_color = match color_name {
			"AliceBlue" => "#F0F8FF",
			"AntiqueWhite" => "#FAEBD7",
			"Aqua" => "#00FFFF",
			"Aquamarine" => "#7FFFD4",
			"Azure" => "#F0FFFF",
			"Beige" => "#F5F5DC",
			"Bisque" => "#FFE4C4",
			"Black" => "#000000",
			"BlanchedAlmond" => "#FFEBCD",
			"Blue" => "#0000FF",
			"BlueViolet" => "#8A2BE2",
			"Brown" => "#A52A2A",
			"BurlyWood" => "#DEB887",
			"CadetBlue" => "#5F9EA0",
			"Chartreuse" => "#7FFF00",
			"Chocolate" => "#D2691E",
			"Coral" => "#FF7F50",
			"CornflowerBlue" => "#6495ED",
			"Cornsilk" => "#FFF8DC",
			"Crimson" => "#DC143C",
			"Cyan" => "#00FFFF",
			"DarkBlue" => "#00008B",
			"DarkCyan" => "#008B8B",
			"DarkGoldenRod" => "#B8860B",
			"DarkGray" => "#A9A9A9",
			"DarkGreen" => "#006400",
			"DarkKhaki" => "#BDB76B",
			"DarkMagenta" => "#8B008B",
			"DarkOliveGreen" => "#556B2F",
			"Darkorange" => "#FF8C00",
			"DarkOrchid" => "#9932CC",
			"DarkRed" => "#8B0000",
			"DarkSalmon" => "#E9967A",
			"DarkSeaGreen" => "#8FBC8F",
			"DarkSlateBlue" => "#483D8B",
			"DarkSlateGray" => "#2F4F4F",
			"DarkTurquoise" => "#00CED1",
			"DarkViolet" => "#9400D3",
			"DeepPink" => "#FF1493",
			"DeepSkyBlue" => "#00BFFF",
			"DimGray" => "#696969",
			"DodgerBlue" => "#1E90FF",
			"Feldspar" => "#D19275",
			"FireBrick" => "#B22222",
			"FloralWhite" => "#FFFAF0",
			"ForestGreen" => "#228B22",
			"Fuchsia" => "#FF00FF",
			"Gainsboro" => "#DCDCDC",
			"GhostWhite" => "#F8F8FF",
			"Gold" => "#FFD700",
			"GoldenRod" => "#DAA520",
			"Gray" => "#808080",
			"Green" => "#008000",
			"GreenYellow" => "#ADFF2F",
			"HoneyDew" => "#F0FFF0",
			"HotPink" => "#FF69B4",
			"IndianRed" => "#CD5C5C",
			"Indigo" => "#4B0082",
			"Ivory" => "#FFFFF0",
			"Khaki" => "#F0E68C",
			"Lavender" => "#E6E6FA",
			"LavenderBlush" => "#FFF0F5",
			"LawnGreen" => "#7CFC00",
			"LemonChiffon" => "#FFFACD",
			"LightBlue" => "#ADD8E6",
			"LightCoral" => "#F08080",
			"LightCyan" => "#E0FFFF",
			"LightGoldenRodYellow" => "#FAFAD2",
			"LightGrey" => "#D3D3D3",
			"LightGreen" => "#90EE90",
			"LightPink" => "#FFB6C1",
			"LightSalmon" => "#FFA07A",
			"LightSeaGreen" => "#20B2AA",
			"LightSkyBlue" => "#87CEFA",
			"LightSlateBlue" => "#8470FF",
			"LightSlateGray" => "#778899",
			"LightSteelBlue" => "#B0C4DE",
			"LightYellow" => "#FFFFE0",
			"Lime" => "#00FF00",
			"LimeGreen" => "#32CD32",
			"Linen" => "#FAF0E6",
			"Magenta" => "#FF00FF",
			"Maroon" => "#800000",
			"MediumAquaMarine" => "#66CDAA",
			"MediumBlue" => "#0000CD",
			"MediumOrchid" => "#BA55D3",
			"MediumPurple" => "#9370D8",
			"MediumSeaGreen" => "#3CB371",
			"MediumSlateBlue" => "#7B68EE",
			"MediumSpringGreen" => "#00FA9A",
			"MediumTurquoise" => "#48D1CC",
			"MediumVioletRed" => "#C71585",
			"MidnightBlue" => "#191970",
			"MintCream" => "#F5FFFA",
			"MistyRose" => "#FFE4E1",
			"Moccasin" => "#FFE4B5",
			"NavajoWhite" => "#FFDEAD",
			"Navy" => "#000080",
			"OldLace" => "#FDF5E6",
			"Olive" => "#808000",
			"OliveDrab" => "#6B8E23",
			"Orange" => "#FFA500",
			"OrangeRed" => "#FF4500",
			"Orchid" => "#DA70D6",
			"PaleGoldenRod" => "#EEE8AA",
			"PaleGreen" => "#98FB98",
			"PaleTurquoise" => "#AFEEEE",
			"PaleVioletRed" => "#D87093",
			"PapayaWhip" => "#FFEFD5",
			"PeachPuff" => "#FFDAB9",
			"Peru" => "#CD853F",
			"Pink" => "#FFC0CB",
			"Plum" => "#DDA0DD",
			"PowderBlue" => "#B0E0E6",
			"Purple" => "#800080",
			"Red" => "#FF0000",
			"RosyBrown" => "#BC8F8F",
			"RoyalBlue" => "#4169E1",
			"SaddleBrown" => "#8B4513",
			"Salmon" => "#FA8072",
			"SandyBrown" => "#F4A460",
			"SeaGreen" => "#2E8B57",
			"SeaShell" => "#FFF5EE",
			"Sienna" => "#A0522D",
			"Silver" => "#C0C0C0",
			"SkyBlue" => "#87CEEB",
			"SlateBlue" => "#6A5ACD",
			"SlateGray" => "#708090",
			"Snow" => "#FFFAFA",
			"SpringGreen" => "#00FF7F",
			"SteelBlue" => "#4682B4",
			"Tan" => "#D2B48C",
			"Teal" => "#008080",
			"Thistle" => "#D8BFD8",
			"Tomato" => "#FF6347",
			"Turquoise" => "#40E0D0",
			"Violet" => "#EE82EE",
			"VioletRed" => "#D02090",
			"Wheat" => "#F5DEB3",
			"White" => "#FFFFFF",
			"WhiteSmoke" => "#F5F5F5",
			"Yellow" => "#FFFF00",
			"YellowGreen" => "#9ACD32",
			_ => panic!("[color-convert] map_color_name not match color_name value.")
		};
		to_color
	}

	// map color name to color name
	pub fn map_name_to_name(color_name: &str) -> &str {
		let to_color_name = match color_name {
			"blue" => "Blue",
			"pink" => "Pink",
			"powderblue" => "PowderBlue",
			"darkorange" => "Darkorange",
			"saddlebrown" => "SaddleBrown",
			"slategray" => "SlateGray",
			"indianred" => "IndianRed",
			"fuchsia" => "Fuchsia",
			"snow" => "Snow",
			"lawngreen" => "LawnGreen",
			"steelblue" => "SteelBlue",
			"mediumslateblue" => "MediumSlateBlue",
			"black" => "Black",
			"aliceblue" => "AliceBlue",
			"salmon" => "Salmon",
			"crimson" => "Crimson",
			"royalblue" => "RoyalBlue",
			"white" => "White",
			"navajowhite" => "NavajoWhite",
			"cornsilk" => "Cornsilk",
			"bisque" => "Bisque",
			"palegreen" => "PaleGreen",
			"brown" => "Brown",
			"darkturquoise" => "DarkTurquoise",
			"darkgreen" => "DarkGreen",
			"mediumvioletred" => "MediumVioletRed",
			"darkviolet" => "DarkViolet",
			"darkgray" => "DarkGray",
			"darkgoldenrod" => "DarkGoldenRod",
			"mediumorchid" => "MediumOrchid",
			"chocolate" => "Chocolate",
			"purple" => "Purple",
			"papayawhip" => "PapayaWhip",
			"olive" => "Olive",
			"lightslategray" => "LightSlateGray",
			"darkmagenta" => "DarkMagenta",
			"peachpuff" => "PeachPuff",
			"tomato" => "Tomato",
			"violet" => "Violet",
			"mediumspringgreen" => "MediumSpringGreen",
			"dodgerblue" => "DodgerBlue",
			"aqua" => "Aqua",
			"hotpink" => "HotPink",
			"violetred" => "VioletRed",
			"forestgreen" => "ForestGreen",
			"lemonchiffon" => "LemonChiffon",
			"mintcream" => "MintCream",
			"seashell" => "SeaShell",
			"goldenrod" => "GoldenRod",
			"indigo" => "Indigo",
			"cornflowerblue" => "CornflowerBlue",
			"cadetblue" => "CadetBlue",
			"lightyellow" => "LightYellow",
			"darkblue" => "DarkBlue",
			"limegreen" => "LimeGreen",
			"deepskyblue" => "DeepSkyBlue",
			"darkkhaki" => "DarkKhaki",
			"lightgrey" => "LightGrey",
			"whitesmoke" => "WhiteSmoke",
			"yellow" => "Yellow",
			"gainsboro" => "Gainsboro",
			"sienna" => "Sienna",
			"lavenderblush" => "LavenderBlush",
			"sandybrown" => "SandyBrown",
			"deeppink" => "DeepPink",
			"feldspar" => "Feldspar",
			"magenta" => "Magenta",
			"silver" => "Silver",
			"lime" => "Lime",
			"darkcyan" => "DarkCyan",
			"greenyellow" => "GreenYellow",
			"darkorchid" => "DarkOrchid",
			"lightsalmon" => "LightSalmon",
			"lightgoldenrodyellow" => "LightGoldenRodYellow",
			"olivedrab" => "OliveDrab",
			"darkred" => "DarkRed",
			"lightskyblue" => "LightSkyBlue",
			"slateblue" => "SlateBlue",
			"orange" => "Orange",
			"chartreuse" => "Chartreuse",
			"maroon" => "Maroon",
			"peru" => "Peru",
			"mediumturquoise" => "MediumTurquoise",
			"aquamarine" => "Aquamarine",
			"lightcoral" => "LightCoral",
			"thistle" => "Thistle",
			"red" => "Red",
			"darkslategray" => "DarkSlateGray",
			"khaki" => "Khaki",
			"wheat" => "Wheat",
			"lightslateblue" => "LightSlateBlue",
			"lightpink" => "LightPink",
			"burlywood" => "BurlyWood",
			"lightseagreen" => "LightSeaGreen",
			"mediumblue" => "MediumBlue",
			"darksalmon" => "DarkSalmon",
			"rosybrown" => "RosyBrown",
			"blueviolet" => "BlueViolet",
			"cyan" => "Cyan",
			"mediumpurple" => "MediumPurple",
			"midnightblue" => "MidnightBlue",
			"firebrick" => "FireBrick",
			"paleturquoise" => "PaleTurquoise",
			"gray" => "Gray",
			"palevioletred" => "PaleVioletRed",
			"mediumseagreen" => "MediumSeaGreen",
			"lightblue" => "LightBlue",
			"coral" => "Coral",
			"turquoise" => "Turquoise",
			"ivory" => "Ivory",
			"darkslateblue" => "DarkSlateBlue",
			"lightgreen" => "LightGreen",
			"linen" => "Linen",
			"green" => "Green",
			"beige" => "Beige",
			"teal" => "Teal",
			"azure" => "Azure",
			"moccasin" => "Moccasin",
			"orangered" => "OrangeRed",
			"lightsteelblue" => "LightSteelBlue",
			"tan" => "Tan",
			"antiquewhite" => "AntiqueWhite",
			"palegoldenrod" => "PaleGoldenRod",
			"skyblue" => "SkyBlue",
			"ghostwhite" => "GhostWhite",
			"honeydew" => "HoneyDew",
			"floralwhite" => "FloralWhite",
			"dimgray" => "DimGray",
			"seagreen" => "SeaGreen",
			"lavender" => "Lavender",
			"blanchedalmond" => "BlanchedAlmond",
			"darkolivegreen" => "DarkOliveGreen",
			"lightcyan" => "LightCyan",
			"darkseagreen" => "DarkSeaGreen",
			"mediumaquamarine" => "MediumAquaMarine",
			"plum" => "Plum",
			"gold" => "Gold",
			"springgreen" => "SpringGreen",
			"navy" => "Navy",
			"mistyrose" => "MistyRose",
			"orchid" => "Orchid",
			"yellowgreen" => "YellowGreen",
			"oldlace" => "OldLace",
			_ => panic!("[color-convert] map_name_to_name not match color_name value.")
		};
		to_color_name
	}

	pub fn map_hex(match_char: &str) -> usize {
		let match_number = match match_char {
			"A" => 10,
			"B" => 11,
			"C" => 12,
			"D" => 13,
			"E" => 14,
			"F" => 15,
			"1" => 1,
			"2" => 2,
			"3" => 3,
			"4" => 4,
			"5" => 5,
			"6" => 6,
			"7" => 7,
			"8" => 8,
			"9" => 9,
			"0" => 0,
			_ => panic!("[color-convert] map_hex not match match_char value.")
		};
		match_number
	}

	pub fn map_rgb<'a>(match_number: &'a usize) -> &'a str {
		let match_str: &'a str = match *match_number {
			0 => "0",
			1 => "1",
			2 => "2",
			3 => "3",
			4 => "4",
			5 => "5",
			6 => "6",
			7 => "7",
			8 => "8",
			9 => "9",
			10 => "A",
			11 => "B",
			12 => "C",
			13 => "D",
			14 => "E",
			15 => "F",
			_ => panic!("[color-convert] map_rgb not match match_number value.")
		};
		match_str
	}
}

pub mod handle {
	use config::cc_config::Setting;
	use handles::map_name;
	// hex -- #fff,#ffffff,#ffffff80,#80ffffff etc..
	// return -- ['f','f','f','f','f','f'],['f','f','f','f','f','f','8','0'] etc...
	pub fn handle_hex_value<'a>(hex: &'a str, setting: &Setting) -> Vec<&'a str> {
		let mut hex_vec: Vec<&'a str> = hex.split("").collect();
		hex_vec.retain(|&x| x != "" && x != "#");

		let mut return_vex: Vec<&'a str> = vec![];
		match hex_vec.len() {
			3 => {
				for item in &hex_vec {
					// let value: &'a str = &format!("{}{}", item, item);
					// let upper_item: &'a str = item.to_ascii_uppercase();
					return_vex.push(item);
					return_vex.push(item);
				}
			},
			6 => return_vex.extend(&hex_vec),
			8 => {
				if setting.is_android {
					return_vex.extend(&hex_vec[2..]);
					return_vex.extend(&hex_vec[0..2]);
				} else {
					return_vex.extend(&hex_vec);
				}
			},
			_ => panic!("[color-convert] hex value length must one of in [3, 6, 8]")
		}
		return_vex
	}

	// transparency value convert to hexadecimal
	// 0.5 -> "80"
	pub fn handel_alpha_to_hexadecimal(alpha: f32) -> String {
		if alpha == 1f32 || alpha > 1f32 {
			return String::from("FF");
		}

		let to_deciaml = alpha * 256f32;
		let b = (to_deciaml % 16f32) as usize;
		let a = (to_deciaml / 16f32 % 16f32) as usize;

		// let value: &'a str = &*format!("{}{}", map_name::map_rgb(a), map_name::map_rgb(b));
		map_name::map_rgb(&a).to_owned() + map_name::map_rgb(&b)
	}
}