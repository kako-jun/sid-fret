use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// スケール種別の半音パターンを返す
pub fn scale_intervals(scale_type: &str) -> Option<Vec<i32>> {
    match scale_type {
        "" | "ionian" => Some(vec![0, 2, 4, 5, 7, 9, 11]),
        "m" | "aeolian" => Some(vec![0, 2, 3, 5, 7, 8, 10]),
        "dorian" => Some(vec![0, 2, 3, 5, 7, 9, 10]),
        "phrygian" => Some(vec![0, 1, 3, 5, 7, 8, 10]),
        "lydian" => Some(vec![0, 2, 4, 6, 7, 9, 11]),
        "mixolydian" => Some(vec![0, 2, 4, 5, 7, 9, 10]),
        "locrian" => Some(vec![0, 1, 3, 5, 6, 8, 10]),
        "penta" => Some(vec![0, 2, 4, 7, 9]),
        "m_penta" => Some(vec![0, 3, 5, 7, 10]),
        "blues" => Some(vec![0, 3, 5, 6, 7, 10]),
        "harm_minor" => Some(vec![0, 2, 3, 5, 7, 8, 11]),
        "melo_minor" => Some(vec![0, 2, 3, 5, 7, 9, 11]),
        _ => None,
    }
}

/// 全音階の音名（C基準、半音→音名）
const NOTE_NAMES: [&str; 12] = [
    "C", "C＃", "D", "D＃", "E", "F", "F＃", "G", "G＃", "A", "A＃", "B",
];
const NOTE_NAMES_FLAT: [&str; 12] = [
    "C", "D♭", "D", "E♭", "E", "F", "G♭", "G", "A♭", "A", "B♭", "B",
];

/// 音名から半音値を取得（C=0）
fn note_to_semitone(note: &str) -> Option<i32> {
    match note {
        "C" | "B＃" => Some(0),
        "C＃" | "D♭" => Some(1),
        "D" => Some(2),
        "D＃" | "E♭" => Some(3),
        "E" | "F♭" => Some(4),
        "F" | "E＃" => Some(5),
        "F＃" | "G♭" => Some(6),
        "G" => Some(7),
        "G＃" | "A♭" => Some(8),
        "A" => Some(9),
        "A＃" | "B♭" => Some(10),
        "B" | "C♭" => Some(11),
        _ => None,
    }
}

/// ルート音がフラット系かを判定
/// C, G, D, A, E, B はシャープ系（ただしフラット付きは除く）
/// F, B♭, E♭, A♭, D♭, G♭, C♭ はフラット系
fn is_flat_key(root: &str) -> bool {
    root.contains('♭') || matches!(root, "F")
}

/// ルート音 + 半音パターンからスケール構成音名を計算
pub fn compute_scale_notes(root: &str, scale_type: &str) -> Vec<String> {
    let intervals = match scale_intervals(scale_type) {
        Some(i) => i,
        None => return vec![],
    };

    let root_semitone = match note_to_semitone(root) {
        Some(s) => s,
        None => return vec![],
    };

    // フラット系を使うかの判定:
    // - ルート音がフラット付きならフラット系
    // - F はフラット系（B♭を含む）
    // - マイナー系スケール（m, aeolian, dorian, phrygian, locrian, m_penta, blues, harm_minor, melo_minor）で
    //   ルートがC, G, D等のシャープ系の場合でも、♭3等が出るためフラット系を使う
    let minor_like = matches!(scale_type,
        "m" | "aeolian" | "dorian" | "phrygian" | "locrian"
        | "m_penta" | "blues" | "harm_minor" | "melo_minor"
    );
    let use_flat = is_flat_key(root) || minor_like;
    let names = if use_flat { &NOTE_NAMES_FLAT } else { &NOTE_NAMES };

    intervals
        .iter()
        .map(|&interval| {
            let semitone = (root_semitone + interval) % 12;
            names[semitone as usize].to_string()
        })
        .collect()
}

