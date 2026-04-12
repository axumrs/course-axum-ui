use axum::{Router, routing::get};
use axum_ui::ArcAppState;

use crate::handler;

pub fn init(state: ArcAppState) -> Router {
    Router::new()
        .nest("/pure", pure_init(state.clone()))
        .nest("/tpl", tpl_init(state))
}

fn pure_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/{id}", get(handler::pure::find_user))
        .route("/", get(handler::pure::list_user))
        .with_state(state)
}

fn tpl_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/{id}", get(handler::tpl::find_user))
        .route("/", get(handler::tpl::list_user))
        .with_state(state)
}
