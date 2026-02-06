# sid-fret WASM API 仕様書

ベースギター特化の音楽理論ライブラリ。sid-noteからWASM importして使用。

## 概要

- **外部依存**: なし（rust-music-theory削除済み、全て自前実装）
- **フレットオフセット基準**: E=0（ベース4弦開放弦）
- **対応キー数**: 48キー（12音 × メジャー/マイナー × ナチュラル/シャープ/フラット + エンハーモニック）
- **WASM export**: 26関数 + 3構造体

---

## WASM Export 関数一覧

### 初期化・情報

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `init()` | `() -> ()` | WASM初期化（自動呼び出し） |
| `version()` | `() -> String` | パッケージバージョン |

### core/note — 音符ユーティリティ

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_line(pitch)` | `(&str) -> Option<f32>` | 五線譜のライン番号（E1=0.0〜G4=23.0） |
| `get_key_position(scale)` | `(&str) -> KeyPosition` | 五度圏での位置（circle: outer/inner, index: 0-11） |
| `compare_pitch(p1, p2)` | `(&str, &str) -> bool` | 異名同音比較（C＃2 == D♭2） |
| `value_text(value)` | `(&str) -> String` | 音符テキスト（"quarter" → "Quarter Note"） |

### chord/parser — コード解析

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_root_note(chord)` | `(&str) -> String` | コード名からルート音を抽出（"C＃maj7" → "C＃"） |
| `get_fret_offset(root)` | `(&str) -> i32` | ルート音のフレットオフセット（E=0基準） |

### chord/positions — ポジション計算

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_chord_positions(chord)` | `(&str) -> JsValue` | コード名から全ポジション配列（Position[]） |
| `get_interval(chord, pitch)` | `(&str, &str) -> String` | コードに対するピッチのインターバル記号 |

### scale/diatonic — スケール・ダイアトニック

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_scale_note_names(scale)` | `(&str) -> Vec<JsValue>` | スケール構成音（7音） |
| `get_scale_diatonic_chords(scale)` | `(&str) -> Vec<JsValue>` | ダイアトニックコード（トライアド、7つ） |
| `get_scale_diatonic_chords_with_7th(scale)` | `(&str) -> Vec<JsValue>` | ダイアトニックコード（7th、7つ） |
| `scale_text(scale)` | `(&str) -> String` | スケール名英語表記（"C" → "C Major Scale"） |

### harmony/functional — 機能和声

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `get_functional_harmony(scale, chord)` | `(&str, &str) -> i32` | 度数番号（I=1〜VII=7、不明=0） |
| `functional_harmony_text(degree)` | `(i32) -> String` | 度数テキスト（"Ⅰ Tonic"） |
| `functional_harmony_info(degree)` | `(i32) -> HarmonyInfo` | 音階度の情報（日本語desc付き） |
| `roman_numeral_harmony_info(degree)` | `(i32) -> HarmonyInfo` | トライアドのローマ数字情報 |
| `roman_numeral_7th_harmony_info(degree)` | `(i32) -> HarmonyInfo` | 7thコードのローマ数字情報 |
| `get_chord_tone_label(scale, chord, pitch)` | `(&str, &str, &str) -> String` | コードトーンラベル（"Tonic Note"等） |

### harmony/cadence — カデンツ

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `cadence_text(prev, current)` | `(i32, i32) -> String` | カデンツ種類判定 |

### utils — ユーティリティ

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `is_chromatic_note(pitch, next)` | `(Option<String>, Option<String>) -> bool` | 半音関係判定 |
| `get_chord_name_aliases(chord)` | `(&str) -> Vec<JsValue>` | コード名の別表記一覧 |

### fingering — 運指アルゴリズム

| 関数 | シグネチャ | 説明 |
|------|-----------|------|
| `calculate_fingering(pitches, mode)` | `(Vec<u8>, &str) -> JsValue` | 運指パターン計算 |

---

## WASM Export 構造体

### Position
```typescript
interface Position {
  string: number;    // 弦番号（1=G, 2=D, 3=A, 4=E）
  fret: number;      // フレット番号（0=開放弦）
  pitch: string;     // ピッチ名（"C2", "G＃3"等）
  interval: string;  // インターバル記号（"1", "♭3", "5"等）
}
```

### KeyPosition
```typescript
interface KeyPosition {
  circle: string;  // "outer"（メジャー）/ "inner"（マイナー）/ "none"
  index: number;   // 五度圏上の位置（0-11）、-1=該当なし
}
```

### HarmonyInfo
```typescript
interface HarmonyInfo {
  roman: string;  // ローマ数字（"Ⅰ", "Ⅱm", "Ⅶm7♭5"等）
  desc: string;   // 日本語説明（"Tonic (主音): 安心・落ち着き"等）
}
```

---

## get_fret_offset 基準値

E=0基準（ベース4弦開放）:

| ルート | オフセット | | ルート | オフセット |
|--------|-----------|---|--------|-----------|
| E / F♭ | 0 | | A＃ / B♭ | 6 |
| E＃ / F | 1 | | B / C♭ | 7 |
| F＃ / G♭ | 2 | | B＃ / C | 8 |
| G | 3 | | C＃ / D♭ | 9 |
| G＃ / A♭ | 4 | | D | 10 |
| A | 5 | | D＃ / E♭ | 11 |

---

## 対応キー一覧（48キー）

C, Cm, C＃, C＃m, C♭, C♭m,
D, Dm, D＃, D＃m, D♭, D♭m,
E, Em, E＃, E＃m, E♭, E♭m,
F, Fm, F＃, F＃m, F♭, F♭m,
G, Gm, G＃, G＃m, G♭, G♭m,
A, Am, A＃, A＃m, A♭, A♭m,
B, Bm, B＃, B＃m, B♭, B♭m

---

## カデンツパターン

| 進行 | 名称 |
|------|------|
| V → I | Perfect Cadence |
| IV → I | Plagal Cadence |
| V → VI | Deceptive Cadence |
| * → V | Half Cadence |
| * → VII | Phrygian Cadence |

---

## 運指モード

| モード名 | 説明 |
|----------|------|
| `shortest` | 最短移動優先 |
| `position-stable` | ポジション固定優先 |
| `string-priority` | 弦移動優先（横移動より縦移動） |
| `open-string` | 開放弦活用 |
| `balanced` | バランス型（最良スコア選択） |

---

## コードエイリアス一覧

| 入力 | 別表記 |
|------|--------|
| `""` (メジャー) | maj, △ |
| `maj7` | M7, △7 |
| `m7` | -7 |
| `m` | - |
| `dim` | o |
| `aug` | + |
| `sus4` | sus |
| `m_maj7` | m(maj7), mM7, -M7 |
| `m6` | -6 |
| `m9` | -9 |
| `M9` | maj9, △9 |
| `m_maj9` | m(maj9), mM9, -M9 |