/// スケールキーをルート音とスケール種別に分割
/// "C" -> ("C", ""), "C_dorian" -> ("C", "dorian"), "Cm" -> ("C", "m")
pub fn parse_scale_key(scale: &str) -> (String, String) {
    // "_" 区切りの場合
    if let Some(pos) = scale.find('_') {
        let root = &scale[..pos];
        let scale_type = &scale[pos + 1..];
        return (root.to_string(), scale_type.to_string());
    }
    // "m" suffix（ただし1文字の音名 + "m" の場合のみ）
    if let Some(root_part) = scale.strip_suffix('m') {
        // ルート音が有効な音名かチェック
        if note_to_semitone(root_part).is_some() {
            return (root_part.to_string(), "m".to_string());
        }
    }
    // メジャー
    (scale.to_string(), String::new())
}

/// スケールの構成音を取得
#[wasm_bindgen]
pub fn get_scale_note_names(scale: &str) -> Vec<JsValue> {
    // まずハードコードマップを検索（エンハーモニックスペルの正確性のため）
    let scale_map = create_scale_note_map();
    if let Some(notes) = scale_map.get(scale) {
        return notes.iter().map(|s| JsValue::from_str(s)).collect();
    }

    // なければ計算で生成
    let (root, scale_type) = parse_scale_key(scale);
    compute_scale_notes(&root, &scale_type)
        .iter()
        .map(|s| JsValue::from_str(s))
        .collect()
}

/// 内部用: スケール構成音をStringのVecで返す
pub fn get_scale_note_names_internal(scale: &str) -> Vec<String> {
    let scale_map = create_scale_note_map();
    if let Some(notes) = scale_map.get(scale) {
        return notes.iter().map(|s| s.to_string()).collect();
    }
    let (root, scale_type) = parse_scale_key(scale);
    compute_scale_notes(&root, &scale_type)
}

/// スケールごとの構成音マップを作成（メジャー/マイナー 48キー）
pub fn create_scale_note_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // C系
    map.insert("C", vec!["C", "D", "E", "F", "G", "A", "B"]);
    map.insert("Cm", vec!["C", "D", "E♭", "F", "G", "A♭", "B♭"]);
    map.insert("C＃", vec!["C＃", "D＃", "E＃", "F＃", "G＃", "A＃", "B＃"]);
    map.insert("C＃m", vec!["C＃", "D＃", "E", "F＃", "G＃", "A", "B"]);
    map.insert("C♭", vec!["C♭", "D♭", "E♭", "F♭", "G♭", "A♭", "B♭"]);
    map.insert("C♭m", vec!["C♭", "D♭", "E♭♭", "F♭", "G♭", "A♭♭", "B♭♭"]);

    // D系
    map.insert("D", vec!["D", "E", "F＃", "G", "A", "B", "C＃"]);
    map.insert("Dm", vec!["D", "E", "F", "G", "A", "B♭", "C"]);
    map.insert("D＃", vec!["D＃", "E＃", "F＃＃", "G＃", "A＃", "B＃", "C＃＃"]);
    map.insert("D＃m", vec!["D＃", "E＃", "F＃", "G＃", "A＃", "B", "C＃"]);
    map.insert("D♭", vec!["D♭", "E♭", "F", "G♭", "A♭", "B♭", "C"]);
    map.insert("D♭m", vec!["D♭", "E♭", "F♭", "G♭", "A♭", "B♭♭", "C♭"]);

    // E系
    map.insert("E", vec!["E", "F＃", "G＃", "A", "B", "C＃", "D＃"]);
    map.insert("Em", vec!["E", "F＃", "G", "A", "B", "C", "D"]);
    map.insert("E＃", vec!["E＃", "F＃＃", "G＃＃", "A＃", "B＃", "C＃＃", "D＃＃"]);
    map.insert("E＃m", vec!["E＃", "F＃＃", "G＃", "A＃", "B＃", "C＃", "D＃"]);
    map.insert("E♭", vec!["E♭", "F", "G", "A♭", "B♭", "C", "D"]);
    map.insert("E♭m", vec!["E♭", "F", "G♭", "A♭", "B♭", "C♭", "D♭"]);

    // F系
    map.insert("F", vec!["F", "G", "A", "B♭", "C", "D", "E"]);
    map.insert("Fm", vec!["F", "G", "A♭", "B♭", "C", "D♭", "E♭"]);
    map.insert("F＃", vec!["F＃", "G＃", "A＃", "B", "C＃", "D＃", "E＃"]);
    map.insert("F＃m", vec!["F＃", "G＃", "A", "B", "C＃", "D", "E"]);
    map.insert("F♭", vec!["F♭", "G♭", "A♭", "B♭♭", "C♭", "D♭", "E♭"]);
    map.insert("F♭m", vec!["F♭", "G♭", "A♭♭", "B♭♭", "C♭", "D♭♭", "E♭♭"]);

    // G系
    map.insert("G", vec!["G", "A", "B", "C", "D", "E", "F＃"]);
    map.insert("Gm", vec!["G", "A", "B♭", "C", "D", "E♭", "F"]);
    map.insert("G＃", vec!["G＃", "A＃", "B＃", "C＃", "D＃", "E＃", "F＃＃"]);
    map.insert("G＃m", vec!["G＃", "A＃", "B", "C＃", "D＃", "E", "F＃"]);
    map.insert("G♭", vec!["G♭", "A♭", "B♭", "C♭", "D♭", "E♭", "F"]);
    map.insert("G♭m", vec!["G♭", "A♭", "B♭♭", "C♭", "D♭", "E♭♭", "F♭"]);

    // A系
    map.insert("A", vec!["A", "B", "C＃", "D", "E", "F＃", "G＃"]);
    map.insert("Am", vec!["A", "B", "C", "D", "E", "F", "G"]);
    map.insert("A＃", vec!["A＃", "B＃", "C＃＃", "D＃", "E＃", "F＃＃", "G＃＃"]);
    map.insert("A＃m", vec!["A＃", "B＃", "C＃", "D＃", "E＃", "F＃", "G＃"]);
    map.insert("A♭", vec!["A♭", "B♭", "C", "D♭", "E♭", "F", "G"]);
    map.insert("A♭m", vec!["A♭", "B♭", "C♭", "D♭", "E♭", "F♭", "G♭"]);

    // B系
    map.insert("B", vec!["B", "C＃", "D＃", "E", "F＃", "G＃", "A＃"]);
    map.insert("Bm", vec!["B", "C＃", "D", "E", "F＃", "G", "A"]);
    map.insert("B＃", vec!["B＃", "C＃＃", "D＃＃", "E＃", "F＃＃", "G＃＃", "A＃＃"]);
    map.insert("B＃m", vec!["B＃", "C＃＃", "D＃", "E＃", "F＃＃", "G＃", "A＃"]);
    map.insert("B♭", vec!["B♭", "C", "D", "E♭", "F", "G", "A"]);
    map.insert("B♭m", vec!["B♭", "C", "D♭", "E♭", "F", "G♭", "A♭"]);

    map
}

