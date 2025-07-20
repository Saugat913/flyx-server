use crate::app_state::AppState;
/// This is the file responsible for handling the initialization of the app
use crate::config::Config;
use crate::routes::routes;
use log::info;
use tokio::net::TcpListener;

pub async fn run_app() {
    let config = Config::from_env();
    env_logger::init();

    let app_state = AppState::new();
    let router = routes(app_state);
    let listener = TcpListener::bind(&config.server_address).await.unwrap();
    info!("Server is running at {}", &config.server_address);
    axum::serve(listener, router).await.unwrap();
}
