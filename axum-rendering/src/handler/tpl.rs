use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_ui::ArcAppState;
use db::model;

use crate::Result;

#[derive(Template)]
#[template(path = "find_user.html")]
pub struct FindUserTemplate {
    pub user: model::User,
}

pub async fn find_user(
    State(state): State<ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>> {
    let user = db::model::User::find(&state.pool, id)
        .await
        .map_err(|e| e.to_string())?;
    let user = match user {
        Some(v) => v,
        None => return Err("user not found".to_string()),
    };

    let t = FindUserTemplate { user };
    let html = t.render().map_err(|e| e.to_string())?;

    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "list_user.html")]
pub struct ListUserTemplate {
    pub user_list: Vec<model::User>,
}

pub async fn list_user(State(state): State<axum_ui::ArcAppState>) -> Result<Html<String>> {
    let user_list = db::model::User::list(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let t = ListUserTemplate { user_list };
    let html = t.render().map_err(|e| e.to_string())?;
    Ok(Html(html))
}
