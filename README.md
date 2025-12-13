# Management Website
ウェブサイト上でパソコンを管理できるようにします

## ディレクトリ構造
- **/agnet**
管理されるサーバー用APIのソースコード
- **/central**
集約&配信サーバーのソースコード
- **/common**
agent、central用のデータ型
- **/migrations**
SQLのmigrate
- **/public**
React用の配信用ファイルの置き場所
- **/static**
フロントエンドのビルド済みファイル
- **/ts-src**
フロントエンドのソースコード

## 実行方法
- **Agentの実行**
```bush
cargo run -p agent
```

- **Centralの実行**
```bush
sqlx database create
sqlx migrate run
cargo run -p central
```