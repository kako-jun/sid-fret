# sid-fret

ベースギター特化の音楽理論ライブラリ（Rust/WASM）

## 概要

sid-fretは、ベース演奏に特化した音楽理論計算をRust+WebAssemblyで提供するライブラリです。
既存の音楽理論ライブラリ（rust-music-theory）にはない機能を実装しています。

## 特徴

### 🎸 ベース特化機能
- **4弦ベース用フレット計算**: 音程からフレットポジションへの変換
- **ベース弦マッピング**: 標準チューニング（E1, A1, D2, G2）対応
- **オクターブ展開**: 4オクターブ範囲のポジション生成

### 🎵 機能和声分析
- **度数判定**: I-VII度の機能和声判定
- **カデンツ検出**: Perfect/Plagal/Deceptive/Half/Phrygian Cadence
- **ローマ数字記譜**: Ⅰ-Ⅶのローマ数字表記と機能名

### 📝 日本語記譜対応
- **全角記号対応**: ＃（全角シャープ）、♭（フラット）
- **コード名エイリアス**: Cmaj7 ⇔ CM7 ⇔ C△7 など

## 依存関係

- [rust-music-theory](https://github.com/ozankasikci/rust-music-theory): 基本的な音楽理論（Note, Chord, Scale, Interval）
- **kordには依存しません**: シンプルな依存構成

## API

### ベースフレット計算

```rust
use sid_fret::chord::*;

// ルート音から半音オフセットを取得
let offset = get_fret_offset("C"); // 0

// コード構成音からフレット配列を生成
let frets = get_frets(
    false, // has_minor_3rd
    false, // has_sus4
    false, // has_dim_5th
    false, // has_maj_7th
    false, // has_min_7th
    false  // has_aug_7th
);

// フレット配列をベースの4弦ポジションに変換
let positions = convert_frets_to_positions(&frets, offset);
```

### 機能和声分析

```rust
use sid_fret::harmony::*;

// 機能和声の度数を取得
let degree = get_functional_harmony("C", "G"); // 5 (V)

// カデンツを判定
let cadence = cadence_text(5, 1); // "Perfect Cadence" (V→I)

// ローマ数字記譜を取得
let text = functional_harmony_text(5); // "Ⅴ Dominant"
```

### ユーティリティ

```rust
use sid_fret::utils::*;

// 半音階判定
let is_chromatic = is_chromatic_note(
    Some("C2".to_string()),
    Some("C＃2".to_string())
); // true

// コード名エイリアス
let aliases = get_chord_name_aliases("Cmaj7");
// ["Cmaj7", "CM7", "C△7"]
```

## ビルド

### 通常のRustライブラリとして

```bash
cargo build --release
cargo test
```

### WebAssemblyとして

```bash
# wasm-packのインストール
cargo install wasm-pack

# WASMビルド
wasm-pack build --target web

# Node.js用
wasm-pack build --target nodejs
```

## テスト

```bash
cargo test
```

全17テストが実装されています。

## ライセンス

MIT

## 作者

kako-jun