/// ダイアトニックコード（トライアド）を取得
#[wasm_bindgen]
pub fn get_scale_diatonic_chords(scale: &str) -> Vec<JsValue> {
    let chords = get_scale_diatonic_chords_internal(scale);
    chords.iter().map(|s| JsValue::from_str(s)).collect()
}

/// 内部用: ダイアトニックコードをStringのVecで返す
pub fn get_scale_diatonic_chords_internal(scale: &str) -> Vec<String> {
    // まずハードコードマップを検索
    let chord_map = create_diatonic_chord_map();
    if let Some(chords) = chord_map.get(scale) {
        return chords.iter().map(|s| s.to_string()).collect();
    }

    // なければ計算で生成
    let (root, scale_type) = parse_scale_key(scale);
    let notes = compute_scale_notes(&root, &scale_type);
    if notes.is_empty() {
        return vec![];
    }

    let qualities = diatonic_triad_qualities(&scale_type);
    if qualities.is_empty() {
        return vec![]; // ペンタトニック・ブルースにはダイアトニックコードなし
    }

    notes
        .iter()
        .zip(qualities.iter())
        .map(|(note, quality)| format!("{note}{quality}"))
        .collect()
}

/// スケール種別ごとのダイアトニックトライアド品質
fn diatonic_triad_qualities(scale_type: &str) -> Vec<&'static str> {
    match scale_type {
        "" | "ionian" => vec!["", "m", "m", "", "", "m", "dim"],
        "m" | "aeolian" => vec!["m", "dim", "", "m", "m", "", ""],
        "dorian" => vec!["m", "m", "", "", "m", "dim", ""],
        "phrygian" => vec!["m", "", "", "m", "dim", "", "m"],
        "lydian" => vec!["", "", "m", "dim", "", "m", "m"],
        "mixolydian" => vec!["", "m", "dim", "", "m", "m", ""],
        "locrian" => vec!["dim", "", "m", "m", "", "", "m"],
        "harm_minor" => vec!["m", "dim", "aug", "m", "", "", "dim"],
        "melo_minor" => vec!["m", "m", "aug", "", "", "dim", "dim"],
        _ => vec![], // ペンタトニック・ブルースはなし
    }
}

