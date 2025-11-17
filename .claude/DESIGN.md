# sid-fret 設計ドキュメント

## アーキテクチャ概要

### レイヤー構成

```
┌─────────────────────────────────────┐
│   WASM API Layer                    │
│   (calculate_fingering, etc.)       │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│   Application Layer                 │
│   - fingering/  運指アルゴリズム     │
│   - harmony/    機能和声分析         │
│   - chord/      ベースフレット計算   │
│   - utils/      ユーティリティ       │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│   Foundation Layer                  │
│   - core/       基本音楽理論         │
│   - scale/      スケール理論         │
│   (rust-music-theoryに委譲)          │
└─────────────────────────────────────┘
```

## モジュール詳細設計

### 1. fingering/ - 運指アルゴリズム

#### position.rs
**FretPosition**: 単一フレットポジション
```rust
pub struct FretPosition {
    pub string: u8,      // 1-4 (G, D, A, E)
    pub fret: u8,        // 0-24
    pub finger: Option<u8>, // 1-4 (人差し指-小指)
}
```

**FingeringPattern**: 運指パターン
```rust
pub struct FingeringPattern {
    pub positions: Vec<FretPosition>,
    pub score: f32,
    pub algorithm: String,
}
```

メトリクス:
- `total_movement()`: 総移動距離
- `position_changes()`: ポジション変更回数
- `open_string_count()`: 開放弦使用回数
- `string_changes()`: 弦移動回数

#### scoring.rs
**AlgorithmWeights**: スコアリング重み
```rust
pub struct AlgorithmWeights {
    pub movement_weight: f32,           // 移動距離
    pub position_change_weight: f32,    // ポジション変更
    pub open_string_weight: f32,        // 開放弦使用
    pub string_change_weight: f32,      // 弦移動
}
```

プリセット:
- `shortest()`: 移動距離重視
- `position_stable()`: ポジション固定重視
- `open_string()`: 開放弦重視
- `string_priority()`: 弦移動重視
- `balanced()`: バランス型

#### algorithm.rs
**FingeringMode**: アルゴリズムモード
```rust
pub enum FingeringMode {
    Shortest,
    PositionStable,
    StringPriority,
    OpenString,
    Balanced,
}
```

アルゴリズム実装:
- `calculate_shortest_path()`: 貪欲法で最短経路
- `calculate_position_stable()`: ポジション距離最小化
- `calculate_string_priority()`: 弦移動コスト削減
- `calculate_open_string()`: 開放弦優先
- `calculate_balanced()`: 複数試行して最適選択

### 2. chord/ - ベースフレット計算

#### fret.rs
**BassString**: ベース弦定義
```rust
pub struct BassString {
    pub name: &'static str,  // "E", "A", "D", "G"
    pub min_fret: i32,       // 最小フレット
    pub max_fret: i32,       // 最大フレット
}
```

4弦ベース標準チューニング:
```rust
const BASS_STRINGS: [BassString; 4] = [
    BassString { name: "G", min_fret: 15, max_fret: 39 }, // 1弦
    BassString { name: "D", min_fret: 10, max_fret: 34 }, // 2弦
    BassString { name: "A", min_fret: 5, max_fret: 29 },  // 3弦
    BassString { name: "E", min_fret: 0, max_fret: 24 },  // 4弦
];
```

主要関数:
- `get_fret_offset(root: &str) -> i32`: ルート音→半音オフセット
- `get_frets(...)`: コード構成音→フレット配列
- `convert_frets_to_positions()`: フレット→ベース4弦ポジション
- `get_pitches()`: フレット→音程名

### 3. harmony/ - 機能和声分析

#### functional.rs
**HarmonyInfo**: 和声情報
```rust
pub struct HarmonyInfo {
    roman: String,    // "Ⅰ", "Ⅴ", etc.
    function: String, // "Tonic", "Dominant", etc.
}
```

ダイアトニックコードマップ:
```rust
// 24キー対応（12メジャー + 12マイナー）
"C"  => ["C", "Dm", "Em", "F", "G", "Am", "Bdim"]
"Cm" => ["Cm", "Ddim", "Eb", "Fm", "Gm", "Ab", "Bb"]
```

機能:
- `get_functional_harmony()`: I-VII度数判定
- `functional_harmony_text()`: "Ⅴ Dominant"形式
- `roman_numeral_harmony_info()`: ローマ数字+機能名
- `get_chord_tone_label()`: コードトーン役割判定

