use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub async fn list(e: impl PgExecutor<'_>) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as(r#"SELECT "id","username","email","password","created_at"  FROM users ORDER by "id" LIMIT 10"#)
            .fetch_all(e)
            .await
    }

    pub async fn find(e: impl PgExecutor<'_>, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as(
            r#"SELECT "id","username","email","password","created_at"  FROM users WHERE "id" = $1"#,
        )
        .bind(id)
        .fetch_optional(e)
        .await
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{PgPool, Result};

    use crate::model::User;

    async fn get_pool() -> Result<PgPool> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://axum_ui:axum_ui@127.0.0.1:5432/axum_ui".into());
        PgPool::connect(&database_url).await
    }

    #[tokio::test]
    async fn test_user_list() {
        let pool = get_pool().await.unwrap();
        let user_list = User::list(&pool).await.unwrap();
        assert_eq!(user_list.len(), 10);
    }

    #[tokio::test]
    async fn test_user_find() {
        let pool = get_pool().await.unwrap();
        let user = User::find(&pool, 300).await.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().id, 300);
    }
}