/// スケール種別ごとのダイアトニック7thコード品質
fn diatonic_7th_qualities(scale_type: &str) -> Vec<&'static str> {
    match scale_type {
        "" | "ionian" => vec!["maj7", "m7", "m7", "maj7", "7", "m7", "m7♭5"],
        "m" | "aeolian" => vec!["m7", "m7♭5", "maj7", "m7", "m7", "maj7", "7"],
        "dorian" => vec!["m7", "m7", "maj7", "7", "m7", "m7♭5", "maj7"],
        "phrygian" => vec!["m7", "maj7", "7", "m7", "m7♭5", "maj7", "m7"],
        "lydian" => vec!["maj7", "7", "m7", "m7♭5", "maj7", "m7", "m7"],
        "mixolydian" => vec!["7", "m7", "m7♭5", "maj7", "m7", "m7", "maj7"],
        "locrian" => vec!["m7♭5", "maj7", "m7", "m7", "maj7", "7", "m7"],
        "harm_minor" => vec!["m(maj7)", "m7♭5", "aug(maj7)", "m7", "7", "maj7", "dim7"],
        "melo_minor" => vec!["m(maj7)", "m7", "aug(maj7)", "7", "7", "m7♭5", "m7♭5"],
        _ => vec![],
    }
}

/// ダイアトニックコードマップを作成（メジャー/マイナー 48キー）
pub fn create_diatonic_chord_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // C系
    map.insert("C", vec!["C", "Dm", "Em", "F", "G", "Am", "Bdim"]);
    map.insert("Cm", vec!["Cm", "Ddim", "E♭", "Fm", "Gm", "A♭", "B♭"]);
    map.insert("C＃", vec!["C＃", "D＃m", "Fm", "F＃", "G＃", "A＃m", "Cdim"]);
    map.insert("C＃m", vec!["C＃m", "D＃dim", "E", "F＃m", "G＃m", "A", "B"]);
    map.insert("C♭", vec!["C♭", "D♭m", "E♭m", "F♭", "G♭", "A♭m", "B♭dim"]);
    map.insert("C♭m", vec!["C♭m", "D♭dim", "E♭♭", "F♭m", "G♭m", "A♭♭", "B♭♭"]);

    // D系
    map.insert("D", vec!["D", "Em", "F＃m", "G", "A", "Bm", "C＃dim"]);
    map.insert("Dm", vec!["Dm", "Edim", "F", "Gm", "Am", "B♭", "C"]);
    map.insert("D＃", vec!["D＃", "Fm", "Gm", "G＃", "A＃", "Cm", "Ddim"]);
    map.insert("D＃m", vec!["D＃m", "Fdim", "F＃", "G＃m", "A＃m", "B", "C＃"]);
    map.insert("D♭", vec!["D♭", "E♭m", "Fm", "G♭", "A♭", "B♭m", "Cdim"]);
    map.insert("D♭m", vec!["D♭m", "E♭dim", "F♭", "G♭m", "A♭m", "B♭♭", "C♭"]);

    // E系
    map.insert("E", vec!["E", "F＃m", "G＃m", "A", "B", "C＃m", "D＃dim"]);
    map.insert("Em", vec!["Em", "F＃dim", "G", "Am", "Bm", "C", "D"]);
    map.insert("E＃", vec!["E＃", "F＃＃m", "G＃＃m", "A＃", "B＃", "C＃＃m", "D＃＃dim"]);
    map.insert("E＃m", vec!["E＃m", "F＃＃dim", "G＃", "A＃m", "B＃m", "C＃", "D＃"]);
    map.insert("E♭", vec!["E♭", "Fm", "Gm", "A♭", "B♭", "Cm", "Ddim"]);
    map.insert("E♭m", vec!["E♭m", "Fdim", "G♭", "A♭m", "B♭m", "C♭", "D♭"]);

    // F系
    map.insert("F", vec!["F", "Gm", "Am", "B♭", "C", "Dm", "Edim"]);
    map.insert("Fm", vec!["Fm", "Gdim", "A♭", "B♭m", "Cm", "D♭", "E♭"]);
    map.insert("F＃", vec!["F＃", "G＃m", "A＃m", "B", "C＃", "D＃m", "E＃dim"]);
    map.insert("F＃m", vec!["F＃m", "G＃dim", "A", "Bm", "C＃m", "D", "E"]);
    map.insert("F♭", vec!["F♭", "G♭m", "A♭m", "B♭♭", "C♭", "D♭m", "E♭dim"]);
    map.insert("F♭m", vec!["F♭m", "G♭dim", "A♭♭", "B♭♭m", "C♭m", "D♭♭", "E♭♭"]);

    // G系
    map.insert("G", vec!["G", "Am", "Bm", "C", "D", "Em", "F＃dim"]);
    map.insert("Gm", vec!["Gm", "Adim", "B♭", "Cm", "Dm", "E♭", "F"]);
    map.insert("G＃", vec!["G＃", "A＃m", "Cm", "C＃", "D＃", "Fm", "Gdim"]);
    map.insert("G＃m", vec!["G＃m", "A＃dim", "B", "C＃m", "D＃m", "E", "F＃"]);
    map.insert("G♭", vec!["G♭", "A♭m", "B♭m", "C♭", "D♭", "E♭m", "Fdim"]);
    map.insert("G♭m", vec!["G♭m", "A♭dim", "B♭♭", "C♭m", "D♭m", "E♭♭", "F♭"]);

    // A系
    map.insert("A", vec!["A", "Bm", "C＃m", "D", "E", "F＃m", "G＃dim"]);
    map.insert("Am", vec!["Am", "Bdim", "C", "Dm", "Em", "F", "G"]);
    map.insert("A＃", vec!["A＃", "Cm", "Dm", "D＃", "F", "Gm", "Adim"]);
    map.insert("A＃m", vec!["A＃m", "Cdim", "C＃", "D＃m", "Fm", "F＃", "G＃"]);
    map.insert("A♭", vec!["A♭", "B♭m", "Cm", "D♭", "E♭", "Fm", "Gdim"]);
    map.insert("A♭m", vec!["A♭m", "B♭dim", "C♭", "D♭m", "E♭m", "F♭", "G♭"]);

    // B系
    map.insert("B", vec!["B", "C＃m", "D＃m", "E", "F＃", "G＃m", "A＃dim"]);
    map.insert("Bm", vec!["Bm", "C＃dim", "D", "Em", "F＃m", "G", "A"]);
    map.insert("B＃", vec!["B＃", "C＃＃m", "D＃＃m", "E＃", "F＃＃", "G＃＃m", "A＃＃dim"]);
    map.insert("B＃m", vec!["B＃m", "C＃＃dim", "D＃", "E＃m", "F＃＃m", "G＃", "A＃"]);
    map.insert("B♭", vec!["B♭", "Cm", "Dm", "E♭", "F", "Gm", "Adim"]);
    map.insert("B♭m", vec!["B♭m", "Cdim", "D♭", "E♭m", "Fm", "G♭", "A♭"]);

    map
}

