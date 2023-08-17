//! Test build

use std::sync::Once;

use docz_lib::{
    rend::{DebugRenderer, HTMLRenderer},
    Service,
};

static INIT: Once = Once::new();

/// INitializes the tests
fn init_tests() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[test]
fn test_build() {
    init_tests();

    let dbg_renderer = DebugRenderer::new();
    let html_renderer = HTMLRenderer::new().unwrap();

    let service = Service::builder()
        .root_dir("../docz-demo")
        .renderer(dbg_renderer)
        .renderer(html_renderer)
        .build()
        .unwrap();
    // service.init_root_dir().unwrap();

    service.build().unwrap();
}
