# 5弦/6弦・チューニング対応仕様

## 概要

既存の4弦ベース標準チューニング（E-A-D-G）のみ対応から、
5弦/6弦ベース・ドロップチューニングに対応する。

既存APIは完全に維持し、`_with_tuning()` サフィックスの新関数を追加する。

## 新規構造体

### `StringDef`

```rust
/// 弦の定義
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StringDef {
    pub open_note: String,  // 開放弦の音名（例: "E", "B"）
    pub offset: i32,        // E1=0基準の半音オフセット
}
```

### `Tuning`

```rust
/// チューニング定義
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tuning {
    pub name: String,
    pub strings: Vec<StringDef>,  // 低音弦から高音弦の順
    pub max_fret: i32,            // 最大フレット数（デフォルト24）
}
```

## プリセットチューニング

### `Tuning::bass_4()` — 4弦スタンダード

| 弦 | 音名 | offset (E=0) |
|----|------|-------------|
| 4弦 | E | 0 |
| 3弦 | A | 5 |
| 2弦 | D | 10 |
| 1弦 | G | 15 |

max_fret: 24

### `Tuning::bass_5()` — 5弦スタンダード

| 弦 | 音名 | offset (E=0) |
|----|------|-------------|
| 5弦 | B | -5 |
| 4弦 | E | 0 |
| 3弦 | A | 5 |
| 2弦 | D | 10 |
| 1弦 | G | 15 |

max_fret: 24

### `Tuning::bass_6()` — 6弦スタンダード

| 弦 | 音名 | offset (E=0) |
|----|------|-------------|
| 6弦 | B | -5 |
| 5弦 | E | 0 |
| 4弦 | A | 5 |
| 3弦 | D | 10 |
| 2弦 | G | 15 |
| 1弦 | C | 20 |

max_fret: 24

### `Tuning::bass_drop_d()` — ドロップD

| 弦 | 音名 | offset (E=0) |
|----|------|-------------|
| 4弦 | D | -2 |
| 3弦 | A | 5 |
| 2弦 | D | 10 |
| 1弦 | G | 15 |

max_fret: 24

## 新規WASM API

### `get_chord_positions_with_tuning(chord: &str, tuning_name: &str) -> JsValue`

```rust
/// チューニング指定付きコードポジション取得
/// tuning_name: "bass_4", "bass_5", "bass_6", "bass_drop_d"
#[wasm_bindgen]
pub fn get_chord_positions_with_tuning(chord: &str, tuning_name: &str) -> JsValue
```

### `get_tuning_info(tuning_name: &str) -> JsValue`

```rust
/// チューニング情報を返す（弦数、各弦の音名等）
#[wasm_bindgen]
pub fn get_tuning_info(tuning_name: &str) -> JsValue
```

### `list_tunings() -> JsValue`

```rust
/// 利用可能なチューニングプリセット一覧を返す
#[wasm_bindgen]
pub fn list_tunings() -> JsValue
```

## 内部変更

### `convert_frets_to_positions()` の拡張

現在の4弦ハードコード:
```rust
// 現状: string 1-4、固定オフセット
(1, 15..=39),  // G弦
(2, 10..=34),  // D弦
(3,  5..=29),  // A弦
(4,  0..=24),  // E弦
```

Tuningベースに変更:
```rust
fn convert_frets_to_positions_with_tuning(
    frets: &[FretWithPitch],
    tuning: &Tuning,
) -> Vec<Position>
```

弦番号は1=最高音弦（既存互換）。各弦の範囲は `offset..=(offset + max_fret)` で計算。

### Fingeringモジュールへの影響

- `FretPosition.string` — 弦番号の最大値がチューニング依存に
- `generate_all_positions()` — Tuningを引数に取る新バージョン追加
- スコアリングは弦数に関わらず同じ計算式を使用

## 変更方針

- `get_chord_positions(chord)` — 内部で `Tuning::bass_4()` を使用するようリファクタ。
- `get_chord_positions_with_tuning()` で他のチューニングに対応。
- Fingeringモジュールも `Tuning` ベースにリファクタ。

## テスト追加

```rust
#[test]
fn test_tuning_presets() {
    let bass4 = Tuning::bass_4();
    assert_eq!(bass4.strings.len(), 4);
    assert_eq!(bass4.strings[0].offset, 0);  // E弦

    let bass5 = Tuning::bass_5();
    assert_eq!(bass5.strings.len(), 5);
    assert_eq!(bass5.strings[0].offset, -5); // B弦
}

#[test]
fn test_chord_positions_with_tuning() {
    // 4弦で既存と同じ結果
    let pos_default = get_chord_positions("C");
    let pos_4string = get_chord_positions_with_tuning("C", "bass_4");
    // 両者の結果が一致することを確認

    // 5弦では追加ポジションが存在
    let pos_5string = get_chord_positions_with_tuning("C", "bass_5");
    // 5弦のポジション数 >= 4弦のポジション数
}

#[test]
fn test_drop_d_tuning() {
    let drop_d = Tuning::bass_drop_d();
    assert_eq!(drop_d.strings[0].offset, -2); // D弦（E-2半音）
    assert_eq!(drop_d.strings[0].open_note, "D");
}
```
