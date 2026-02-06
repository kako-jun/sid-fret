# sid-fret WASM API 仕様書

ベースギター特化の音楽理論ライブラリ。sid-noteからWASM importして使用。

## 概要

- **外部依存**: なし（全て自前実装）
- **フレットオフセット基準**: E=0（ベース4弦開放弦）
- **対応キー数**: 48キー（メジャー/マイナー） + モード/ペンタトニック/ブルース等
- **対応コードタイプ**: 22種類
- **対応チューニング**: 4種類プリセット（4弦/5弦/6弦/ドロップD）

---

## WASM Export 関数一覧

### 初期化・情報

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `init()` | `() -> ()` | WASM初期化（自動呼び出し） |
| `version()` | `() -> String` | パッケージバージョン |

### core/pitch — ピッチ基盤

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `fret_offset(root)` | `(&str) -> i32` | フレットオフセット（E=0基準） |
| `compare_pitch(p1, p2)` | `(&str, &str) -> bool` | 異名同音比較 |

### core/chord_type — コード解析

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_root_note(chord)` | `(&str) -> String` | コード名からルート音を抽出 |

#### 内部関数（WASM非公開）

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_chord_tones(chord_type)` | `(&str) -> Vec<ChordTone>` | コードタイプ→構成音配列 |
| `parse_chord_type(chord)` | `(&str) -> (String, String)` | コード名→(ルート, タイプ)分離 |

### core/interval — インターバル計算

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `semitone_distance(p1, p2)` | `(&str, &str) -> i32` | 2音間の半音距離 |
| `interval_name(semitones)` | `(i32) -> String` | 半音数→インターバル名（P1, M3, P5等） |
| `detect_inversion(chord, bass)` | `(&str, &str) -> i32` | 転回形判定（0-3, -1=非構成音） |

### core/scale_type — スケール定義

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_scale_note_names(scale)` | `(&str) -> Vec<JsValue>` | スケール構成音 |

### harmony/diatonic — ダイアトニックコード

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_scale_diatonic_chords(scale)` | `(&str) -> Vec<JsValue>` | ダイアトニックコード（トライアド） |
| `get_scale_diatonic_chords_with_7th(scale)` | `(&str) -> Vec<JsValue>` | ダイアトニックコード（7th） |

### harmony/functional — 機能和声

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_functional_harmony(scale, chord)` | `(&str, &str) -> i32` | 度数番号 |
| `functional_harmony_text(degree)` | `(i32) -> String` | 度数テキスト |
| `functional_harmony_info(degree)` | `(i32) -> HarmonyInfo` | 音階度の情報 |
| `roman_numeral_harmony_info(degree)` | `(i32) -> HarmonyInfo` | トライアドのローマ数字 |
| `roman_numeral_7th_harmony_info(degree)` | `(i32) -> HarmonyInfo` | 7thのローマ数字 |
| `get_chord_tone_label(scale, chord, pitch)` | `(&str, &str, &str) -> String` | コードトーンラベル |
| `analyze_progression(scale, chords)` | `(&str, Vec<JsValue>) -> JsValue` | 進行分析 |

### harmony/cadence — カデンツ

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `cadence_text(prev, current)` | `(i32, i32) -> String` | カデンツ判定（7パターン） |
| `cadence_text_extended(prev2, prev, current)` | `(i32, i32, i32) -> String` | 3コードカデンツ |
| `functional_area(degree)` | `(i32) -> String` | T/S/D機能分類 |

### instrument/fretboard — フレットボード計算

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_chord_positions(chord)` | `(&str) -> JsValue` | コードの全ポジション（4弦デフォルト） |
| `get_chord_positions_with_tuning(chord, tuning)` | `(&str, &str) -> JsValue` | チューニング指定付きポジション |
| `get_interval(chord, pitch)` | `(&str, &str) -> String` | インターバル記号 |
| `get_tuning_info(tuning_name)` | `(&str) -> JsValue` | チューニング情報 |
| `list_tunings()` | `() -> JsValue` | プリセット一覧 |

### instrument/fingering — 運指アルゴリズム

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `calculate_fingering(pitches, mode)` | `(Vec<u8>, &str) -> JsValue` | 運指パターン計算 |

### utils/chromatic — 半音関係

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `is_chromatic_note(pitch, next)` | `(Option<String>, Option<String>) -> bool` | 半音関係判定 |

### utils/chord_alias — コード別表記

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_chord_name_aliases(chord)` | `(&str) -> Vec<JsValue>` | コード名の別表記一覧 |

### utils/notation — 表記ユーティリティ

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_line(pitch)` | `(&str) -> Option<f32>` | 五線譜のライン番号（E1=0.0基準、計算生成） |
| `get_key_position(scale)` | `(&str) -> KeyPosition` | 五度圏での位置 |
| `value_text(value)` | `(&str) -> String` | 音符テキスト |
| `scale_text(scale)` | `(&str) -> String` | スケール名英語表記 |

