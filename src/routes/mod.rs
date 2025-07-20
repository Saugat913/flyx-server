use axum::Router;
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;
pub mod room;
pub mod ws;

pub fn routes(state: AppState) -> Router {
    let router = Router::new()
        .merge(room::routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    return router;
}
