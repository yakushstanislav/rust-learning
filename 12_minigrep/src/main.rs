use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Can't parse config: {err}");

        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Can't run: {err}");

        process::exit(1);
    }
}
