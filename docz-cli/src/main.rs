use colored::Colorize;

mod args;
mod cmd;

fn main() {
    match cmd::run() {
        Ok(_) => {}
        Err(err) => {
            let msg = format!("❌ {}", err).bright_red();
            eprintln!("{msg}")
        }
    }
}
