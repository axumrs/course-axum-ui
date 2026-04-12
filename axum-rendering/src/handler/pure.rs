use axum::{
    extract::{Path, State},
    response::Html,
};

use crate::Result;

pub async fn find_user(
    State(state): State<axum_ui::ArcAppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>> {
    let user = db::model::User::find(&state.pool, id)
        .await
        .map_err(|e| e.to_string())?;
    let user = match user {
        Some(v) => v,
        None => return Err("user not found".to_string()),
    };

    Ok(Html(format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>AXUM UI</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-sRIl4kxILFvY47J16cr9ZwB07vP4J8+LH7qKQnuqkuIAvNWLzeN8tE5YBujZqJLB" crossorigin="anonymous">
  </head>
  <body>
    <h1>用户 #{0}</h1>
    <div class="card">
  <div class="card-body">
    <h5 class="card-title">{1}</h5>
    <ul class="card-text">
        <li>用户名：{1}</li>
        <li>邮箱：{2}</li>
        <li>创建时间：{3}</li>
    </ul>
  </div>
</div>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.bundle.min.js" integrity="sha384-FKyoEForCGlyvwx9Hj09JcYn3nv7wiPVlz7YYwJrWVcXK/BmnVDxM+D2scQbITxI" crossorigin="anonymous"></script>
  </body>
</html>"#,
        user.id, user.username, user.email, user.created_at
    )))
}

pub async fn list_user(State(state): State<axum_ui::ArcAppState>) -> Result<Html<String>> {
    let user_list = db::model::User::list(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    let user_list_html = user_list
        .iter()
        .map(|u| {
            format!(
                r#"<tr>
      <th scope="row">{0}</th>
      <td><a href="/pure/{0}">{1}</a></td>
      <td>{2}</td>
      <td>{3}</td>
    </tr>"#,
                u.id, u.username, u.email, u.created_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        .to_string();

    Ok(Html(format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>AXUM UI</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-sRIl4kxILFvY47J16cr9ZwB07vP4J8+LH7qKQnuqkuIAvNWLzeN8tE5YBujZqJLB" crossorigin="anonymous">
  </head>
  <body>
    <h1>用户列表</h1>
    <table class="table">
  <thead>
    <tr>
      <th scope="col">#</th>
      <th scope="col">用户名</th>
      <th scope="col">邮箱</th>
      <th scope="col">时间</th>
    </tr>
  </thead>
  <tbody>
    {user_list_html}
  </tbody>
</table>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.bundle.min.js" integrity="sha384-FKyoEForCGlyvwx9Hj09JcYn3nv7wiPVlz7YYwJrWVcXK/BmnVDxM+D2scQbITxI" crossorigin="anonymous"></script>
  </body>
</html>"#
    )))
}
