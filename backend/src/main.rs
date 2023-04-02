mod repositories;
mod handlers;

use crate::repositories::{
    PostRepository, PostRepositoryForDb
};

use handlers::{
    find_post,
    insert_post,
};

use axum::{
    extract::{
        Form,
        //Extension,
        //State,
    },
    //body::HttpBody,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
    //BoxError,
};
use serde::{Deserialize};
use std::{env, sync::Arc};
use std::net::SocketAddr;
use thiserror::Error;
use anyhow::{Result};
use askama::Template;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // database情報読み込み
    dotenv().ok();    // .envの読み込み
    let database_url = &env::var("DATABASE_URL").expect("undefined DATABASE_URL");

    // プールの作成
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(20)
        .connect(&database_url).await.unwrap();

    // 初期化
    let repository = PostRepositoryForDb::new(pool.clone());

    let app = create_app(repository);
    let addr = SocketAddr::from(([172, 100, 1, 100], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// トレイト境界
// PostRepositoryトレイトを実装している何かを引数とする
fn create_app<T>(repository: T) -> Router
where
    T: PostRepository,
{
    Router::new()
        //.route("/", get(show_form).post(accept_form))
        // https://docs.rs/axum/latest/axum/handler/trait.Handler.html
        // ここを読む
        // ::<> を Turbofish (ターボフィッシュ)
        .route("/", get(show_form))
        // 型推論を使わず、型Tを指定してinsert_postを引数にしている
        .route("/", post(insert_post::<T>))
        .route("/p/:id", get(find_post::<T>))
        // axumアプリケーション内でrepositoryを共有することができ、
        // repositoryは各ハンドラの引数で受け取ることが可能
        //.layer(Extension(Arc::new(repository)))
        .with_state(Arc::new(repository))
}

/*
formを表示する
 */
async fn show_form() -> impl IntoResponse {
    let template = HelloTemplate { post_msg: "".to_string() };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    post_msg: String,
}

/*
HtmlTemplateという構造体に対してIntoResponseトレイトを実装している
 */
struct HtmlTemplate<T>(T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct PostInput {
    text: String,
}

#[derive(Error, Debug)]
enum PostError {
    #[error("message empty")]
    MsgIsEmpty,
    #[error("message must be within 100 charcters")]
    MsgIsOverSize
}

// validate
async fn check_msg(msg: &str) -> Result<&'static str> {
    // 空の場合はエラー
    let msg_length = msg.len();
    if msg_length == 0 {
        Err(PostError::MsgIsEmpty.into())
    } else if msg_length >= 100 {
        Err(PostError::MsgIsOverSize.into())
    } else {
        Ok("ok")
    }
}

/*
form受け取り
 */
async fn accept_form(Form(post_input): Form<PostInput>) -> impl IntoResponse {

    // 入力チェック
    let res = check_msg(&post_input.text).await;
    let template;
    if res.is_ok() {
        println!("{}", post_input.text);
        template = HelloTemplate { post_msg: post_input.text.to_string() };
        
    }else{
        println!("{:?}", res);
        template = HelloTemplate { post_msg: "err".to_string() };
    }
    HtmlTemplate(template)
}
