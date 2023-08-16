//! Serves assets

use docz_lib::{
    rend::{DebugRenderer, HTMLRenderer},
    serve::ServeOptions,
    Service,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let dbg_renderer = DebugRenderer::new();
    let html_renderer = HTMLRenderer::new().unwrap();

    let service = Service::builder()
        .root_dir("./tests_root")
        .renderer(dbg_renderer)
        .renderer(html_renderer)
        .build()
        .unwrap();

    service
        .serve(ServeOptions {
            port: 5000,
            open: true,
            watch: true,
        })
        .await;
}
