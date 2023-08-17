//! Serves assets

use docz_lib::{serve::ServeOptions, Service};

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if let Some(doc_path) = args.get(1) {
        std::env::set_current_dir(&args[1]).unwrap();
    } else {
        std::env::set_current_dir("../doc").unwrap();
    }
    eprintln!("args: {:?}", args);

    eprintln!(
        "current_dir: {}",
        std::env::current_dir().unwrap().display()
    );

    let service = Service::builder()
        .root_dir("../doc")
        .dbg_renderer()
        .html_renderer()
        .unwrap()
        .build()
        .unwrap();

    service
        .serve(ServeOptions {
            port: 5000,
            open: true,
            watch: true,
        })
        .await
        .unwrap();
}
