//! Serves assets

use docz_lib::{serve::ServeOptions, watch::WatchOptions, Service};

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let root_dir_str = if let Some(arg_1) = args.get(1) {
        arg_1
    } else {
        panic!("Example requires an extra argument to specify the root directory")
    };

    let service = Service::builder()
        .root_dir(root_dir_str)
        .dbg_renderer()
        .html_renderer()
        .build()
        .unwrap();

    service
        .serve(
            ServeOptions {
                port: 5000,
                open: true,
            },
            Some(WatchOptions::default()),
        )
        .await
        .unwrap();
}
