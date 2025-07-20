use axum::{Router, extract::State, routing::post};

use crate::app_state::AppState;

pub fn routes() -> Router<AppState> {
    let router = Router::new().route("/room", post(create_room));
    router
}

async fn create_room(State(state): State<AppState>) {
    state.create_new_room();
}
