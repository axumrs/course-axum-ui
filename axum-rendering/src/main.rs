use axum_rendering::handler;
use axum_ui::new_arc_state;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

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

    let app = handler::router::init(new_arc_state(pool));

    axum::serve(listener, app).await?;
    Ok(())
}