/// ダイアトニックコード（7th）を取得
#[wasm_bindgen]
pub fn get_scale_diatonic_chords_with_7th(scale: &str) -> Vec<JsValue> {
    let chords = get_scale_diatonic_chords_7th_internal(scale);
    chords.iter().map(|s| JsValue::from_str(s)).collect()
}

/// 内部用: ダイアトニック7thコードをStringのVecで返す
pub fn get_scale_diatonic_chords_7th_internal(scale: &str) -> Vec<String> {
    // まずハードコードマップを検索
    let chord_map = create_diatonic_chord_7th_map();
    if let Some(chords) = chord_map.get(scale) {
        return chords.iter().map(|s| s.to_string()).collect();
    }

    // なければ計算で生成
    let (root, scale_type) = parse_scale_key(scale);
    let notes = compute_scale_notes(&root, &scale_type);
    if notes.is_empty() {
        return vec![];
    }

    let qualities = diatonic_7th_qualities(&scale_type);
    if qualities.is_empty() {
        return vec![];
    }

    notes
        .iter()
        .zip(qualities.iter())
        .map(|(note, quality)| format!("{note}{quality}"))
        .collect()
}

/// ダイアトニックコード（7th）マップを作成（メジャー/マイナー 48キー）
pub fn create_diatonic_chord_7th_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // C系
    map.insert("C", vec!["Cmaj7", "Dm7", "Em7", "Fmaj7", "G7", "Am7", "Bm7♭5"]);
    map.insert("Cm", vec!["Cm(maj7)", "Dm7♭5", "E♭maj7", "Fm7", "Gm7", "A♭maj7", "B♭7"]);
    map.insert("C＃", vec!["C＃maj7", "D＃m7", "Fm7", "F＃maj7", "G＃7", "A＃m7", "C7"]);
    map.insert("C＃m", vec!["C＃m(maj7)", "D＃m7♭5", "Emaj7", "F＃m7", "G＃m7", "Amaj7", "B7"]);
    map.insert("C♭", vec!["C♭maj7", "D♭m7", "E♭m7", "F♭maj7", "G♭7", "A♭m7", "B♭m7♭5"]);
    map.insert("C♭m", vec!["C♭m(maj7)", "D♭m7♭5", "E♭♭maj7", "F♭m7", "G♭m7", "A♭♭maj7", "B♭♭7"]);

    // D系
    map.insert("D", vec!["Dmaj7", "Em7", "F＃m7", "Gmaj7", "A7", "Bm7", "C＃m7♭5"]);
    map.insert("Dm", vec!["Dm(maj7)", "Em7♭5", "Fmaj7", "Gm7", "Am7", "B♭maj7", "C7"]);
    map.insert("D＃", vec!["D＃maj7", "Fm7", "Gm7", "G＃maj7", "A＃7", "Cm7", "D7"]);
    map.insert("D＃m", vec!["D＃m(maj7)", "Fm7♭5", "F＃maj7", "G＃m7", "A＃m7", "Bmaj7", "C＃7"]);
    map.insert("D♭", vec!["D♭maj7", "E♭m7", "Fm7", "G♭maj7", "A♭7", "B♭m7", "C7"]);
    map.insert("D♭m", vec!["D♭m(maj7)", "E♭m7♭5", "F♭maj7", "G♭m7", "A♭m7", "B♭♭maj7", "C♭7"]);

    // E系
    map.insert("E", vec!["Emaj7", "F＃m7", "G＃m7", "Amaj7", "B7", "C＃m7", "D＃m7♭5"]);
    map.insert("Em", vec!["Em(maj7)", "F＃m7♭5", "Gmaj7", "Am7", "Bm7", "Cmaj7", "D7"]);
    map.insert("E＃", vec!["E＃maj7", "F＃＃m7", "G＃＃m7", "A＃maj7", "B＃7", "C＃＃m7", "D＃＃m7♭5"]);
    map.insert("E＃m", vec!["E＃m(maj7)", "F＃＃m7♭5", "G＃maj7", "A＃m7", "B＃m7", "C＃maj7", "D＃7"]);
    map.insert("E♭", vec!["E♭maj7", "Fm7", "Gm7", "A♭maj7", "B♭7", "Cm7", "Dm7♭5"]);
    map.insert("E♭m", vec!["E♭m(maj7)", "Fm7♭5", "G♭maj7", "A♭m7", "B♭m7", "C♭maj7", "D♭7"]);

    // F系
    map.insert("F", vec!["Fmaj7", "Gm7", "Am7", "B♭maj7", "C7", "Dm7", "Em7♭5"]);
    map.insert("Fm", vec!["Fm(maj7)", "Gm7♭5", "A♭maj7", "B♭m7", "Cm7", "D♭maj7", "E♭7"]);
    map.insert("F＃", vec!["F＃maj7", "G＃m7", "A＃m7", "Bmaj7", "C＃7", "D＃m7", "E＃m7♭5"]);
    map.insert("F＃m", vec!["F＃m(maj7)", "G＃m7♭5", "Amaj7", "Bm7", "C＃m7", "Dmaj7", "E7"]);
    map.insert("F♭", vec!["F♭maj7", "G♭m7", "A♭m7", "B♭♭maj7", "C♭7", "D♭m7", "E♭m7♭5"]);
    map.insert("F♭m", vec!["F♭m(maj7)", "G♭m7♭5", "A♭♭maj7", "B♭♭m7", "C♭m7", "D♭♭maj7", "E♭♭7"]);

    // G系
    map.insert("G", vec!["Gmaj7", "Am7", "Bm7", "Cmaj7", "D7", "Em7", "F＃m7♭5"]);
    map.insert("Gm", vec!["Gm(maj7)", "Am7♭5", "B♭maj7", "Cm7", "Dm7", "E♭maj7", "F7"]);
    map.insert("G＃", vec!["G＃maj7", "A＃m7", "Cm7", "C＃maj7", "D＃7", "Fm7", "Gm7♭5"]);
    map.insert("G＃m", vec!["G＃m(maj7)", "A＃m7♭5", "Bmaj7", "C＃m7", "D＃m7", "Emaj7", "F＃7"]);
    map.insert("G♭", vec!["G♭maj7", "A♭m7", "B♭m7", "C♭maj7", "D♭7", "E♭m7", "Fm7♭5"]);
    map.insert("G♭m", vec!["G♭m(maj7)", "A♭m7♭5", "B♭♭maj7", "C♭m7", "D♭m7", "E♭♭maj7", "F♭7"]);

    // A系
    map.insert("A", vec!["Amaj7", "Bm7", "C＃m7", "Dmaj7", "E7", "F＃m7", "G＃m7♭5"]);
    map.insert("Am", vec!["Am(maj7)", "Bm7♭5", "Cmaj7", "Dm7", "Em7", "Fmaj7", "G7"]);
    map.insert("A＃", vec!["A＃maj7", "Cm7", "Dm7", "D＃maj7", "F7", "Gm7", "Am7♭5"]);
    map.insert("A＃m", vec!["A＃m(maj7)", "Cm7♭5", "C＃maj7", "D＃m7", "Fm7", "F＃maj7", "G＃7"]);
    map.insert("A♭", vec!["A♭maj7", "B♭m7", "Cm7", "D♭maj7", "E♭7", "Fm7", "Gm7♭5"]);
    map.insert("A♭m", vec!["A♭m(maj7)", "B♭m7♭5", "C♭maj7", "D♭m7", "E♭m7", "F♭maj7", "G♭7"]);

    // B系
    map.insert("B", vec!["Bmaj7", "C＃m7", "D＃m7", "Emaj7", "F＃7", "G＃m7", "A＃m7♭5"]);
    map.insert("Bm", vec!["Bm(maj7)", "C＃m7♭5", "Dmaj7", "Em7", "F＃m7", "Gmaj7", "A7"]);
    map.insert("B＃", vec!["B＃maj7", "C＃＃m7", "D＃＃m7", "E＃maj7", "F＃＃7", "G＃＃m7", "A＃＃m7♭5"]);
    map.insert("B＃m", vec!["B＃m(maj7)", "C＃＃m7♭5", "D＃maj7", "E＃m7", "F＃＃m7", "G＃maj7", "A＃7"]);
    map.insert("B♭", vec!["B♭maj7", "Cm7", "Dm7", "E♭maj7", "F7", "Gm7", "Am7♭5"]);
    map.insert("B♭m", vec!["B♭m(maj7)", "Cm7♭5", "D♭maj7", "E♭m7", "Fm7", "G♭maj7", "A♭7"]);

    map
}

