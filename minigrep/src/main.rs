
use minigrep::Config;
use minigrep::run;
use std::process;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(msg) = run(&config) {
        println!("failed to run config. {}", msg);
    }
}
