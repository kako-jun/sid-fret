# sid-fret アーキテクチャと依存関係

## 全体構成図

```
┌─────────────────────────────────────────────────┐
│  sid-note (Next.js / TypeScript)                │
│  - UIレイヤー                                    │
│  - ユーザー入力処理                              │
│  - 譜面表示                                      │
└─────────────────┬───────────────────────────────┘
                  │ import WASM
                  │ (calculate_fingering, etc.)
                  ↓
┌─────────────────────────────────────────────────┐
│  sid-fret (Rust → WASM)                         │
│  ┌───────────────────────────────────────────┐  │
│  │ 独自実装部分（既存ライブラリにない）      │  │
│  │ - ベース特化フレット計算                  │  │
│  │ - 運指アルゴリズム (5種類)               │  │
│  │ - 機能和声・カデンツ分析                  │  │
│  │ - 日本語記譜対応                          │  │
│  └───────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────┐  │
│  │ rust-music-theory を使う部分              │  │
│  │ - Note（音符）の基本構造                  │  │
│  │ - Scale（スケール）理論                   │  │
│  │ - Interval（音程）計算                    │  │
│  └───────────────────────────────────────────┘  │
└─────────────────┬───────────────────────────────┘
                  │ uses as dependency
                  │ (Cargo.toml)
                  ↓
┌─────────────────────────────────────────────────┐
│  rust-music-theory (既存の著名ライブラリ)       │
│  - Note, Chord, Scale, Interval                 │
│  - 基本的な音楽理論                             │
│  - sid-fretはこれに依存                         │
└─────────────────────────────────────────────────┘
```

## データフロー

### sid-note → sid-fret の呼び出し

```typescript
// sid-note (TypeScript)
import init, { calculate_fingering } from './wasm/sid_fret';

await init(); // WASM初期化

// ユーザーが音符入力
const notes = [0, 3, 5, 7]; // E, G, A, B

// sid-fret を呼ぶ
const pattern = calculate_fingering(notes, "balanced");
// → sid-fretの中で処理される

// 結果を表示
console.log(pattern.positions); // [{string: 4, fret: 0}, ...]
```

### sid-fret 内部での rust-music-theory の使用

```rust
// sid-fret の内部実装
use rust_music_theory::note::Note;
use rust_music_theory::scale::Scale;

// rust-music-theory を使う
pub fn get_scale_notes(scale_name: &str) -> Vec<Note> {
    // rust-music-theory の機能を使う
    Scale::new(scale_name).notes()
}

// 独自実装（rust-music-theoryにない）
pub fn calculate_fingering(pitches: &[u8], mode: &str) -> FingeringPattern {
    // これは sid-fret のオリジナル機能
    // rust-music-theory は使わない
    match mode {
        "shortest" => calculate_shortest_path(pitches),
        "balanced" => calculate_balanced(pitches),
        _ => calculate_shortest_path(pitches),
    }
}
```

## 重要なポイント

### ❌ 間違った理解
```
sid-note → rust-music-theory → sid-fret
```
**sid-noteはrust-music-theoryを直接呼ばない**

### ✅ 正しい理解
```
sid-note → sid-fret (内部でrust-music-theoryを使用)
```

### sid-fretの役割

1. **ラッパー機能**
   - rust-music-theoryの機能を必要に応じて使う
   - TypeScript/JavaScriptから使いやすいWASM APIを提供

2. **拡張機能**
   - rust-music-theoryにない機能を追加実装
   - ベース特化、運指計算など

3. **統合レイヤー**
   - rust-music-theoryの基本機能 + 独自機能
   - 一つのWASMモジュールとして提供

## 具体的な依存関係

### Cargo.toml（sid-fret）
```toml
[dependencies]
wasm-bindgen = "0.2"
rust-music-theory = "0.2"  # ← 依存している
```

### package.json（sid-note）
```json
{
  "dependencies": {
    // rust-music-theoryは入っていない
    // WASMファイルを直接使う
  }
}
```

## 使い分け

### rust-music-theory を使う部分（sid-fret内）
```rust
// core/mod.rs, scale/mod.rs
pub use rust_music_theory::note::*;
pub use rust_music_theory::scale::*;
```
- Note構造体
- Scale理論
- 基本的なInterval計算

### 独自実装する部分（sid-fret）
```rust
// fingering/, chord/, harmony/, utils/
```
- ベースフレット計算（rust-music-theoryにない）
- 運指アルゴリズム（rust-music-theoryにない）
- 機能和声分析（rust-music-theoryにない）
- カデンツ判定（rust-music-theoryにない）

## なぜこの構成？

### 車輪の再発明を避ける
```rust
// ❌ 悪い例：Noteを再実装
pub struct Note {
    pitch: String,
    octave: u8,
}

// ✅ 良い例：rust-music-theoryを使う
use rust_music_theory::note::Note;
```

### 既存にない機能を追加
```rust
// rust-music-theoryにはない → 独自実装
pub fn calculate_fingering() { ... }
pub fn get_functional_harmony() { ... }
pub fn cadence_text() { ... }
```

## 出力と入力の関係

### sid-noteからの視点

```typescript
// 1. ユーザー入力（sid-note）
const userInput = {
  notes: ["E2", "G2", "A2", "B2"],
  mode: "balanced"
};

// 2. 前処理（sid-note TypeScript）
const pitches = convertNotesToPitches(userInput.notes);
// → [0, 3, 5, 7]

// 3. sid-fretを呼ぶ（WASM）
const pattern = calculate_fingering(pitches, userInput.mode);
// sid-fretの内部で：
//   - rust-music-theoryでNote理論を使う
//   - 独自の運指アルゴリズムを実行
//   - FingeringPatternを返す

// 4. 結果を使う（sid-note TypeScript）
pattern.positions.forEach(pos => {
  displayOnFretboard(pos.string, pos.fret);
});
```

### データ型の変換

```
sid-note (TypeScript)
  ↓
  String[] → number[]
  ["E2", "G2"] → [0, 3]
  ↓
sid-fret WASM API
  ↓
  Uint8Array → Vec<u8> (Rust内部)
  ↓
sid-fret 内部処理
  - rust-music-theory: Note理論
  - 独自アルゴリズム: 運指計算
  ↓
  FingeringPattern (Rust)
  ↓
sid-fret WASM API
  ↓
  JsValue → Object (TypeScript)
  ↓
sid-note (TypeScript)
  表示
```

## まとめ

### 3層構造
```
Layer 3: sid-note (UI)
         ↓ WASM import
Layer 2: sid-fret (Logic)
         ├─ 独自機能（ベース特化、運指）
         └─ rust-music-theory活用（基本理論）
         ↓ Cargo dependency
Layer 1: rust-music-theory (Foundation)
```

### 責任範囲

| レイヤー | 責任 | 例 |
|---------|------|-----|
| **rust-music-theory** | 基本音楽理論 | Note, Scale, Interval |
| **sid-fret** | ベース特化機能 | 運指、フレット計算、機能和声 |
| **sid-note** | UI/UX | 譜面表示、ユーザー入力 |

### データの流れ

```
ユーザー入力
  ↓
sid-note (前処理)
  ↓
sid-fret WASM API (エントリーポイント)
  ↓
sid-fret内部
  ├─ rust-music-theory（必要に応じて）
  └─ 独自アルゴリズム
  ↓
sid-fret WASM API (結果返却)
  ↓
sid-note (表示)
```

**つまり**：sid-noteは**sid-fretだけ**を呼ぶ。rust-music-theoryはsid-fret内部で使われる。ユーザー（sid-note）からは見えない。
