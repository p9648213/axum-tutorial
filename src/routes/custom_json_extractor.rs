use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "Must be a valid email"))]
    username: String,
    #[validate(length(min = 8, message = "Must have at lest 8 characters"))]
    password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(err) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", err)));
        }

        Ok(user)
    }
}
pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
