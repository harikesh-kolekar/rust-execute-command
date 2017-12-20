#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate term_painter;
use std::fmt;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
#[derive(Clone)]
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


fn run_item() -> Result<std::process::Output, String> {
    let parts: Vec<&str> = "git clone ssh://hkolekar@chef@automate.chef.co:8989/chef/products/automate".split(" ").collect();

    parts
        .split_first()
        .ok_or("Command not found".to_string())
        .and_then(|(cmd, rest)| {
            Command::new(cmd)
                .args(rest)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|err| err.to_string())
        })

}

fn main() {
    let result = run_item();


    match result {
        Ok(config) => println!("{}", ""),

        Err(err) => println!("{}", err),
    }

}
