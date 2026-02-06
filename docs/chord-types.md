# コードタイプ拡充仕様

## 概要

既存の `get_frets()` はboolean 6個方式（m3, sus4, dim5, maj7, m7, aug7）でコードを構成していたが、
これを**コードタイプ文字列 → インターバル Vec<Fret> 直接生成**方式に置き換える。

旧 `get_frets(bool×6)` は廃止し、`get_frets(chord_type: &str) -> Vec<Fret>` に変更する。

## 新規関数

### `get_frets(chord_type: &str) -> Vec<Fret>`

コードタイプ文字列からインターバル配列を直接生成する。旧boolean方式を置き換え。

```rust
/// コードタイプ文字列からフレット配列を生成
/// chord_type: "", "m", "7", "m7", "maj7", "dim", "aug", "sus4", "6", "m6",
///             "9", "m9", "maj9", "add9", "sus2", "dim7", "m7b5",
///             "aug7", "7sus4", "m_maj7", "7b9", "7#9"
pub fn get_frets(chord_type: &str) -> Vec<Fret>
```

### `parse_chord_type(chord: &str) -> (&str, &str)`

コード名からルート音とコードタイプを分離する。

```rust
/// "Cm7" -> ("C", "m7"), "F#dim7" -> ("F#", "dim7"), "Bb7sus4" -> ("Bb", "7sus4")
pub fn parse_chord_type(chord: &str) -> (String, String)
```

## コードタイプ一覧

| タイプ文字列 | 構成音（半音数） | インターバル表記 | 例 |
|-------------|----------------|-----------------|-----|
| `""` (major) | 0, 4, 7 | 1, 3, 5 | C |
| `"m"` | 0, 3, 7 | 1, ♭3, 5 | Cm |
| `"7"` | 0, 4, 7, 10 | 1, 3, 5, ♭7 | C7 |
| `"m7"` | 0, 3, 7, 10 | 1, ♭3, 5, ♭7 | Cm7 |
| `"maj7"` | 0, 4, 7, 11 | 1, 3, 5, 7 | Cmaj7 |
| `"m_maj7"` | 0, 3, 7, 11 | 1, ♭3, 5, 7 | Cm(maj7) |
| `"dim"` | 0, 3, 6 | 1, ♭3, ♭5 | Cdim |
| `"dim7"` | 0, 3, 6, 9 | 1, ♭3, ♭5, ♭♭7 | Cdim7 |
| `"m7b5"` | 0, 3, 6, 10 | 1, ♭3, ♭5, ♭7 | Cm7♭5 |
| `"aug"` | 0, 4, 8 | 1, 3, ＃5 | Caug |
| `"aug7"` | 0, 4, 8, 10 | 1, 3, ＃5, ♭7 | Caug7 |
| `"sus4"` | 0, 5, 7 | 1, 4, 5 | Csus4 |
| `"sus2"` | 0, 2, 7 | 1, 2, 5 | Csus2 |
| `"7sus4"` | 0, 5, 7, 10 | 1, 4, 5, ♭7 | C7sus4 |
| `"6"` | 0, 4, 7, 9 | 1, 3, 5, 6 | C6 |
| `"m6"` | 0, 3, 7, 9 | 1, ♭3, 5, 6 | Cm6 |
| `"9"` | 0, 4, 7, 10, 14 | 1, 3, 5, ♭7, 9 | C9 |
| `"m9"` | 0, 3, 7, 10, 14 | 1, ♭3, 5, ♭7, 9 | Cm9 |
| `"maj9"` | 0, 4, 7, 11, 14 | 1, 3, 5, 7, 9 | Cmaj9 |
| `"add9"` | 0, 4, 7, 14 | 1, 3, 5, 9 | Cadd9 |
| `"7b9"` | 0, 4, 7, 10, 13 | 1, 3, 5, ♭7, ♭9 | C7♭9 |
| `"7#9"` | 0, 4, 7, 10, 15 | 1, 3, 5, ♭7, ＃9 | C7＃9 |

## `chord_alias.rs` への追加

```rust
// 新規追加
map.insert("dim7".to_string(), vec!["dim7".to_string(), "o7".to_string(), "°7".to_string()]);
map.insert("m7b5".to_string(), vec!["m7♭5".to_string(), "m7b5".to_string(), "ø".to_string(), "ø7".to_string()]);
map.insert("aug7".to_string(), vec!["aug7".to_string(), "+7".to_string()]);
map.insert("7sus4".to_string(), vec!["7sus4".to_string(), "7sus".to_string()]);
map.insert("7b9".to_string(), vec!["7♭9".to_string(), "7b9".to_string()]);
map.insert("7#9".to_string(), vec!["7＃9".to_string(), "7#9".to_string()]);
```

## 破壊的変更

- `get_frets(bool×6)` → 廃止。`get_frets(chord_type: &str) -> Vec<Fret>` に置き換え。
- `get_chord_positions_internal()` — `parse_chord_type()` + `get_frets()` を使うようリファクタ。

## テスト追加

```rust
#[test]
fn test_get_frets() {
    // 基本トライアド
    let major = get_frets("");
    assert_eq!(major.len(), 3);
    assert_eq!(major[0].fret, 0); // root
    assert_eq!(major[1].fret, 4); // 3rd
    assert_eq!(major[2].fret, 7); // 5th

    // dim7（4音）
    let dim7 = get_frets("dim7");
    assert_eq!(dim7.len(), 4);
    assert_eq!(dim7[3].fret, 9); // ♭♭7

    // 9th（5音）
    let ninth = get_frets("9");
    assert_eq!(ninth.len(), 5);
    assert_eq!(ninth[4].fret, 14); // 9

    // aug triad（3音）
    let aug = get_frets("aug");
    assert_eq!(aug.len(), 3);
    assert_eq!(aug[2].fret, 8); // ＃5
}

#[test]
fn test_parse_chord_type() {
    assert_eq!(parse_chord_type("Cm7"), ("C".to_string(), "m7".to_string()));
    assert_eq!(parse_chord_type("F＃dim7"), ("F＃".to_string(), "dim7".to_string()));
    assert_eq!(parse_chord_type("B♭7sus4"), ("B♭".to_string(), "7sus4".to_string()));
    assert_eq!(parse_chord_type("C"), ("C".to_string(), "".to_string()));
}
```
