#[allow(unused_imports)]
use http_server::run;

use std::process::exit;

fn main() {
    match run() {
        Ok(_) => println!("server-exited"),
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}
