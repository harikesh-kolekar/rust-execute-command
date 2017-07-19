use std::fs::File;
use std::io::prelude::*;

fn readConfig() -> String {
	// let mut file = File::open("./run.toml")?;
	// let mut contents = String::new();

	// file.read_to_string(&mut contents)?;

	let mut file = File::open("./run.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

	contents.to_string()
	// "hello world".to_string()
}

fn main() {
	let config = readConfig();

	println!("{}", config);
}
