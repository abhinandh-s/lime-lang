use dashmap::DashMap;
use lime_analyzer::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        doc_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
