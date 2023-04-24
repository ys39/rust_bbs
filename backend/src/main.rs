mod repositories;
mod handlers;

use crate::repositories::{
    PostRepository, PostRepositoryForDb
};

use handlers::{
    find_post,
    insert_post,
    select_all_post,
    delete_post,
};

use axum::{
    routing::{get, post},
    Router,
};
use std::{env, sync::Arc};
use std::net::SocketAddr;
use dotenv::dotenv;
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer, AllowOrigin};

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // database情報読み込み
    dotenv().ok(); // .envの読み込み
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
        //.route("/", get(show_form))
        // 型推論を使わず、型Tを指定してinsert_postを引数にしている
        .route("/getposts", get(select_all_post::<T>))
        .route("/delete", post(delete_post::<T>))
        .route("/", post(insert_post::<T>))
        .route("/p/:id", get(find_post::<T>))
        // axumアプリケーション内でrepositoryを共有することができ、
        // repositoryは各ハンドラの引数で受け取ることが可能
        //.layer(Extension(Arc::new(repository)))
        .with_state(Arc::new(repository))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact("http://f1.nono.com".parse().unwrap()))
                .allow_origin(AllowOrigin::exact("http://nono:9000".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE])
        )
}
