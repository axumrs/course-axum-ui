use axum::{Router, routing::get};
use axum_ui::ArcAppState;

use crate::handler;

pub fn init(state: ArcAppState) -> Router {
    Router::new().nest("/pure", pure_init(state.clone()))
}

fn pure_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/{id}", get(handler::pure::find_user))
        .route("/", get(handler::pure::list_user))
        .with_state(state)
}
