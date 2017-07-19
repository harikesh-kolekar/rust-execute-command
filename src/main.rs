use std::fs::File;
use std::io::prelude::*;

fn readConfig() -> Result<String, std::io::Error> {
	let mut contents = String::new();

	let mut file =
		File::open("./run.toml")?;

	let res = file
		.read_to_string(&mut contents);

	match res {
		Ok(_) =>
			Ok(contents.to_string()),

		Err(err) =>
			Err(err),
	}
}

fn main() {
	let config = readConfig();

	match config {
		Ok(text) =>
			println!("{}", text),

		Err(err) =>
			println!("{}", err),
	}

}
