extern crate toml;
extern crate serde;
extern crate serde_derive;

use serde_derive::Deserialize;

// Could be used (in the future)
// to make configuration configuration easier
// -> but pretty hard to setup
//
// // toml get's parse into this format first
// #[derive(Deserialize)]
// struct OptColor {
//     fg: Option<[usize; 3]>,
//     bg: Option<[usize; 3]>
// }
// 
// #[derive(Deserialize)]
// struct OptTheme {
//     selected: Option<Color>,
//     default: Option<Color>
// }
// 
// #[derive(Deserialize)]
// struct OptConfig {
//     theme: Option<Theme>,
//     selector: Option<String>
// }


// create a non optional struct
// with default/user configuration options
#[derive(Deserialize, Clone)]
pub struct Color {
    pub fg: Option<[u8; 3]>,
    pub bg: Option<[u8; 3]>
}

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub selected: Color,
    pub default: Color
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub theme: Theme,
    pub selector: String
}

pub fn read_config(string: &str) -> Config {
    // return the default if string is empty
    if !string.is_empty() {
        return toml::from_str::<Config>(string)
            .expect("Failed to parse toml")
    }
    // else use the default values
    Config {
        theme: Theme {
            selected: Color { 
                fg: None,
                bg: None
            },
            default: Color {
                fg: Some([100, 100, 100]),
                bg: None
            }
        },
        selector: "> ".to_string()
    }
}
