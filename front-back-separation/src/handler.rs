use axum::{
    Json,
    extract::{Path, State},
};
use axum_ui::ArcAppState;
use db::model;

use crate::{Error, Resp, resp};

pub async fn find_user(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Json<Resp<model::User>>, Error> {
    let user = model::User::find(&state.pool, id).await?;
    let user = match user {
        Some(v) => v,
        None => return Err(anyhow::anyhow!("user not exists").into()),
    };

    Ok(resp::ok(user).to_json())
}

pub async fn list_user(
    State(state): State<ArcAppState>,
) -> Result<Json<Resp<Vec<model::User>>>, Error> {
    let user_list = model::User::list(&state.pool).await?;
    Ok(resp::ok(user_list).to_json())
}
