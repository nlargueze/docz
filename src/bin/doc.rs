//! CLI

use colored::Colorize;
use docz;

fn  main() {
    match docz::run() {
        Ok(_) => {},
        Err(err) => {
            let msg = format!("❌ {}", err.to_string()).bright_red();
            eprintln!("{msg}" )
        },
    }
}