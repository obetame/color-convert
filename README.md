color-convert
====

Convert color to each other by rust.

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/zhouyuexie/color-convert/blob/master/LICENSE-MIT)

## Installation

Add this to the Cargo.toml file of your project:

```toml
[dependencies]
color-convert = "0.1.0"
```

Don't forget to add the external crate:

```rust
extern crate color_convert;
```

## Getting Started

```rust
extern crate color_convert;

use color_convert::color::{Color, Error};

fn main() -> Result<(), Error> {
    let mut color = Color::init("#c8c8c8ff", false, false, false);
    // or just 
    let mut color = Color::new("#c8c8c8ff");

    assert_eq!("rgb(200,200,200)", color.to_rgb()?);
}
```

## Settings

Color has three simple configurations: 

1. whether it is uppercase: `upper`
2. whether it contains transparency: `alpha`
3. whether it is Android format (only hex will be used): `andriod`

```rust
// ...
fn main() -> Result<(), Error> {
    let mut color = Color::new("#c8c8c8ff");

    // you can change some config
    assert_eq!("RGB(200,200,200)", color.to_upper(true).to_rgb()?);
    assert_eq!("RGBA(200,200,200,1)", color.to_alpha(true).to_rgb()?);
    assert_eq!("RGBA(200,200,255,0.78)", color.to_android(true).to_rgb()?);
}
```

## Features

Support `RGB`,`RGBA`,`HEX`,`HSL`,`HSLA`,`HSV`,`CMYK` to transform each other.

Their corresponding method is `to_rgb`, `to_hex`, `to_hsl`, `to_hsv`, `to_cmyk`. For formats that need to be converted to contain transparency values, you need to first set the configuration alpha to true(use method `to_alpha`).

## Notes

1. Some values are lost when some precision values are converted to CMYK and HSV, this is the result of the algorithm, so be careful in the process of using it.
2. Do not support css3 color:`transparent` and `currentColor`.

## Next

- Will support [css3 color name](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value) convert to `HEX` and convert back.

## Help

If you have any questions or suggestions, please contact me via issues or email.

For those who use sublime text, try a similar plugin I wrote for [ColorConvert](https://github.com/zhouyuexie/ColorConvert).