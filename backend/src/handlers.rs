/*
handler
リクエスト・レスポンスを扱う
 */
use axum::{
    extract::{
        Extension,
        Path,
        FromRequest,
        State,
        rejection::JsonRejection,
    },
    http::{
        StatusCode,
        Request,
    },
    body::HttpBody,
    response::IntoResponse,
    Json, async_trait, BoxError,
};
use serde::de::DeserializeOwned;
//use tokio::sync::mpsc::error::SendTimeoutError;

use std::sync::Arc;
use validator::Validate;

use crate::repositories::{ PostRepository, PostContent };

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send + 'static + std::fmt::Debug,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    //Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    //B: Send + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        println!("{:?}", req);
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|rejection| {
            println!("debug1");
            let message = format!("Json parse error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;
        value.validate().map_err(|rejection|{
            println!("debug2");
            let message = format!("Validate error: [{}]", rejection).replace('\n',", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

// State, ValidatedJsonの引数の順番
pub async fn insert_post<T: PostRepository>(
    //Json(payload): Json<PostContent>,
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<PostContent>,
) -> Result<impl IntoResponse, StatusCode> {
    //let post = repository.insert(payload).await.or(Err(StatusCode::NOT_FOUND))?;
    repository.insert(payload).await.or(Err(StatusCode::NOT_FOUND))?;
    //Ok((StatusCode::CREATED, Json(post)))
    Ok(StatusCode::CREATED)
}

pub async fn find_post<T: PostRepository>(
    Path(id): Path<u32>,
    //Extension(repository): Extension<Arc<T>>,
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let post = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(post)))
}

