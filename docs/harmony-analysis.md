# 和声進行分析仕様

## 概要

既存のカデンツ判定を7パターン + 3コード拡張（ii-V-I）に拡充し、
複数コードの進行分析機能を新規追加する。

## カデンツパターン（7パターン + ii-V-I）

### `cadence_text` 対応パターン

| # | 進行 | 名称 | 説明 |
|---|------|------|------|
| 1 | V → I | Perfect Cadence | 完全終止 |
| 2 | IV → I | Plagal Cadence | アーメン終止 |
| 3 | VII → I | Leading-tone Cadence | 導音終止 |
| 4 | V → VI | Deceptive Cadence | 偽終止 |
| 5 | V → IV | Interrupted Cadence | 中断終止 |
| 6 | * → V | Half Cadence | 半終止 |
| 7 | * → VII | Phrygian Cadence | フリギア終止 |

### `cadence_text_extended` 対応パターン

| # | 進行 | 名称 | 説明 |
|---|------|------|------|
| 8 | II → V → I | ii-V-I Cadence | ツーファイブワン |

### 将来追加候補

以下のパターンは現在未実装。度数ベースの2引数では判定困難なため、別途検討が必要。

| 進行 | 名称 | 説明 |
|------|------|------|
| ♭VII → I | Backdoor Cadence | バックドア終止 |
| iv → I | Minor Plagal Cadence | マイナープラーガル終止 |
| ♭II → I | Neapolitan Cadence | ナポリタン終止 |
| ♭II → I (tritone sub) | Tritone Sub Cadence | トライトーンサブ終止 |

### `cadence_text` の変更

既存シグネチャを維持しつつ追加パターンを判定:

```rust
#[wasm_bindgen]
pub fn cadence_text(prev_functional_harmony: i32, functional_harmony: i32) -> String
```

新パターン 6-10 は既存の2引数で判定可能。
パターン 11 (ii-V-I) は3コード進行のため別関数で対応。

### `cadence_text_extended` — 新規

```rust
/// 3コード進行のカデンツ判定（ii-V-I対応）
#[wasm_bindgen]
pub fn cadence_text_extended(
    prev2_harmony: i32,
    prev_harmony: i32,
    current_harmony: i32,
) -> String
```

## 進行分析（新規）

### `functional_area(degree: i32) -> String`

スケール度数からT/S/D機能分類を返す。

```rust
/// 1,3,6 -> "T" (Tonic)
/// 2,4   -> "S" (Subdominant)
/// 5,7   -> "D" (Dominant)
#[wasm_bindgen]
pub fn functional_area(degree: i32) -> String
```

| 度数 | 機能領域 | 説明 |
|------|---------|------|
| I (1) | T | Tonic |
| II (2) | S | Subdominant |
| III (3) | T | Tonic |
| IV (4) | S | Subdominant |
| V (5) | D | Dominant |
| VI (6) | T | Tonic |
| VII (7) | D | Dominant |

### `ProgressionInfo` 構造体

```rust
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressionInfo {
    degree: i32,           // スケール度数 (1-7, 0=non-diatonic)
    roman: String,         // ローマ数字表記
    function: String,      // T/S/D
    cadence: String,       // カデンツ名（該当する場合）
    is_secondary_dominant: bool,  // セカンダリードミナント
    secondary_target: String,     // セカンダリードミナントの解決先（"V/ii"等）
}
```

### `analyze_progression(scale: &str, chords: JsValue) -> JsValue`

```rust
/// 複数コードの進行を分析
/// scale: "C", "Am" 等
/// chords: ["C", "Am", "F", "G"] のJS配列
/// 戻り値: Vec<ProgressionInfo> のJsValue
#[wasm_bindgen]
pub fn analyze_progression(scale: &str, chords: JsValue) -> JsValue
```

### セカンダリードミナント検出

非ダイアトニックコードがダイアトニックコードのV7に相当するかを判定。

例: キーCで `A7` → V7/ii (Dmの5度上のドミナント7th)

検出ロジック:
1. コードがダイアトニックでない場合
2. コードがドミナント7thの構造（Root + M3 + P5 + m7）を持つ場合
3. ルートの完全5度下がダイアトニックコードのルートに一致する場合
4. → セカンダリードミナントとして `V/X` を返す

### トライトーンサブスティテューション検出（未実装・将来候補）

ドミナント7thコードが本来のV7の増4度（トライトーン）関係にある場合を検出。

例: キーCで `D♭7` → Tritone Sub of V (G7のトライトーン代理)

検出ロジック:
1. コードがドミナント7th構造を持つ
2. ルートがV度の増4度（6半音）の関係にある
3. → `Tritone Sub of V` を返す

## テスト追加

```rust
#[test]
fn test_extended_cadences() {
    assert_eq!(cadence_text(5, 4), "Interrupted Cadence");
    assert_eq!(cadence_text(7, 1), "Leading-tone Cadence");
}

#[test]
fn test_cadence_text_extended_ii_v_i() {
    assert_eq!(
        cadence_text_extended(2, 5, 1),
        "ii-V-I Cadence"
    );
}

#[test]
fn test_functional_area() {
    assert_eq!(functional_area(1), "T");
    assert_eq!(functional_area(4), "S");
    assert_eq!(functional_area(5), "D");
}

#[test]
fn test_analyze_progression_basic() {
    // キーCで C -> Am -> F -> G の進行分析
    // I(T) -> vi(T) -> IV(S) -> V(D)
}

#[test]
fn test_secondary_dominant() {
    // キーCで A7 はセカンダリードミナント V/ii
}
```
