# ピッチ基盤仕様

## 概要

全モジュールの土台となる音名・ピッチの統一基盤。
12音の半音値変換、ピッチ文字列の解析、異名同音比較を提供する。

モジュール: `core/pitch.rs`

## 12音配列

3種類の12音配列を定義。C=0 のインデックスで参照する。

| インデックス | CHROMATIC_SHARP | CHROMATIC_FLAT | CHROMATIC_BOTH |
|-------------|-----------------|----------------|----------------|
| 0 | C | C | C |
| 1 | C＃ | D♭ | C＃/D♭ |
| 2 | D | D | D |
| 3 | D＃ | E♭ | D＃/E♭ |
| 4 | E | E | E |
| 5 | F | F | F |
| 6 | F＃ | G♭ | F＃/G♭ |
| 7 | G | G | G |
| 8 | G＃ | A♭ | G＃/A♭ |
| 9 | A | A | A |
| 10 | A＃ | B♭ | A＃/B♭ |
| 11 | B | B | B |

## 関数

### `note_to_semitone(note) -> Option<i32>`

音名から半音値を取得（C=0基準）。異名同音を含む全音名に対応。

| 音名 | 半音値 | 異名同音 |
|------|--------|---------|
| C | 0 | B＃ |
| C＃ | 1 | D♭ |
| D | 2 | |
| D＃ | 3 | E♭ |
| E | 4 | F♭ |
| F | 5 | E＃ |
| F＃ | 6 | G♭ |
| G | 7 | |
| G＃ | 8 | A♭ |
| A | 9 | |
| A＃ | 10 | B♭ |
| B | 11 | C♭ |

### `parse_pitch(pitch) -> Option<(String, i32)>`

ピッチ文字列を音名とオクターブに分割。末尾の数字をオクターブとして抽出する。

```
"C3"  -> ("C", 3)
"E♭1" -> ("E♭", 1)
"F＃2" -> ("F＃", 2)
"C"   -> None（オクターブなし）
```

### `absolute_semitone(pitch) -> Option<i32>`

ピッチの絶対半音値を計算（C0 = 0）。

計算式: `octave * 12 + note_to_semitone(note)`

```
"C0" -> 0
"C1" -> 12
"E1" -> 16
"A4" -> 57
```

### `strip_octave(pitch) -> String`

ピッチ文字列からオクターブ数字を除去して音名のみ返す。

```
"C3"  -> "C"
"E♭1" -> "E♭"
"G＃"  -> "G＃"（オクターブなしでもそのまま返す）
```

### `pitch_map_for_root(root) -> Vec<String>`

ルート音に基づく12音マップを生成。`CHROMATIC_BOTH` をルート位置でローテーション。

```
pitch_map_for_root("C") -> ["C", "C＃/D♭", "D", ..., "B"]
pitch_map_for_root("E") -> ["E", "F", "F＃/G♭", ..., "D＃/E♭"]
```

### `fret_offset(root) -> i32`（WASM公開）

E=0基準のフレットオフセットを計算。ベースの4弦開放弦（E）からの半音距離。

計算式: `(note_to_semitone(root) - 4 + 12) % 12`

| ルート | オフセット |
|--------|----------|
| E | 0 |
| F | 1 |
| G | 3 |
| A | 5 |
| B | 7 |
| C | 8 |
| D | 10 |

### `compare_pitch(p1, p2) -> bool`（WASM公開）

ピッチの異名同音比較。オクターブと半音値の両方が一致すれば `true`。

```
compare_pitch("C＃2", "D♭2") -> true
compare_pitch("C2", "C3")    -> false
```
