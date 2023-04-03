/*
repository
DBとのやり取りを定義する
 */

use axum::{
    async_trait,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;
use validator::Validate;

// エラー設定
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(u32),
    #[error("Unexpectd Error: [{0}]")]
    Unexpected(String),
}

// bbsへの投稿内容
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct Post {
    id: u32,
    content: String
}

// bbsへの投稿内容(insert)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct PostContent {
    #[validate(length(min=1, message="投稿内容を入力してください"))]
    #[validate(length(max=10, message="10文字以内で投稿内容を入力してください"))]
    content: String,
}

// PostRepository
// postに関するデータを取り出すコンポーネント
#[async_trait]
pub trait PostRepository : Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn find(&self, id:u32) -> anyhow::Result<Post>;
    //async fn insert(&self, payload:PostContent) -> anyhow::Result<Post>;
    async fn insert(&self, payload:PostContent) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct PostRepositoryForDb {
    pool: MySqlPool,
}

// poolをコンストラクタとして読み込む
impl PostRepositoryForDb {
    pub fn new(pool: MySqlPool) -> Self {
        PostRepositoryForDb{
            pool
        }
    }
}

// PostRepositoryトレイトをPostRepositoryForDbに実装する
#[async_trait]
impl PostRepository for PostRepositoryForDb {
    // selectメソッド
    async fn find(&self, id: u32) -> anyhow::Result<Post>{
        let post = sqlx::query_as::<_, Post>(
        r#"
SELECT * FROM post WHERE id=?
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e{
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;
        println!("{:?}", post);
        Ok(post)
    }
    // insertメソッド
    // 投稿内容をinsertした後にinsertされたデータを取得してPostへ格納
    //async fn insert(&self, payload: PostContent) -> anyhow::Result<Post>{
    async fn insert(&self, payload: PostContent) -> anyhow::Result<()>{
        println!("{:?}", payload);
        /*
        let post = sqlx::query_as::<_, Post>(
        r#"
insert into post (content, is_delete, created_at, updated_at)
values($1, 0, NOW(), NOW());
select * from post where id = (select last_insert_id());
        "#,
        )
        .bind(payload.content.clone())
        .fetch_one(&self.pool)
        .await?;
        Ok(post)
        */

        sqlx::query::<_>(
        r#"
insert into post (content, is_delete)
values (?, 0)
        "#,
        )
        .bind(payload.content.clone())
        .execute(&self.pool)
        .await?;
        Ok(())

    }
}