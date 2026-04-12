use axum::Json;
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            msg: "OK".into(),
            data: Some(data),
        }
    }
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl Resp<()> {
    pub fn err(e: Error) -> Self {
        Self {
            code: e.code(),
            msg: e.to_string(),
            data: None,
        }
    }
}

pub fn ok<T: Serialize>(data: T) -> Resp<T> {
    Resp::ok(data)
}

pub fn err(e: Error) -> Resp<()> {
    Resp::err(e)
}
