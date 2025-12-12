# 📜 API要件定義書: エントリーサーバー (Gateway & Aggregator) (完全版)

## 1\. 概要と責務

### 1.1. システム構造

本システムは、**エントリーサーバー**と**管理対象サーバー**の二層構造をとる。

### 1.2. エントリーサーバーの責務

エントリーサーバーは、Webクライアントからの全てのリクエストを受け付ける単一のエントリーポイントであり、以下の役割を担う。

1.  **静的ファイル配信**: Webクライアント（SPA）のアセットホスティング。
2.  **インベントリ管理**: 管理対象サーバーの接続情報、認証情報、タグ等の永続化。
3.  **API Gateway/Proxy**: クライアントからの操作リクエストを管理対象サーバーへ安全に中継。
4.  **データ集約 (Aggregator)**: 複数の管理対象サーバーからの情報を一元的に収集し、クライアントへ配信。
5.  **通信多重化 (Multiplexing)**: 複数のクライアントからの同一サーバーに対するリアルタイムデータ要求を単一のバックエンド接続に集約し、管理対象サーバーへの負荷を軽減する。

## 2\. アーキテクチャとプロトコル

### 2.1. データ取得・配信ロジック (Multiplexing)

リソース監視データは、**監視マネージャー**によるPub/Subモデル（または内部ストリーム）を通じて処理される。

| 対象 | クライアント \<-\> エントリーサーバー | エントリーサーバー \<-\> 管理対象サーバー |
| :--- | :--- | :--- |
| **監視データ** | HTTP (SSE) | REST API または SSH/Command Execution (ポーリング) |
| **操作/設定** | HTTPS (REST) | REST API または SSH/Command Execution (リクエスト毎) |
| **ターミナル/ログ** | WebSocket | SSH (対話セッション/ `tail -f`) |

### 2.2. 技術要件

* **通信プロトコル**: HTTPS (REST), SSE, WSS (WebSocket)
* **認証**: クライアント \<-\> エントリーサーバー間はセッショントークンまたはJWT。
* **認証情報**: 管理対象サーバーへの認証情報は、保存時に強度の高い暗号化（例: AES-256）を施し、環境変数や設定ファイルではなく安全なデータベースに格納する。

## 3\. APIエンドポイント定義

### 3.1. 静的ファイル配信 (Static Serving)

| Method | Path | Description |
| :--- | :--- | :--- |
| `GET` | `/*` | SPAアセット配信（APIルート除く） |

### 3.2. サーバーインベントリ管理 (Tier 1: サーバー監視ダッシュボード)

| Method | Path | Description | Data Model |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/v1/servers` | サーバー一覧取得 | `Server[]` |
| `GET` | `/api/v1/servers/{id}` | サーバー詳細情報取得 | `Server` |
| `POST` | `/api/v1/servers` | 新規サーバー登録 | `Server` (入力フィールド) |
| `PUT` | `/api/v1/servers/{id}` | サーバー情報更新 | `Server` (更新フィールド) |
| `DELETE`| `/api/v1/servers/{id}` | サーバー削除 | - |

### 3.3. リアルタイム監視 (Tier 1: 監視ダッシュボード) - SSE採用

#### エンドポイント

| Method | Path | Description | Content Type |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/v1/servers/{id}/monitor/stream` | リソース監視データのストリーミング | `text/event-stream` |

#### イベントフォーマット (`event: ResourceUpdate`)

```json
// dataフィールドの内容
{
  "server_id": "uuid",
  "timestamp": 1715400000,
  "data": {
    "cpu": 45.5,
    "memory_used_mib": 4096,
    "memory_total_mib": 8192,
    "disk_usage_percent": 50,
    "status": "online" // dead or alive (Ping/SSH check result)
  }
}
```

### 3.4. リモート操作・情報収集 (REST - Tier 1 & 2)

#### サーバー詳細情報収集 (Tier 1 & 2)

これらのエンドポイントは、リクエスト時にエントリーサーバーが管理対象サーバーへ接続し、データを取得・整形して返却する。

