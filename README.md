### front と backend

- /front/ : フロントエンドのソースコード

  - /front/src/ : フロントエンドのソースコード
  - /front/dist/ : フロントエンドのビルド結果

- /backend/ : バックエンドのソースコード

  - /backend/src/ : バックエンドのソースコード

- 基本ルール

  - フロントエンドは html/CSS/Vanilla JavaScript を使用
  - バックエンドは Rust を使用
  - フロントエンドとバックエンドは REST API で通信

- コーディング規約
  - フロントエンド
    - [Google JavaScript Style Guide](https://google.github.io/styleguide/jsguide.html)
  - バックエンド
    - [Rust Style Guide]
    - [Rust API Guidelines]

### bbs の CRUD と http メソッド

---

- POST
  - / : メッセージの投稿
  - /:id : id に対応する投稿への返信
- GET
  - / : 投稿の一覧取得
  - /:id : id に対応する投稿の取得
- PATCH
  - /:id : id に対応する投稿の更新
- DELETE
  - /:id : id に対応する投稿の削除
