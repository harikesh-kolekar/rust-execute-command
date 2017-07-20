#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use std::error::Error;

#[derive(Deserialize)]
struct Config {
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    cmd: String,
    code: String,
    description: String,
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

fn parse_config(config: String) -> Result<Value, String> {
    config.parse::<Value>().map_err(|e: toml::de::Error| e.to_string())
}

fn main() {
    let config = read_config().and_then(parse_config);

    match config {
        Ok(text) => println!("{}", text),

        Err(err) => println!("{}", err),
    }

}
