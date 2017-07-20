#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate term_painter;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use term_painter::Attr::*;
use term_painter::Color::*;
use term_painter::ToStyle;
use toml::Value;

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
    toml::from_str(&config).map_err(|e: toml::de::Error| e.to_string())
}

fn print_menu(config: Config) -> Result<String, String> {
    println!("{}", "==============================================");
    for item in config.item {
        println!("{:6} {}", Bold.paint(&item.code), item.desc);
    }
    println!("{}", "==============================================");
    println!("{}", Green.paint("Select an option"));

    io::stdout().flush();
    let mut opt = String::new();
    io::stdin().read_line(&mut opt);

    Ok("".to_string())
}

fn main() {
    let config_result = read_config().and_then(parse_config);

    let result = config_result.and_then(|config| print_menu(config));

    match result {
        Ok(config) => println!("{}", "Bye!"),

        Err(err) => println!("{}", err),
    }

}
