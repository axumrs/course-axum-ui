use axum::response::IntoResponse;

use crate::resp;

pub struct Error(anyhow::Error);

impl Error {
    pub fn code(&self) -> i32 {
        -1
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Error(e.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        resp::err(self).to_json().into_response()
    }
}
