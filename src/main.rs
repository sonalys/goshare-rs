mod domain;
mod application;
mod repositories;
mod adapters;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Arc::new(adapters::InMemoryGroupRepo::new());
    let svc = Arc::new(application::LedgerService::new(repo));

    println!("Starting Ledger API on http://127.0.0.1:8080");

    adapters::http::run_server(svc).await
}
