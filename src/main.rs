#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use std::error::Error;
use std::fmt;

#[derive(Deserialize)]
struct Config {
    item: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    cmd: String,
    code: String,
    desc: String,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

fn read_config() -> Result<String, String> {
    File::open("./run.toml")
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
}

fn parse_config(config: String) -> Result<Config, String> {
    // config.parse::<Value>().map_err(|e: toml::de::Error| e.to_string())
    toml::from_str(&config).map_err(|e: toml::de::Error| e.to_string())
}

fn main() {
    let result = read_config().and_then(parse_config);

    match result {
        Ok(config) => println!("{:?}", config.item),

        Err(err) => println!("{}", err),
    }

}
