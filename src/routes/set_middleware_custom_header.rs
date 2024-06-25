use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(mut request: Request, next: Next) -> Result<Response, StatusCode>{
  let header = request.headers();

  let message = header.get("message").ok_or_else(|| {
    StatusCode::BAD_REQUEST
  })?;

  let message = message.to_str().map_err(|_| {
    StatusCode::BAD_REQUEST
  })?.to_owned();

  request.extensions_mut().insert(HeaderMessage(message));

  Ok(next.run(request).await)
}