---

## 構造体

### Position
```typescript
interface Position {
  string: number;    // 弦番号（1=最高音弦）
  fret: number;      // フレット番号（0=開放弦）
  pitch: string;     // ピッチ名（"C2", "G＃3"等）
  interval: string;  // インターバル記号（"1", "♭3", "5"等）
}
```

### KeyPosition
```typescript
interface KeyPosition {
  circle: string;  // "outer" / "inner" / "none"
  index: number;   // 五度圏位置（0-11, -1=該当なし）
}
```

### HarmonyInfo
```typescript
interface HarmonyInfo {
  roman: string;  // ローマ数字
  desc: string;   // 日本語説明
}
```

### ProgressionInfo
```typescript
interface ProgressionInfo {
  degree: number;              // スケール度数（1-7, 0=non-diatonic）
  roman: string;               // ローマ数字
  function: string;            // "T" / "S" / "D"
  cadence: string;             // カデンツ名（該当時）
  is_secondary_dominant: bool; // セカンダリードミナント
  secondary_target: string;    // "V/ii" 等
}
```

### Tuning
```typescript
interface Tuning {
  name: string;
  strings: StringDef[];  // 低音弦→高音弦順
  max_fret: number;
}
interface StringDef {
  open_note: string;
  offset: number;  // E=0基準
}
```

---

## 対応コードタイプ（22種類）

| タイプ | 構成音（半音） | 別表記 |
|--------|-------------|--------|
| `""` (major) | 0,4,7 | maj, △ |
| `"m"` | 0,3,7 | - |
| `"7"` | 0,4,7,10 | |
| `"m7"` | 0,3,7,10 | -7 |
| `"maj7"` | 0,4,7,11 | M7, △7 |
| `"m_maj7"` | 0,3,7,11 | m(maj7), mM7, -M7 |
| `"dim"` | 0,3,6 | o |
| `"dim7"` | 0,3,6,9 | o7, °7 |
| `"m7b5"` | 0,3,6,10 | m7♭5, ø, ø7 |
| `"aug"` | 0,4,8 | + |
| `"aug7"` | 0,4,8,10 | +7 |
| `"sus4"` | 0,5,7 | sus |
| `"sus2"` | 0,2,7 | |
| `"7sus4"` | 0,5,7,10 | 7sus |
| `"6"` | 0,4,7,9 | |
| `"m6"` | 0,3,7,9 | -6 |
| `"9"` | 0,4,7,10,14 | |
| `"m9"` | 0,3,7,10,14 | -9 |
| `"maj9"` | 0,4,7,11,14 | M9, △9 |
| `"add9"` | 0,4,7,14 | |
| `"7b9"` | 0,4,7,10,13 | 7♭9 |
| `"7#9"` | 0,4,7,10,15 | 7＃9 |

---

## 対応スケール

### メジャー/マイナー（48キー、計算生成）
C, Cm, C＃, C＃m, C♭, C♭m, D, Dm, D＃, D＃m, D♭, D♭m,
E, Em, E＃, E＃m, E♭, E♭m, F, Fm, F＃, F＃m, F♭, F♭m,
G, Gm, G＃, G＃m, G♭, G♭m, A, Am, A＃, A＃m, A♭, A♭m,
B, Bm, B＃, B＃m, B♭, B♭m

### モード
`{Root}_dorian`, `{Root}_phrygian`, `{Root}_lydian`, `{Root}_mixolydian`, `{Root}_locrian`

### ペンタトニック・ブルース
`{Root}_penta`, `{Root}_m_penta`, `{Root}_blues`

### ハーモニック/メロディック
`{Root}_harm_minor`, `{Root}_melo_minor`

---

## カデンツパターン（7種）

| 進行 | 名称 |
|------|------|
| V → I | Perfect Cadence |
| IV → I | Plagal Cadence |
| VII → I | Leading-tone Cadence |
| V → VI | Deceptive Cadence |
| V → IV | Interrupted Cadence |
| * → V | Half Cadence |
| * → VII | Phrygian Cadence |

### 3コード拡張
| 進行 | 名称 |
|------|------|
| II → V → I | ii-V-I Cadence |

---

## チューニングプリセット

| 名前 | 弦構成 | オフセット |
|------|--------|-----------|
| `bass_4` | E-A-D-G | 0,5,10,15 |
| `bass_5` | B-E-A-D-G | -5,0,5,10,15 |
| `bass_6` | B-E-A-D-G-C | -5,0,5,10,15,20 |
| `bass_drop_d` | D-A-D-G | -2,5,10,15 |

---

## 運指モード

| モード名 | 説明 |
|----------|------|
| `shortest` | 最短移動優先 |
| `position-stable` | ポジション固定優先 |
| `string-priority` | 弦移動優先 |
| `open-string` | 開放弦活用 |
| `balanced` | バランス型 |
