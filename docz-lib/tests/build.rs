//! Test build

use std::sync::Once;

use docz_lib::{build::BuildOptions, Service};

static INIT_ONCE: Once = Once::new();

/// Initializes the service
fn init_service() -> Service {
    INIT_ONCE.call_once(|| {
        env_logger::init();
    });

    Service::builder()
        // NB: current_dir() points to the root of the crate (tests only)
        .root_dir("./tests/build")
        .dbg_renderer()
        .html_renderer()
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_build() {
    let service = init_service();
    service
        .build(BuildOptions {
            watch: false,
            extra_watch_dirs: vec![],
            on_rebuilt: None,
        })
        .await
        .unwrap();
}
