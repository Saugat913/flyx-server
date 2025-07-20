use crate::app::run_app;

mod app;
mod app_state;
mod config;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    run_app().await;
}
