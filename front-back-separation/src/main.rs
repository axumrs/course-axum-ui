use axum::{Router, routing::get};
use front_back_separation::handler;
use tokio::net::TcpListener;

use axum_ui::{ArcAppState, new_arc_state};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9527").await?;

    let dsn = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://axum_ui:axum_ui@127.0.0.1:5432/axum_ui".into());
    let max_conn: u32 = std::env::var("MAX_CONNECTIONS")
        .unwrap_or("5".into())
        .parse()
        .unwrap_or(5);

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&dsn)
        .await?;
    let state = new_arc_state(pool);

    let app = Router::new().nest("/api", router_init(state));

    axum::serve(listener, app).await?;
    Ok(())
}

fn router_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/user", get(handler::list_user))
        .route("/user/{id}", get(handler::find_user))
        .with_state(state)
}