/// スケール名の英語表記を取得
#[wasm_bindgen]
pub fn scale_text(scale: &str) -> String {
    // まず既存の固定マップを検索
    let scale_names: HashMap<&str, &str> = [
        ("C", "C Major"),
        ("Cm", "C Minor"),
        ("C＃", "C＃ Major"),
        ("C＃m", "C＃ Minor"),
        ("C♭", "C♭ Major"),
        ("C♭m", "C♭ Minor"),
        ("D", "D Major"),
        ("Dm", "D Minor"),
        ("D＃", "D＃ Major"),
        ("D＃m", "D＃ Minor"),
        ("D♭", "D♭ Major"),
        ("D♭m", "D♭ Minor"),
        ("E", "E Major"),
        ("Em", "E Minor"),
        ("E＃", "E＃ Major"),
        ("E＃m", "E＃ Minor"),
        ("E♭", "E♭ Major"),
        ("E♭m", "E♭ Minor"),
        ("F", "F Major"),
        ("Fm", "F Minor"),
        ("F＃", "F＃ Major"),
        ("F＃m", "F＃ Minor"),
        ("F♭", "F♭ Major"),
        ("F♭m", "F♭ Minor"),
        ("G", "G Major"),
        ("Gm", "G Minor"),
        ("G＃", "G＃ Major"),
        ("G＃m", "G＃ Minor"),
        ("G♭", "G♭ Major"),
        ("G♭m", "G♭ Minor"),
        ("A", "A Major"),
        ("Am", "A Minor"),
        ("A＃", "A＃ Major"),
        ("A＃m", "A＃ Minor"),
        ("A♭", "A♭ Major"),
        ("A♭m", "A♭ Minor"),
        ("B", "B Major"),
        ("Bm", "B Minor"),
        ("B＃", "B＃ Major"),
        ("B＃m", "B＃ Minor"),
        ("B♭", "B♭ Major"),
        ("B♭m", "B♭ Minor"),
    ]
    .iter()
    .cloned()
    .collect();

    if let Some(name) = scale_names.get(scale) {
        return format!("{name} Scale");
    }

    // 新スケールの動的生成
    let (root, scale_type) = parse_scale_key(scale);
    let type_name = match scale_type.as_str() {
        "" | "ionian" => "Major",
        "m" | "aeolian" => "Minor",
        "dorian" => "Dorian",
        "phrygian" => "Phrygian",
        "lydian" => "Lydian",
        "mixolydian" => "Mixolydian",
        "locrian" => "Locrian",
        "penta" => "Major Pentatonic",
        "m_penta" => "Minor Pentatonic",
        "blues" => "Blues",
        "harm_minor" => "Harmonic Minor",
        "melo_minor" => "Melodic Minor",
        other => other,
    };

    format!("{root} {type_name} Scale")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_scale_note_names() {
        let scale_map = create_scale_note_map();
        let c_major = scale_map.get("C").unwrap();
        assert_eq!(c_major.len(), 7);
        assert_eq!(c_major[0], "C");
        assert_eq!(c_major[6], "B");
    }

    #[test]
    fn test_get_scale_diatonic_chords() {
        let chord_map = create_diatonic_chord_map();
        let c_major = chord_map.get("C").unwrap();
        assert_eq!(c_major.len(), 7);
        assert_eq!(c_major[0], "C");
        assert_eq!(c_major[4], "G");
    }

    #[test]
    fn test_scale_text() {
        assert_eq!(scale_text("C"), "C Major Scale");
        assert_eq!(scale_text("Am"), "A Minor Scale");
    }

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

        let notes = compute_scale_notes("G", "mixolydian");
        assert_eq!(notes, vec!["G", "A", "B", "C", "D", "E", "F"]);
    }

    #[test]
    fn test_parse_scale_key() {
        assert_eq!(parse_scale_key("C"), ("C".to_string(), "".to_string()));
        assert_eq!(parse_scale_key("Am"), ("A".to_string(), "m".to_string()));
        assert_eq!(parse_scale_key("C_dorian"), ("C".to_string(), "dorian".to_string()));
        assert_eq!(parse_scale_key("A_blues"), ("A".to_string(), "blues".to_string()));
    }

    #[test]
    fn test_new_scale_diatonic_chords() {
        let chords = get_scale_diatonic_chords_internal("C_dorian");
        assert_eq!(chords.len(), 7);
        assert_eq!(chords[0], "Cm");  // i
        assert_eq!(chords[3], "F");   // IV
    }

    #[test]
    fn test_new_scale_text() {
        assert_eq!(scale_text("C_dorian"), "C Dorian Scale");
        assert_eq!(scale_text("A_blues"), "A Blues Scale");
        assert_eq!(scale_text("E_penta"), "E Major Pentatonic Scale");
    }

    #[test]
    fn test_pentatonic_no_diatonic_chords() {
        let chords = get_scale_diatonic_chords_internal("C_penta");
        assert!(chords.is_empty());
    }

    #[test]
    fn test_diatonic_7th_for_modes() {
        let chords = get_scale_diatonic_chords_7th_internal("C_dorian");
        assert_eq!(chords.len(), 7);
        assert_eq!(chords[0], "Cm7");    // im7
        assert_eq!(chords[3], "F7");     // IV7
    }
}
