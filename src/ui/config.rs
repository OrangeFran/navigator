extern crate serde;
extern crate serde_derive;
extern crate toml;

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
    pub bg: Option<[u8; 3]>,
}

// a prefix (in front of the titles)
#[derive(Deserialize, Clone)]
pub struct Prefix {
    pub search: String,
    pub list: String,
    pub folder: String,
}

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub selected: Color,
    pub default: Color,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub prefixes: Prefix,
    pub theme: Theme,
    pub selector: String,
    pub lame: bool,
}

// takes the content of the config file / or an empty string
// + addition values passed in at runtime
pub fn read_config(string: &str, lame: bool) -> Config {
    // return the default if string is empty
    let mut config = if !string.is_empty() {
        toml::from_str::<Config>(string).expect("Failed to parse toml")
    } else {
        Config {
            prefixes: Prefix {
                search: "🔍 ".to_string(),
                list: "📂 ".to_string(),
                folder: "📁 ".to_string(),
            },
            theme: Theme {
                selected: Color { fg: None, bg: None },
                default: Color {
                    fg: Some([100, 100, 100]),
                    bg: None,
                },
            },
            selector: "> ".to_string(),
            lame: false,
        }
    };

    // fill in the additional values
    if lame {
        config.lame = lame;
    }

    config
}
