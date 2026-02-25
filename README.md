# rust-wolf

チャット人狼ゲームシステム

## プロジェクト構成

```
rust-wolf/
├── backend/       # Rust (actix-web) API サーバー
├── frontend/      # Vite + Svelte フロントエンド
└── packages/
    └── ui/        # UIコンポーネント集 (@rust-wolf/ui)
```

## セットアップ

```bash
pnpm install          # フロントエンド + UI パッケージの依存をインストール
cd backend && cargo build  # バックエンドのビルド
```

## 起動方法

### バックエンド

```bash
cd backend
cargo run
# → http://localhost:8080
```

### フロントエンド

```bash
cd frontend
pnpm dev
# → http://localhost:5173 (APIは http://localhost:8080 にプロキシ)
```

### Storybook (UIコンポーネントカタログ)

```bash
cd packages/ui
pnpm storybook
# → http://localhost:6006
```
