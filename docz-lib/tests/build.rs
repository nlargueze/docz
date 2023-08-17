//! Test build

use std::sync::Once;

use docz_lib::Service;

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
        .unwrap()
        .build()
        .unwrap()
}

#[test]
fn test_build() {
    let service = init_service();
    service.build().unwrap();
}
