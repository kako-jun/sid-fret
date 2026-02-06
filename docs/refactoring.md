# リファクタリング計画

## 設計原則

「音楽理論」と「楽器固有」の分離。core/ は楽器に依存しない音楽理論。instrument/ はベースギター固有。

## 理想構造

```
src/
├── lib.rs                    # WASM init, version
├── core/                     # 楽器非依存の音楽理論
│   ├── pitch.rs              # 音名・ピッチの統一基盤
│   ├── interval.rs           # インターバル計算
│   ├── chord_type.rs         # コード構成音定義（楽器非依存）
│   └── scale_type.rs         # スケール定義（楽器非依存）
├── harmony/                  # 和声理論（楽器非依存）
│   ├── diatonic.rs           # ダイアトニックコード生成
│   ├── functional.rs         # 機能和声分析
│   └── cadence.rs            # カデンツ判定
├── instrument/               # ベースギター固有
│   ├── tuning.rs             # チューニング定義
│   ├── fretboard.rs          # フレットボード計算
│   └── fingering/            # 運指アルゴリズム（変更なし）
│       ├── position.rs
│       ├── scoring.rs
│       └── algorithm.rs
└── utils/                    # ユーティリティ
    ├── chromatic.rs           # → core/pitch を使用
    ├── chord_alias.rs         # → core/chord_type を使用
    └── notation.rs            # value_text（音価表記）
```

## 解消する問題

### 1. note_to_semitone 重複の一元化

**現状**: `core/interval.rs:6-22` と `scale/diatonic.rs:32-48` で完全重複

**対応**: `core/pitch.rs` に統一し、両方から参照

### 2. pitch解析の統一

**現状**: 3つの異なる実装
- `core/interval.rs:parse_pitch()` — 末尾から数字を逆抽出
- `core/note.rs:get_pitch_index()` — compare_pitch内のローカル関数
- `utils/chromatic.rs:get_absolute_pitch_index()` — 先頭から音名+符号を順抽出

**対応**: `core/pitch.rs::parse_pitch()` に統一、`absolute_semitone()` も統一

### 3. 12音配列の集約

**現状**: 4箇所に分散
- `core/note.rs:144` — `["C", "C＃/D♭", ...]`
- `utils/chromatic.rs:59` — `["C", "C＃/D♭", ...]`（同一）
- `scale/diatonic.rs:24-26` — `NOTE_NAMES` (sharp系)
- `scale/diatonic.rs:27-29` — `NOTE_NAMES_FLAT` (flat系)

**対応**: `core/pitch.rs` に `CHROMATIC_SHARP`, `CHROMATIC_FLAT`, `CHROMATIC_BOTH` を定義

### 4. get_chord_positions_internal 重複解消

**現状**: `positions.rs:317-418` と `positions.rs:210-300` がほぼ同一の100行

**対応**: `get_chord_positions_internal()` を削除し、`get_chord_positions_with_tuning_internal(chord, &Tuning::bass_4())` に統一

### 5. Fret → ChordTone リネーム

**現状**: `Fret { interval: String, fret: i32 }` — 「フレット」は楽器用語だがデータは音楽理論

**対応**: `ChordTone { interval: String, semitones: i32 }` に改名、`core/chord_type.rs` へ移動

### 6. get_pitch_map 計算化

**現状**: `parser.rs:148-175` に12×12のハードコードテーブル

**対応**: `CHROMATIC_BOTH` を root の位置でローテーションして生成

### 7. get_fret_offset 計算化

**現状**: `parser.rs:31-56` に21パターンのmatch

**対応**: `note_to_semitone(root) - note_to_semitone("E")` で計算可能（E=0基準）。
負値は `(result + 12) % 12` で正規化。

### 8. chord/parser.rs の分解

**現状**: 楽器非依存（`get_root_note`, `parse_chord_type`, `get_frets`）と楽器固有（`get_fret_offset`, `get_pitch_map`）が混在

**対応**:
- `get_root_note`, `parse_chord_type`, `get_frets` → `core/chord_type.rs`
- `get_fret_offset`, `get_pitch_map` → `instrument/fretboard.rs`
- `chord/parser.rs` は削除（re-exportで互換維持）

### 9. scale/diatonic.rs の分割

**現状**: 587行に3つの責務が混在
- スケール定義（intervals, compute, parse）
- ダイアトニックコード生成（qualities, maps）
- 48キー×3マップ

**対応**:
- スケール定義 → `core/scale_type.rs`
- ダイアトニック生成 → `harmony/diatonic.rs`（48キーマップもここに残す）

### 10. Tuning の独立モジュール化

**現状**: `chord/positions.rs` にTuning定義(130行)とフレットボード計算(300行)が同居

**対応**: `instrument/tuning.rs` に分離

### 11. get_line 計算化

**現状**: `core/note.rs:6-88` の85行のmatch文

**対応**: 五線譜のダイアトニック位置は計算可能:
- 自然音（C=0, D=1, E=2, F=3, G=4, A=5, B=6）にオクターブ×7を加算
- E1を基準(0.0)としてオフセット
- シャープは+0.5、フラットは-0.5

### 12. value_text の移動

**現状**: `core/note.rs` に音価テキスト変換がピッチ処理と同居

**対応**: `utils/notation.rs` へ移動（音価はピッチと無関係）

### 13. chord_alias.rs のルート抽出重複

**現状**: `chord_alias.rs:13-38` で `get_root_note` と同じルート抽出を再実装

**対応**: `core/chord_type.rs::get_root_note` を呼ぶように変更

## 実装順序

全ステップで `cargo test` 通過、`cargo clippy` clean を維持。

| Step | 内容 | 対象の問題 |
|------|------|-----------|
| 1 | `core/pitch.rs` 作成 — 統一基盤 | #1, #2, #3 |
| 2 | `core/chord_type.rs` 作成 — Fret→ChordTone | #5, #8 |
| 3 | `core/scale_type.rs` 作成 — スケール定義抽出 | #9 |
| 4 | `instrument/tuning.rs` 作成 — Tuning独立化 | #10 |
| 5 | `instrument/fretboard.rs` 作成 — 楽器固有ロジック集約 | #4, #6, #7, #8 |
| 6 | `harmony/diatonic.rs` 移動 — scale→harmony | #9 |
| 7 | `core/note.rs` 整理 — get_line計算化、value_text移動 | #11, #12 |
| 8 | `utils/` 整理 — core依存に変更 | #13 |
| 9 | 旧ファイル削除・mod.rs整理 | — |

## API方針

後方互換性は維持しない。同じ機能が使えればよい。
理想的な関数名・シグネチャに自由に変更する。

## 実装状況

**全9ステップ完了** (2026-02-06)。13問題すべて解消済み。
56テスト通過、clippy clean、release build OK。
