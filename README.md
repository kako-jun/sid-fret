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

### 🎯 運指アルゴリズム（複数モード選択可能）
- **最短移動 (shortest)**: フレット移動距離を最小化
- **ポジション固定 (position-stable)**: 特定ポジション内で演奏を完結
- **弦移動優先 (string-priority)**: 横移動より縦移動を優先
- **開放弦活用 (open-string)**: 開放弦を積極的に使用
- **バランス型 (balanced)**: 複数要素をスコアリングして最適化

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

### 運指計算

```rust
use sid_fret::fingering::*;

// 音程シーケンス（半音階の絶対位置）
let pitches = vec![0, 3, 5, 7]; // E-G-A-B

// 最短移動アルゴリズム
let pattern = calculate_shortest_path(&pitches);
println!("移動距離: {}", pattern.total_movement());

// ポジション固定アルゴリズム（5フレット付近）
let pattern = calculate_position_stable(&pitches, 5);
println!("ポジション変更: {}", pattern.position_changes());

// 開放弦活用アルゴリズム
let pattern = calculate_open_string(&pitches);
println!("開放弦使用: {}", pattern.open_string_count());

// バランス型（複数要素を総合評価）
let pattern = calculate_balanced(&pitches);
println!("スコア: {}", pattern.score);

// WASM API経由
let result = calculate_fingering(pitches, "shortest");
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

## 運指アルゴリズムの詳細

### 1. 最短移動 (shortest)
前の音からの移動距離（フレット+弦）を最小化。シンプルで素早い演奏に適しています。

### 2. ポジション固定 (position-stable)
指定したポジション（例：5フレット付近）を基準に、できるだけその範囲で運指を完結。安定したフォームを維持できます。

### 3. 弦移動優先 (string-priority)
フレット移動よりも弦移動を優先。ベースは弦間移動が比較的楽なため、ポジションを大きく変えずに弦を切り替えます。

### 4. 開放弦活用 (open-string)
開放弦を積極的に使用することで、ポジション移動を減らします。ルート音や低音で特に有効です。

### 5. バランス型 (balanced)
以下の要素をスコアリングして総合評価：
- 移動距離（短いほど良い）
- ポジション安定性
- 開放弦の利用
- 弦移動の頻度

各アルゴリズムの重み付けは`AlgorithmWeights`で調整可能です。

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

全29テストが実装されています：
- ベースフレット計算: 6テスト
- 運指アルゴリズム: 12テスト
- 機能和声分析: 4テスト
- ユーティリティ: 7テスト

## ライセンス

MIT

## 作者

kako-jun
