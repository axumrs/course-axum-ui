use std::sync::Arc;

use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

pub type ArcAppState = Arc<AppState>;

pub fn new_arc_state(pool: PgPool) -> ArcAppState {
    Arc::new(AppState { pool })
}
