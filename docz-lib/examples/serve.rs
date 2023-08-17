//! Serves assets

use docz_lib::{serve::ServeOptions, Service};

#[tokio::main]
async fn main() {
    env_logger::init();

    let service = Service::builder()
        .root_dir("../docz-demo")
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
