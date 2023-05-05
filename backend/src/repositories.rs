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

// bbsへの投稿内容件数分取得用
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct PagePostContent {
    offset: String,
    row_count: String,
}

// bbsへの投稿内容件数分取得用
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct PostDetail {
    id: u32,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

// bbsへのpost全件数取得用
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct PostAllCount {
    count: i64, // sql count()はi64で返ってくる
}

// bbsへの投稿内容を削除するためのid
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct DeletePostId {
    id: String, // JsonでPostされるため、u32でなくStringにする
}

// bbsへの投稿内容をinsertするための構造体
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
    async fn select_all(&self, payload:PagePostContent) -> anyhow::Result<(Vec<PostDetail>, PostAllCount)>;
    async fn find(&self, id:u32) -> anyhow::Result<Post>;
    async fn insert(&self, payload:PostContent) -> anyhow::Result<()>;
    async fn delete(&self, payload: DeletePostId) -> anyhow::Result<()>;
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
        Ok(post)
    }
    // select_allメソッド
    async fn select_all(&self, payload:PagePostContent) -> anyhow::Result<(Vec<PostDetail>, PostAllCount)>{
        let posts = sqlx::query_as::<_, PostDetail>(
        r#"
SELECT * FROM post WHERE is_delete = 0 order by id desc, created_at desc
LIMIT ?, ?;
        "#,
        )
        .bind(payload.offset.clone())
        .bind(payload.row_count.clone())
        .fetch_all(&self.pool)
        .await?;
    let posts_num = sqlx::query_as::<_, PostAllCount>(
        r#"
SELECT COUNT(*) as count FROM post WHERE is_delete = 0;
        "#,
        )
        .fetch_one(&self.pool)
        .await?;
        println!("{:?}", posts_num);
        Ok((posts.clone(), posts_num))
    }
    // insertメソッド
    // 投稿内容をinsertした後にinsertされたデータを取得してPostへ格納
    async fn insert(&self, payload: PostContent) -> anyhow::Result<()>{
        let datetime = chrono::Local::now().naive_local();
        sqlx::query::<_>(
        r#"
insert into post (content, is_delete, created_at, updated_at)
values (?, 0, ?, ?);
        "#,
        )
        .bind(payload.content.clone())
        .bind(datetime).bind(datetime)
        .execute(&self.pool)
        .await?;
        Ok(())

    }
    // deleteメソッド
    async fn delete(&self, payload: DeletePostId) -> anyhow::Result<()>{
        sqlx::query::<_>(
        r#"
update post set is_delete = 1 WHERE id = ?
        "#,
        )
        .bind(payload.id.clone())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}