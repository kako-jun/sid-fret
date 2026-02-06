# sid-fret 開発者向けドキュメント

ベースギター特化の音楽理論ライブラリ（Rust/WASM）

## 背景

- **元プロジェクト**: sid-note（Next.js/TypeScript）
- **分離理由**: TypeScript実装にenharmonic判定のバグがあり、Rust移行を決定
- **役割**: sid-noteから音楽理論ライブラリ部分を抽出し、独立したWASMモジュールとして提供

## プロジェクト構造

```
src/
├── lib.rs                    # エントリーポイント、WASM初期化
├── core/                     # 楽器非依存の音楽理論
│   ├── pitch.rs              # 音名・ピッチの統一基盤（12音配列、半音計算、異名同音比較）
│   ├── interval.rs           # インターバル計算（半音距離、転回形判定）
│   ├── chord_type.rs         # コード構成音定義（22コードタイプ、ChordTone構造体）
│   └── scale_type.rs         # スケール定義（12スケール、48キーマップ）
├── harmony/                  # 和声理論（楽器非依存）
│   ├── diatonic.rs           # ダイアトニックコード生成（triad + 7th）
│   ├── functional.rs         # 機能和声分析（I-VII度数、進行分析）
│   └── cadence.rs            # カデンツ判定（7パターン + 3コード拡張）
├── instrument/               # ベースギター固有
│   ├── tuning.rs             # チューニング定義（4プリセット）
│   ├── fretboard.rs          # フレットボード計算（Position、ポジション生成）
│   └── fingering/            # 運指アルゴリズム
│       ├── algorithm.rs      # 5種類の運指アルゴリズム
│       ├── position.rs       # FretPosition, FingeringPattern
│       └── scoring.rs        # スコアリング重み付け
└── utils/                    # ユーティリティ
    ├── chromatic.rs           # 半音関係判定
    ├── chord_alias.rs         # コード名エイリアス（別表記一覧）
    └── notation.rs            # 表記ユーティリティ（get_line、value_text、scale_text）
```

## 設計原則

**「音楽理論」と「楽器固有」の分離**
- `core/` — 楽器に依存しない音楽理論（ピッチ、インターバル、コード定義、スケール定義）
- `harmony/` — 楽器に依存しない和声理論（ダイアトニック、機能和声、カデンツ）
- `instrument/` — ベースギター固有の処理（フレットボード、チューニング、運指）
- `utils/` — 表示・変換ユーティリティ

## 依存クレート

- `wasm-bindgen` 0.2: WASM連携
- `serde` 1.0 + `serde_json` 1.0: シリアライゼーション
- `serde-wasm-bindgen` 0.6: WASM-JS間のデータ変換

外部の音楽理論ライブラリには依存せず、全て自前実装。

## アーキテクチャ

```
sid-note (TypeScript/Next.js)
    ↓ WASM import
sid-fret (本ライブラリ)
    ├── core/      → 楽器非依存の音楽理論
    ├── harmony/   → 和声理論
    ├── instrument/ → ベースギター固有
    └── utils/     → ユーティリティ
```

## 主要なデータ構造

### ChordTone（core/chord_type.rs）
```rust
struct ChordTone {
    interval: String,   // "1", "♭3", "5" 等
    semitones: i32,     // 0, 3, 7 等
}
```

### Position（instrument/fretboard.rs）
```rust
struct Position {
    string: i32,       // 弦番号（1=最高音弦）
    fret: i32,         // フレット番号（0=開放弦）
    pitch: String,     // "C2", "G＃3" 等
    interval: String,  // "1", "♭3" 等
}
```

### Tuning（instrument/tuning.rs）
```rust
struct StringDef { open_note: String, offset: i32 }
struct Tuning { name: String, strings: Vec<StringDef>, max_fret: i32 }
```

## 運指アルゴリズム

1. **shortest**: 前の音からの移動距離（フレット+弦）を最小化
2. **position-stable**: 指定ポジション付近を維持
3. **string-priority**: 弦移動を優先、フレット移動を避ける
4. **open-string**: 開放弦（fret=0）を最優先
5. **balanced**: 上記を試行し最もスコアが低いものを選択

## ベースフレット計算

### 4弦ベース標準チューニング

| 弦 | 名前 | 開放音 | フレット範囲（半音） |
|----|------|--------|---------------------|
| 1弦 | G | G2 | 15-39 |
| 2弦 | D | D2 | 10-34 |
| 3弦 | A | A1 | 5-29 |
| 4弦 | E | E1 | 0-24 |

### ルート音オフセット

`fret_offset(root)` で計算: `(note_to_semitone(root) - 4 + 12) % 12`

## ビルド・テスト

```bash
cargo build           # デバッグビルド
cargo build --release # リリースビルド
cargo test            # テスト実行（56件）
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

### 高度な運指最適化
- スライド優先アルゴリズム
- フレーズ指向（グループ化）アルゴリズム
- 指の負担分散アルゴリズム
