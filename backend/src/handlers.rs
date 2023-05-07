/*
handler
リクエスト・レスポンスを扱う
 */
use axum::{
    extract::{
        Path,
        FromRequest,
        State,
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
use std::sync::Arc;
use validator::Validate;
use crate::repositories::{ PostRepository, PostContent, DeletePostId, PagePostContent };

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
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|rejection| {
            let message = format!("Json parse error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;
        value.validate().map_err(|rejection|{
            let message = format!("Validate error: [{}]", rejection).replace('\n',", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

// State, ValidatedJsonの引数の順番
pub async fn insert_post<T: PostRepository>(
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<PostContent>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.insert(payload).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok(StatusCode::CREATED)
}

pub async fn select_all_post<T: PostRepository>(
    State(repository): State<Arc<T>>,
    Json(payload): Json<PagePostContent>,
) -> Result<impl IntoResponse, StatusCode> {
    let posts = repository.select_all(payload).await.or(Err(StatusCode::NOT_FOUND))?;
    println!("{:?}", posts);
    Ok((StatusCode::OK, Json(posts)))
}

// State, ValidatedJsonの引数の順番
pub async fn delete_post<T: PostRepository>(
    State(repository): State<Arc<T>>,
    Json(payload): Json<DeletePostId>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(payload).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok(StatusCode::OK)
}