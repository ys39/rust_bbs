# Rust BBS

目的：簡単な bbs のバックエンドシステムを rust を利用して実装します。

## Tech Stack

**Server:**

- Rust
- MariaDB
- axum
- sqlx

**Client:**

- Svelte Kit
- JavaScript
- Windi CSS

## Rule

- Backend と Frontend は REST API で通信
- SSR/SPA
  - 初回のみサーバでレンダリングを行い、以降は JavaScript の通信でデータ取得を行い、ページ更新を実施する


## API Reference

#### Post message

```http
  POST /
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `content` | `string` | **Required**. message |


#### Get all posts

```http
  GET /getposts
```

#### Delete post

```http
  POST /delete
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. Id of Post |

