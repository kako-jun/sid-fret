# スケール拡充仕様

## 概要

既存のメジャー/マイナー（48キー）に加え、モード・ペンタトニック・ブルース・
ハーモニックマイナー・メロディックマイナーを追加する。

## キー命名規則

`"{ルート}_{スケール種別}"` 形式。既存の `"C"`, `"Cm"` はそのまま維持。

| スケール種別 | サフィックス | 例 |
|-------------|------------|-----|
| Major (Ionian) | なし / `_ionian` | `"C"` / `"C_ionian"` |
| Natural Minor (Aeolian) | `m` / `_aeolian` | `"Cm"` / `"C_aeolian"` |
| Dorian | `_dorian` | `"C_dorian"` |
| Phrygian | `_phrygian` | `"C_phrygian"` |
| Lydian | `_lydian` | `"C_lydian"` |
| Mixolydian | `_mixolydian` | `"C_mixolydian"` |
| Locrian | `_locrian` | `"C_locrian"` |
| Major Pentatonic | `_penta` | `"C_penta"` |
| Minor Pentatonic | `_m_penta` | `"C_m_penta"` |
| Blues | `_blues` | `"C_blues"` |
| Harmonic Minor | `_harm_minor` | `"C_harm_minor"` |
| Melodic Minor | `_melo_minor` | `"C_melo_minor"` |

## スケール構成音（半音パターン）

| スケール | 音程パターン（半音数） | 音数 |
|---------|---------------------|------|
| Major (Ionian) | 0, 2, 4, 5, 7, 9, 11 | 7 |
| Dorian | 0, 2, 3, 5, 7, 9, 10 | 7 |
| Phrygian | 0, 1, 3, 5, 7, 8, 10 | 7 |
| Lydian | 0, 2, 4, 6, 7, 9, 11 | 7 |
| Mixolydian | 0, 2, 4, 5, 7, 9, 10 | 7 |
| Aeolian (Natural Minor) | 0, 2, 3, 5, 7, 8, 10 | 7 |
| Locrian | 0, 1, 3, 5, 6, 8, 10 | 7 |
| Major Pentatonic | 0, 2, 4, 7, 9 | 5 |
| Minor Pentatonic | 0, 3, 5, 7, 10 | 5 |
| Blues | 0, 3, 5, 6, 7, 10 | 6 |
| Harmonic Minor | 0, 2, 3, 5, 7, 8, 11 | 7 |
| Melodic Minor | 0, 2, 3, 5, 7, 9, 11 | 7 |

## 実装方式

**ダイアトニックスペリングアルゴリズム**により全スケールを計算で生成する。
ハードコードマップは廃止済み。

### 関数

```rust
/// スケール種別の半音パターンを返す
pub fn scale_intervals(scale_type: &str) -> Option<Vec<i32>>

/// ルート音 + スケール種別からスケール構成音名を計算
/// 7音スケールはダイアトニックスペリング（各度に固有文字名）を使用
/// 例: compute_scale_notes("C", "dorian") -> ["C", "D", "E♭", "F", "G", "A", "B♭"]
/// 例: compute_scale_notes("F＃", "") -> ["F＃", "G＃", "A＃", "B", "C＃", "D＃", "E＃"]
pub fn compute_scale_notes(root: &str, scale_type: &str) -> Vec<String>
```

### ダイアトニックスペリングの仕組み

1. ルート音の文字（C/D/E/F/G/A/B）から連続7文字を割り当て
2. 各度の期待半音値と自然音の半音値の差分でシャープ/フラットを付与
3. ダブルシャープ（＃＃）、ダブルフラット（♭♭）にも対応

### `scale_text` 追加分