| Method | Path | Description | Query Parameters |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/v1/servers/{id}/specs` | 基本スペック（CPU/Memory/Disk構成） | - |
| `GET` | `/api/v1/servers/{id}/packages` | インストール済みパッケージ一覧 | `?query=nginx` |
| `GET` | `/api/v1/servers/{id}/processes` | 実行中プロセス一覧 | `?sort=cpu_desc&limit=50` |
| `GET` | `/api/v1/servers/{id}/files` | ファイル/ディレクトリ一覧 (Tier 2) | `?path=/etc/nginx` |
| `GET` | `/api/v1/servers/{id}/file` | ファイル内容の取得 (Tier 2) | `?path=/etc/nginx/nginx.conf` |

#### サーバー操作系 (Tier 2)

| Method | Path | Description | Request Body (例) |
| :--- | :--- | :--- | :--- |
| `POST` | `/api/v1/servers/{id}/file` | ファイルの作成/編集 (Tier 2) | `{ path: "/tmp/test", content: "..." }` |
| `POST` | `/api/v1/servers/{id}/packages` | パッケージ操作 | `{ name: "vim", action: "install" }` |
| `POST` | `/api/v1/servers/{id}/service` | サービス操作 | `{ name: "nginx", action: "restart" }` |
| `POST` | `/api/v1/servers/{id}/command` | 単一コマンド実行 | `{ command: "uptime" }` |
| `POST` | `/api/v1/servers/{id}/wol` | Wake-on-LAN | - |
| `POST` | `/api/v1/servers/batch-command` | コマンド一括実行 | `{ server_ids: ["id1", "id2"], command: "df -h" }` |

### 3.5. ストリーミング操作 (WebSocket / SSE)

#### 3.5.1. リモートターミナル (Tier 1: SSHクライアント機能) - WSS採用

双方向の対話型通信が必要なため、WebSocketを採用。

| Protocol | Path | Description |
| :--- | :--- | :--- |
| WSS | `/ws/v1/servers/{id}/terminal` | 対話型SSHセッションを提供 (Web Terminal) |

* **Client -\> Server**: `{ type: "input", data: "ls -l\r" }`, `{ type: "resize", rows: 40, cols: 80 }`
* **Server -\> Client**: `{ type: "output", data: "..." }`

#### 3.5.2. ログビューア (Tier 1: ログビューア) - SSE採用

| Method | Path | Description | Content Type |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/v1/servers/{id}/logs/stream` | `tail -f` 相当のログストリーミング | `text/event-stream` |

#### イベントフォーマット (`event: LogLine`)

```json
// dataフィールドの内容
{
  "server_id": "uuid",
  "line": "Dec 12 10:00:00 web-prod-01 nginx: [error] access denied"
}
```

## 4\. データモデル (永続化)

### 4.1. Server (サーバー定義)

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `id` | UUID | サーバー識別子 |
| `hostname` | string | ホスト名 |
| `ip_address` | string | 接続IPアドレス |
| `os_type` | string | OSタイプ (e.g., `ubuntu`, `centos`) |
| `tags` | string[] | 用途タグ (e.g., `production`, `web`) |
| `auth_profile_id`| UUID | 認証情報 (`AuthProfile`) への外部キー |
| `port` | integer | SSHポート (Default 22) |
| `bastion_server_id`| UUID (Optional) | 踏み台サーバーID |
| `wol_mac_address` | string (Optional) | Wake-on-LAN用MACアドレス |

### 4.2. AuthProfile (認証情報)

| フィールド | 型 | 説明 | セキュリティ |
| :--- | :--- | :--- | :--- |
| `id` | UUID | プロファイル識別子 | - |
| `name` | string | プロファイル名 | - |
| `type` | enum | `key` または `password` | - |
| `username` | string | 接続ユーザー名 | - |
| `encrypted_secret`| string | 暗号化された秘密鍵またはパスワード | **At Rest Encryption必須** |

### 4.3. CommandPreset (コマンドプリセット)

| フィールド | 型 | 説明 |
| :--- | :--- | :--- |
| `id` | UUID | プリセット識別子 |
| `name` | string | プリセット名 (例: `Disk Usage`) |
| `command` | string | 実行コマンド (例: `df -h`) |

## 5\. 非機能要件

### 5.1. パフォーマンス・スケーラビリティ

* **多重化効率**: 監視マネージャーは、データ取得頻度の調整（例: 5秒ごと）や、データが変化した場合のみプッシュするロジックを実装し、帯域を最適化する。
* **タイムアウト**: 管理対象サーバーへの接続失敗や応答遅延は、クライアントリクエストをブロックしないよう非同期処理で行い、適切なタイムアウト値を設定する。

### 5.2. セキュリティ

* **暗号化**: サーバー間通信、およびデータベース内の機密情報（`AuthProfile.encrypted_secret`）は、業界標準の暗号化手法で保護する。
* **権限分離**: サーバー操作系API（`POST`, `PUT`, `DELETE`）は、権限を持つユーザーのみが実行可能であることを確認する。

### 5.3. 運用・保守

* **ロギング**: エントリーサーバーは、全てのAPIリクエスト、管理対象サーバーへの操作履歴、および監視マネージャーの状態遷移を詳細にログに出力する。