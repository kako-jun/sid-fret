# CI/CD セットアップドキュメント

## 概要

sid-fretプロジェクトに以下のCI/CD機能を実装しました：
- **Husky + lint-staged**: コミット前の自動lint/format
- **GitHub Actions CI**: テスト、lint、WASM ビルド
- **GitHub Actions Release**: リリース時の自動デプロイ

## 1. Pre-commit Hooks (Husky)

### インストール済みツール
```json
{
  "devDependencies": {
    "husky": "^8.0.3",
    "lint-staged": "^15.5.2"
  }
}
```

### 設定ファイル

#### package.json
```json
{
  "scripts": {
    "prepare": "husky install",
    "lint": "cargo clippy -- -D warnings",
    "format": "cargo fmt",
    "format:check": "cargo fmt -- --check",
    "test": "cargo test",
    "build:wasm": "wasm-pack build --target web --out-dir pkg",
    "build:wasm:nodejs": "wasm-pack build --target nodejs --out-dir pkg-node"
  },
  "lint-staged": {
    "*.rs": [
      "cargo fmt --",
      "cargo clippy --fix --allow-dirty --allow-staged --"
    ]
  }
}
```

#### .husky/pre-commit
```bash
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

npx lint-staged
```

### 動作
1. `git commit` 実行時
2. huskyが `.husky/pre-commit` を実行
3. lint-stagedが変更された `.rs` ファイルに対して：
   - `cargo fmt` で自動フォーマット
   - `cargo clippy --fix` で自動修正
4. エラーがあればコミット中断

## 2. GitHub Actions CI

### ワークフロー: `.github/workflows/ci.yml`

#### トリガー
- `main`, `develop`, `claude/**` ブランチへのpush
- `main`, `develop` へのPull Request

#### ジョブ

##### 1. Test
```yaml
- Run tests
- Cache cargo registry, git, target
```

##### 2. Format Check
```yaml
- Check code formatting with rustfmt
```

##### 3. Clippy
```yaml
- Run clippy linter
- Fail on warnings (-D warnings)
```

##### 4. WASM Build
```yaml
- Build WASM for web target
- Build WASM for nodejs target
- Upload artifacts (pkg/, pkg-node/)
```

### ステータスバッジ（推奨）
```markdown
[![CI](https://github.com/kako-jun/sid-fret/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/sid-fret/actions/workflows/ci.yml)
```

## 3. GitHub Actions Release

### ワークフロー: `.github/workflows/release.yml`

#### トリガー
```yaml
on:
  push:
    tags:
      - 'v*'
```

#### リリース手順
1. タグをpush:
```bash
git tag v0.1.0
git push origin v0.1.0
```

2. 自動実行:
   - WASMビルド（web/nodejs）
   - tarball作成
   - GitHub Release作成
   - WASM成果物を添付
   - crates.io公開（オプション）

#### 成果物
- `sid-fret-wasm-web.tar.gz`: Web向けWASM
- `sid-fret-wasm-nodejs.tar.gz`: Node.js向けWASM

#### crates.io公開設定

**必要な準備:**
1. crates.ioでAPIトークン取得
2. GitHubリポジトリのSecrets設定:
   - Settings > Secrets and variables > Actions
   - `CARGO_TOKEN` を追加

**無効化する場合:**
`.github/workflows/release.yml` の `publish-crate` ジョブをコメントアウト

## 4. Rustfmt設定

### rustfmt.toml
```toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
reorder_imports = true
reorder_modules = true
```

### 手動実行
```bash
cargo fmt
cargo fmt -- --check  # Check only
```

## 5. Clippy設定

### clippy.toml
```toml
too-many-arguments-threshold = 7
type-complexity-threshold = 250
```

### 手動実行
```bash
cargo clippy
cargo clippy -- -D warnings  # Fail on warnings
cargo clippy --fix  # Auto-fix
```

## 6. WASM ビルド

### Web向け
```bash
npm run build:wasm
# または
wasm-pack build --target web --out-dir pkg
```

生成物: `pkg/`
- `sid_fret.js`
- `sid_fret_bg.wasm`
- `sid_fret.d.ts` (TypeScript定義)
- `package.json`

### Node.js向け
```bash
npm run build:wasm:nodejs
# または
wasm-pack build --target nodejs --out-dir pkg-node
```

生成物: `pkg-node/`
- `sid_fret.js`
- `sid_fret_bg.wasm`
- `sid_fret.d.ts`
- `package.json`

## 7. TypeScript連携

### sid-noteプロジェクトでの使用

#### インストール
```bash
# リリースからダウンロード
mkdir -p public/wasm
cd public/wasm
curl -L https://github.com/kako-jun/sid-fret/releases/latest/download/sid-fret-wasm-web.tar.gz | tar xz
```

#### 使用例
```typescript
import init, { calculate_fingering } from '@/public/wasm/sid_fret';

await init();
const pattern = calculate_fingering(new Uint8Array([0, 3, 5, 7]), "shortest");
```

詳細: `examples/typescript-usage.md` を参照

## 8. トラブルシューティング

### Pre-commit hookが動かない
```bash
# huskyを再インストール
rm -rf .husky node_modules
npm install
npx husky install
```

### CIでキャッシュエラー
```bash
# ローカルでキャッシュをクリア
cargo clean
```

### WASM ビルドエラー
```bash
# wasm-packを最新化
cargo install wasm-pack --force

# wasm32ターゲット追加
rustup target add wasm32-unknown-unknown
```

### Clippy警告が多すぎる
```bash
# 自動修正を適用
cargo clippy --fix --allow-dirty

# 特定のlintを無効化（src/lib.rsに追加）
#![allow(clippy::module_name_repetitions)]
```

## 9. ローカル開発フロー

### 推奨ワークフロー
```bash
# 1. 開発
# ... コード編集 ...

# 2. フォーマット・Lint
npm run format
npm run lint

# 3. テスト
npm test

# 4. コミット（自動でlint/format実行）
git add .
git commit -m "Your message"

# 5. WASM動作確認（必要に応じて）
npm run build:wasm
```

### CI通過前チェックリスト
- [ ] `cargo test` パス
- [ ] `cargo fmt -- --check` パス
- [ ] `cargo clippy -- -D warnings` パス
- [ ] `wasm-pack build` 成功

## 10. リリースプロセス

### バージョンアップ手順

1. **Cargo.toml更新**
```toml
[package]
version = "0.2.0"  # バージョンアップ
```

2. **CHANGELOG更新**（作成する場合）
```markdown
## [0.2.0] - 2025-11-17
### Added
- New fingering algorithms
### Changed
- Performance improvements
```

3. **コミット**
```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.0"
```

4. **タグ作成**
```bash
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

5. **自動デプロイ確認**
- GitHub Actions > Release workflow確認
- Releases ページで成果物確認

## 11. セキュリティ

### Secrets管理
- `CARGO_TOKEN`: crates.io API token
- リポジトリSettings > Secrets に保存
- **絶対にコードにコミットしない**

### 依存関係更新
```bash
cargo update
cargo audit  # セキュリティ監査
```

## 12. メンテナンス

### 定期タスク
- [ ] 月次: 依存関係更新
- [ ] 四半期: Rust toolchain更新
- [ ] 随時: GitHub Actions version更新

### モニタリング
- GitHub Actions実行結果
- crates.io ダウンロード数
- Issue/PR状況

---

**最終更新**: 2025-11-17
**担当**: kako-jun
