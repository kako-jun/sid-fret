# sid-fret 開発者向けドキュメント

ベースギター特化の音楽理論ライブラリ（Rust/WASM）

## 背景

- **元プロジェクト**: sid-note（Next.js/TypeScript）
- **分離理由**: TypeScript実装にenharmonic判定のバグがあり、Rust移行を決定
- **役割**: sid-noteから音楽理論ライブラリ部分を抽出し、独立したWASMモジュールとして提供

## プロジェクト構造

```
src/
├── lib.rs              # エントリーポイント、WASM初期化
├── chord/
│   └── fret.rs         # ベースフレット計算、4弦マッピング
├── fingering/
│   ├── algorithm.rs    # 5種類の運指アルゴリズム
│   ├── position.rs     # FretPosition, FingeringPattern
│   └── scoring.rs      # スコアリング重み付け
├── harmony/
│   ├── functional.rs   # 機能和声分析（I-VII度数）
│   └── cadence.rs      # カデンツ判定
├── utils/
│   ├── chromatic.rs    # 半音階判定
│   └── chord_alias.rs  # コード名エイリアス
├── core/               # rust-music-theoryへの委譲
└── scale/              # rust-music-theoryへの委譲
```

## 依存クレート

- `wasm-bindgen` 0.2: WASM連携
- `serde` 1.0 + `serde_json` 1.0: シリアライゼーション
- `serde-wasm-bindgen` 0.6: WASM-JS間のデータ変換
- `rust-music-theory` 0.2: 基本的な音楽理論（Note, Scale, Interval）

## アーキテクチャ

```
sid-note (TypeScript/Next.js)
    ↓ WASM import
sid-fret (本ライブラリ)
    ├── 独自実装: 運指、フレット計算、機能和声
    └── rust-music-theory活用: Note, Scale, Interval
```

sid-noteはsid-fretのみを呼ぶ。rust-music-theoryはsid-fret内部で使用。

## 運指アルゴリズム

### データ構造

```rust
// 単一ポジション
struct FretPosition {
    string: u8,      // 1=G弦, 2=D弦, 3=A弦, 4=E弦
    fret: u8,        // 0=開放弦
    finger: Option<u8>, // 1-4 (人差し指-小指)
}

// 運指パターン
struct FingeringPattern {
    positions: Vec<FretPosition>,
    score: f32,
    algorithm: String,
}
```

### アルゴリズム実装

1. **shortest**: 前の音からの移動距離（フレット+弦）を最小化
2. **position-stable**: 指定ポジション付近を維持
3. **string-priority**: 弦移動を優先、フレット移動を避ける
4. **open-string**: 開放弦（fret=0）を最優先
5. **balanced**: 上記を試行し最もスコアが低いものを選択

### スコアリング

```rust
struct AlgorithmWeights {
    movement_weight: f32,        // 移動距離
    position_change_weight: f32, // ポジション変更
    open_string_weight: f32,     // 開放弦使用（負の重み=ボーナス）
    string_change_weight: f32,   // 弦移動
}
```

## ベースフレット計算

### 4弦ベース標準チューニング

| 弦 | 名前 | 開放音 | フレット範囲（半音） |
|----|------|--------|---------------------|
| 1弦 | G | G2 | 15-39 |
| 2弦 | D | D2 | 10-34 |
| 3弦 | A | A1 | 5-29 |
| 4弦 | E | E1 | 0-24 |

### ルート音オフセット

```rust
"C" => 0, "C#" => 1, "D" => 2, ... "B" => 11
```

日本語記号（＃、♭）にも対応。

## 機能和声分析

### ダイアトニックコード

24キー対応（12メジャー + 12マイナー）

```rust
"C"  => ["C", "Dm", "Em", "F", "G", "Am", "Bdim"]
"Am" => ["Am", "Bdim", "C", "Dm", "Em", "F", "G"]
```

### カデンツパターン

| 進行 | 名称 |
|------|------|
| V → I | Perfect Cadence |
| IV → I | Plagal Cadence |
| V → VI | Deceptive Cadence |
| * → V | Half Cadence |

## ビルド・テスト

```bash
cargo build           # デバッグビルド
cargo build --release # リリースビルド
cargo test            # テスト実行（29件）
cargo clippy          # Lint
cargo fmt             # フォーマット

# WASM
wasm-pack build --target web      # Web向け
wasm-pack build --target nodejs   # Node.js向け
```

## CI/CD

- **pre-commit**: Husky + lint-staged（cargo fmt, clippy）
- **GitHub Actions CI**: テスト、Lint、WASMビルド
- **Release**: タグプッシュで自動リリース

## WASM最適化

```toml
[profile.release]
opt-level = "z"  # サイズ最適化
lto = true       # Link Time Optimization
```

## 将来の拡張

### Phase 2: 高度な運指最適化
- スライド優先アルゴリズム
- フレーズ指向（グループ化）アルゴリズム
- 指の負担分散アルゴリズム

### Phase 3: 多様な楽器対応
- 5弦/6弦ベース対応
- カスタムチューニング（Drop D等）
- フレットレスベース対応
