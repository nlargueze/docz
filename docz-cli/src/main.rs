use colored::Colorize;

mod cmd;

#[tokio::main]
async fn main() {
    eprintln!();
    if let Err(err) = cmd::run().await {
        let msg = format!("❌ {}", err).bright_red();
        eprintln!("{msg}");
    }
}
