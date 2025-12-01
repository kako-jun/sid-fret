# sid-fret

ベースギター特化の音楽理論ライブラリ（Rust/WASM）

## インストール

### Rustライブラリとして

```toml
[dependencies]
sid-fret = "0.1"
```

### WebAssemblyとして

```bash
wasm-pack build --target web
```

## 機能

- **運指アルゴリズム**: 5種類のモードから選択可能
- **ベースフレット計算**: 音程から4弦ベースのポジションを計算
- **機能和声分析**: I-VII度数判定、カデンツ検出
- **日本語記譜対応**: 全角シャープ（＃）、フラット（♭）

## 使い方

### 運指計算

```rust
use sid_fret::fingering::*;

let pitches = vec![0, 3, 5, 7]; // E-G-A-B

// 最短移動
let pattern = calculate_shortest_path(&pitches);

// ポジション固定（5フレット付近）
let pattern = calculate_position_stable(&pitches, 5);

// 開放弦活用
let pattern = calculate_open_string(&pitches);

// バランス型（複数要素を総合評価）
let pattern = calculate_balanced(&pitches);
```

### 運指モード

| モード | 説明 |
|--------|------|
| `shortest` | フレット移動距離を最小化 |
| `position-stable` | 特定ポジション内で演奏を完結 |
| `string-priority` | 横移動より縦移動を優先 |
| `open-string` | 開放弦を積極的に使用 |
| `balanced` | 複数要素をスコアリングして最適化 |

### フレット計算

```rust
use sid_fret::chord::*;

let offset = get_fret_offset("C"); // ルート音のオフセット

let frets = get_frets(
    false, // minor 3rd
    false, // sus4
    false, // dim 5th
    false, // maj 7th
    false, // min 7th
    false  // aug 7th
);

let positions = convert_frets_to_positions(&frets, offset);
```

### 機能和声分析

```rust
use sid_fret::harmony::*;

let degree = get_functional_harmony("C", "G"); // 5 (V)
let cadence = cadence_text(5, 1); // "Perfect Cadence"
```

### WASM API

```javascript
import init, { calculate_fingering } from './pkg/sid_fret.js';

await init();
const pattern = calculate_fingering([0, 3, 5, 7], "shortest");
console.log(pattern.positions);
```

## ビルド

```bash
cargo build --release  # Rustライブラリ
cargo test             # テスト実行
wasm-pack build        # WASMビルド
```

## ライセンス

MIT