```
"C_dorian"      -> "C Dorian Scale"
"C_phrygian"    -> "C Phrygian Scale"
"C_lydian"      -> "C Lydian Scale"
"C_mixolydian"  -> "C Mixolydian Scale"
"C_aeolian"     -> "C Aeolian Scale"
"C_locrian"     -> "C Locrian Scale"
"C_penta"       -> "C Major Pentatonic Scale"
"C_m_penta"     -> "C Minor Pentatonic Scale"
"C_blues"       -> "C Blues Scale"
"C_harm_minor"  -> "C Harmonic Minor Scale"
"C_melo_minor"  -> "C Melodic Minor Scale"
```

## ダイアトニックコード

7音スケールにはダイアトニックコードを定義する。
ペンタトニック・ブルースには定義しない（`get_scale_diatonic_chords` は空Vecを返す）。

### モード別ダイアトニックコード品質（ローマ数字）

| スケール | I | II | III | IV | V | VI | VII |
|---------|---|-----|-----|-----|---|-----|------|
| Ionian | M | m | m | M | M | m | dim |
| Dorian | m | m | M | M | m | dim | M |
| Phrygian | m | M | M | m | dim | M | m |
| Lydian | M | M | m | dim | M | m | m |
| Mixolydian | M | m | dim | M | m | m | M |
| Aeolian | m | dim | M | m | m | M | M |
| Locrian | dim | M | m | m | M | M | m |
| Harmonic Minor | m | dim | aug | m | M | M | dim |
| Melodic Minor | m | m | aug | M | M | dim | dim |

### ダイアトニック7thコード品質

| スケール | I | II | III | IV | V | VI | VII |
|---------|---|-----|-----|-----|---|-----|------|
| Ionian | maj7 | m7 | m7 | maj7 | 7 | m7 | m7♭5 |
| Dorian | m7 | m7 | maj7 | 7 | m7 | m7♭5 | maj7 |
| Phrygian | m7 | maj7 | 7 | m7 | m7♭5 | maj7 | m7 |
| Lydian | maj7 | 7 | m7 | m7♭5 | maj7 | m7 | m7 |
| Mixolydian | 7 | m7 | m7♭5 | maj7 | m7 | m7 | maj7 |
| Aeolian | m7 | m7♭5 | maj7 | m7 | m7 | maj7 | 7 |
| Locrian | m7♭5 | maj7 | m7 | m7 | maj7 | 7 | m7 |
| Harmonic Minor | m(maj7) | m7♭5 | aug(maj7) | m7 | 7 | maj7 | dim7 |
| Melodic Minor | m(maj7) | m7 | aug(maj7) | 7 | 7 | m7♭5 | m7♭5 |

## 実装方針

- 全スケール（メジャー/マイナー48キー含む）をダイアトニックスペリングアルゴリズムで計算
- ハードコードマップ（`create_scale_note_map()`）は廃止済み
- 既存キー名（"C", "Am" 等）は引き続き使用可能（同じ結果）
- 新スケールは `"C_dorian"` 等の新キー名でアクセス

## テスト追加

```rust
#[test]
fn test_scale_intervals() {
    assert_eq!(scale_intervals("dorian"), Some(vec![0, 2, 3, 5, 7, 9, 10]));
    assert_eq!(scale_intervals("blues"), Some(vec![0, 3, 5, 6, 7, 10]));
    assert_eq!(scale_intervals("unknown"), None);
}

#[test]
fn test_compute_scale_notes() {
    let notes = compute_scale_notes("C", "dorian");
    assert_eq!(notes, vec!["C", "D", "E♭", "F", "G", "A", "B♭"]);

    let notes = compute_scale_notes("A", "m_penta");
    assert_eq!(notes, vec!["A", "C", "D", "E", "G"]);
}

#[test]
fn test_new_scale_diatonic_chords() {
    let chords = get_scale_diatonic_chords("C_dorian");
    assert_eq!(chords[0], "Cm");  // i
    assert_eq!(chords[3], "F");   // IV
}

#[test]
fn test_existing_scales_unchanged() {
    // 既存の結果が変わらないことを確認
    let notes = get_scale_note_names("C");
    assert_eq!(notes.len(), 7);
}
```
