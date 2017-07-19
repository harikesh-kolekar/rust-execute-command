extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use std::error::Error;

fn readConfig() -> Result<String, String> {
	File::open("./run.toml")
		.map_err(|err| err.to_string())
		.and_then(|mut file| {
			let mut contents = String::new();
			file.read_to_string(&mut contents)
				.map_err(|err| err.to_string())
				.map(|_| contents)
		})
}

fn parseConfig(config: String) -> Result<Value, String> {
	config.parse::<Value>()
		.map_err(|e: toml::de::Error| e.to_string())
}

fn main() {
	let config = readConfig()
		.and_then(parseConfig);

	match config {
		Ok(text) =>
			println!("{}", text),

		Err(err) =>
			println!("{}", err),
	}

}
