use std::fs::File;
use std::io::prelude::*;

fn readConfig() -> Result<String, String> {
	// let mut file = File::open("./run.toml")?;
	// let mut contents = String::new();

	// file.read_to_string(&mut contents)?;

	let mut file = File::open("./run.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

	Ok(contents.to_string())
	// "hello world".to_string()
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
