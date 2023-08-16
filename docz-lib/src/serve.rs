//! Server

use salvo::prelude::*;

use crate::Service;

/// Serve options
#[derive(Debug)]
pub struct ServeOptions {
    /// Port
    pub port: u16,
    /// Opens the browser when launching the browser
    pub open: bool,
    /// Watches the source when
    pub watch: bool,
}

impl Default for ServeOptions {
    fn default() -> Self {
        Self {
            port: 3000,
            open: false,
            watch: false,
        }
    }
}

impl ServeOptions {
    /// Creates a new set of values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Opens the page when serving  
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Watches the source files and reloads
    pub fn watch(mut self, watch: bool) -> Self {
        self.watch = watch;
        self
    }
}

impl Service {
    /// Servers the build
    pub async fn serve(&self, opts: ServeOptions) {
        let build_dir = self.config.build_dir();
        let router = Router::with_path("<**path>").get(
            StaticDir::new([&build_dir])
                .defaults("index.html")
                .listing(true),
        );

        let addr = format!("127.0.0.1:{}", opts.port);
        let full_addr = format!("http://{}", addr);
        let acceptor = TcpListener::new(addr.as_str()).bind().await;

        if opts.open {
            if let Err(err) = open::that(&full_addr) {
                eprintln!("An error occurred when opening '{}': {}", full_addr, err)
            }
        }

        eprintln!();
        eprintln!("Serving on http://{}", addr);

        Server::new(acceptor).serve(router).await;
    }
}