#### cadence.rs
カデンツパターン:
```rust
V → I   => "Perfect Cadence"    (完全終止)
IV → I  => "Plagal Cadence"     (アーメン終止)
V → VI  => "Deceptive Cadence"  (偽終止)
→ V     => "Half Cadence"       (半終止)
→ VII   => "Phrygian Cadence"   (フリギア終止)
```

### 4. utils/ - ユーティリティ

#### chromatic.rs
- `is_chromatic_note()`: 半音階判定
- `get_absolute_pitch_index()`: 絶対音高インデックス（C0=0）

#### chord_alias.rs
- `get_chord_name_aliases()`: コード名エイリアス取得
- 日本語記号対応: `＃`, `♯`, `♭`

エイリアスマップ:
```rust
"maj7" => ["maj7", "M7", "△7"]
"m"    => ["m", "min", "-"]
"m7"   => ["m7", "min7", "-7"]
```

## データフロー

### 運指計算フロー
```
音程配列 (pitches: Vec<u8>)
    ↓
generate_all_positions() - 各音程の可能ポジション生成
    ↓
アルゴリズム選択 (FingeringMode)
    ↓
calculate_*() - 各アルゴリズムで運指計算
    ↓
AlgorithmWeights.calculate_score() - スコアリング
    ↓
FingeringPattern (positions, score, algorithm)
```

### フレット計算フロー
```
コード名 (chord: &str)
    ↓
get_root_note() - ルート音抽出
    ↓
get_fret_offset() - 半音オフセット計算
    ↓
get_frets() - コード構成音→フレット配列
    ↓
convert_frets_to_positions() - 4弦ベースマッピング
    ↓
Vec<Vec<i32>> (各構成音のポジション配列)
```

## パフォーマンス最適化

### 1. メモリ効率
- 不要な文字列アロケーション削減
- `&'static str`の活用（pitch_map等）
- Vecの事前容量確保

### 2. 計算最適化
- modulo演算での正規化
- 早期フィルタリング（範囲外ポジション排除）
- HashMap使用（O(1)ルックアップ）

### 3. WASM最適化
```toml
[profile.release]
opt-level = "z"     # サイズ最適化
lto = true          # Link Time Optimization
```

## テスト戦略

### 単体テスト（29テスト）
- ベースフレット計算: 6テスト
- 運指アルゴリズム: 12テスト
- 機能和声分析: 4テスト
- ユーティリティ: 7テスト

### テストカバレッジ目標
- コア機能: 100%
- エッジケース: 90%以上
- エラーハンドリング: 100%

### 継続的インテグレーション
```bash
cargo test          # 全テスト実行
cargo clippy        # Lint検査
cargo fmt -- --check # フォーマット検査
```

## WASM API設計

### エクスポート関数
```rust
#[wasm_bindgen]
pub fn calculate_fingering(pitches: Vec<u8>, mode: &str) -> JsValue;

#[wasm_bindgen]
pub fn get_fret_offset(root: &str) -> i32;

#[wasm_bindgen]
pub fn get_functional_harmony(scale: &str, chord: &str) -> i32;

#[wasm_bindgen]
pub fn cadence_text(prev: i32, curr: i32) -> String;
```

### JavaScript使用例
```javascript
import init, { calculate_fingering } from './pkg/sid_fret.js';

await init();
const pattern = calculate_fingering([0, 3, 5, 7], "shortest");
console.log(pattern.positions);
console.log(pattern.score);
```

## エラーハンドリング

### 戦略
- 無効な入力に対してデフォルト値返却
- `unwrap_or()`, `unwrap_or_default()`活用
- WASM境界での安全性確保

### 例
```rust
// 無効なルート音 → 0を返却
get_fret_offset("X") // => 0

// 無効なスケール → 空配列
get_functional_harmony("invalid", "C") // => 0
```

## 拡張性

### プラグインアーキテクチャ（将来）
```rust
pub trait FingeringAlgorithm {
    fn calculate(&self, pitches: &[u8]) -> FingeringPattern;
}

// ユーザー定義アルゴリズム
impl FingeringAlgorithm for CustomAlgorithm { ... }
```

### 設定可能性
```rust
pub struct BassConfig {
    pub strings: Vec<BassString>,
    pub tuning: Tuning,
    pub fret_count: u8,
}
```
