# インターバル計算仕様

## 概要

2音間の距離計算、コンパウンドインターバル名称、転回形判定を追加する。
`core/interval.rs` として新規モジュールを作成する。

## 新規関数

### `semitone_distance(pitch1: &str, pitch2: &str) -> i32`

2つのピッチ間の半音数を計算する。ピッチはオクターブ番号付き（例: "E1", "C3"）。

```rust
/// 2つのピッチ間の半音距離を計算
/// pitch1: "E1", pitch2: "A1" -> 5
/// pitch1: "C3", pitch2: "E1" -> -20 (下方向は負)
#[wasm_bindgen]
pub fn semitone_distance(pitch1: &str, pitch2: &str) -> i32
```

#### ピッチ→半音数変換ルール

音名からの半音値（C=0基準）:
| 音名 | 半音値 |
|------|--------|
| C / B＃ | 0 |
| C＃ / D♭ | 1 |
| D | 2 |
| D＃ / E♭ | 3 |
| E / F♭ | 4 |
| F / E＃ | 5 |
| F＃ / G♭ | 6 |
| G | 7 |
| G＃ / A♭ | 8 |
| A | 9 |
| A＃ / B♭ | 10 |
| B / C♭ | 11 |

絶対半音値 = (オクターブ × 12) + 音名半音値

`semitone_distance = absolute(pitch2) - absolute(pitch1)`

### `interval_name(semitones: i32) -> String`

半音数からインターバル名称を返す。コンパウンドインターバル（9th以上）にも対応。

```rust
/// 半音数からインターバル名称を返す
/// 0 -> "P1" (Perfect Unison)
/// 7 -> "P5" (Perfect Fifth)
/// 14 -> "M9" (Major Ninth)
#[wasm_bindgen]
pub fn interval_name(semitones: i32) -> String
```

#### インターバル名一覧

| 半音数 | 略称 | 正式名称 |
|--------|------|---------|
| 0 | P1 | Perfect Unison |
| 1 | m2 | Minor Second |
| 2 | M2 | Major Second |
| 3 | m3 | Minor Third |
| 4 | M3 | Major Third |
| 5 | P4 | Perfect Fourth |
| 6 | TT | Tritone |
| 7 | P5 | Perfect Fifth |
| 8 | m6 | Minor Sixth |
| 9 | M6 | Major Sixth |
| 10 | m7 | Minor Seventh |
| 11 | M7 | Major Seventh |
| 12 | P8 | Perfect Octave |
| 13 | m9 | Minor Ninth |
| 14 | M9 | Major Ninth |
| 15 | m10 | Minor Tenth |
| 16 | M10 | Major Tenth |
| 17 | P11 | Perfect Eleventh |
| 18 | A11 | Augmented Eleventh |
| 19 | P12 | Perfect Twelfth |
| 20 | m13 | Minor Thirteenth |
| 21 | M13 | Major Thirteenth |

半音数 > 21 の場合: `"{semitones}st"` (半音数のみ返す)
半音数 < 0 の場合: 絶対値で計算して `-` prefix を付与

### `detect_inversion(chord: &str, bass_pitch: &str) -> i32`

コードとバス音から転回形を判定する。

```rust
/// コードの転回形を判定
/// chord: "C", bass_pitch: "C1" -> 0 (基本形)
/// chord: "C", bass_pitch: "E1" -> 1 (第1転回形)
/// chord: "C", bass_pitch: "G1" -> 2 (第2転回形)
/// chord: "Cmaj7", bass_pitch: "B2" -> 3 (第3転回形)
/// 構成音にない場合 -> -1
#[wasm_bindgen]
pub fn detect_inversion(chord: &str, bass_pitch: &str) -> i32
```

#### 転回形判定ロジック

1. `parse_chord_type()` でコードのルート音とタイプを取得
2. `get_chord_tones()` でインターバル配列を取得
3. バス音の音名（オクターブ除去）を取得
4. ルート音からの各構成音の音名を `pitch_map_for_root()` で逆引き
5. バス音がN番目の構成音に一致する場合、N を返す（0=基本形）
6. 一致しない場合 -1 を返す

## モジュール構成

```
src/core/
├── pitch.rs        # 音名・ピッチ基盤
├── chord_type.rs   # コード解析
├── scale_type.rs   # スケール定義
├── interval.rs     # インターバル計算
└── mod.rs
```

## 互換性

完全に新規追加のため、既存APIへの影響なし。

## テスト

```rust
#[test]
fn test_semitone_distance() {
    assert_eq!(semitone_distance("E1", "A1"), 5);
    assert_eq!(semitone_distance("C3", "C4"), 12);
    assert_eq!(semitone_distance("G2", "E2"), -3);
    assert_eq!(semitone_distance("B0", "C1"), 1);
}

#[test]
fn test_interval_name() {
    assert_eq!(interval_name(0), "P1");
    assert_eq!(interval_name(7), "P5");
    assert_eq!(interval_name(12), "P8");
    assert_eq!(interval_name(14), "M9");
    assert_eq!(interval_name(-7), "-P5");
}

#[test]
fn test_detect_inversion() {
    assert_eq!(detect_inversion("C", "C1"), 0);
    assert_eq!(detect_inversion("C", "E1"), 1);
    assert_eq!(detect_inversion("C", "G1"), 2);
    assert_eq!(detect_inversion("Cmaj7", "B2"), 3);
    assert_eq!(detect_inversion("C", "F1"), -1);
}
```
