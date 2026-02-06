//! スケール定義（楽器非依存）

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::core::pitch::{is_flat_key, note_to_semitone, CHROMATIC_FLAT, CHROMATIC_SHARP};

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

    let minor_like = matches!(
        scale_type,
        "m" | "aeolian"
            | "dorian"
            | "phrygian"
            | "locrian"
            | "m_penta"
            | "blues"
            | "harm_minor"
            | "melo_minor"
    );
    let use_flat = is_flat_key(root) || minor_like;
    let names = if use_flat {
        &CHROMATIC_FLAT
    } else {
        &CHROMATIC_SHARP
    };

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
    if let Some(pos) = scale.find('_') {
        let root = &scale[..pos];
        let scale_type = &scale[pos + 1..];
        return (root.to_string(), scale_type.to_string());
    }
    if let Some(root_part) = scale.strip_suffix('m') {
        if note_to_semitone(root_part).is_some() {
            return (root_part.to_string(), "m".to_string());
        }
    }
    (scale.to_string(), String::new())
}

/// スケールの構成音を取得（WASM）
#[wasm_bindgen]
pub fn get_scale_note_names(scale: &str) -> Vec<JsValue> {
    let scale_map = create_scale_note_map();
    if let Some(notes) = scale_map.get(scale) {
        return notes.iter().map(|s| JsValue::from_str(s)).collect();
    }
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

/// スケールごとの構成音マップ（メジャー/マイナー 48キー）
pub fn create_scale_note_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    map.insert("C", vec!["C", "D", "E", "F", "G", "A", "B"]);
    map.insert("Cm", vec!["C", "D", "E♭", "F", "G", "A♭", "B♭"]);
    map.insert("C＃", vec!["C＃", "D＃", "E＃", "F＃", "G＃", "A＃", "B＃"]);
    map.insert("C＃m", vec!["C＃", "D＃", "E", "F＃", "G＃", "A", "B"]);
    map.insert("C♭", vec!["C♭", "D♭", "E♭", "F♭", "G♭", "A♭", "B♭"]);
    map.insert("C♭m", vec!["C♭", "D♭", "E♭♭", "F♭", "G♭", "A♭♭", "B♭♭"]);

    map.insert("D", vec!["D", "E", "F＃", "G", "A", "B", "C＃"]);
    map.insert("Dm", vec!["D", "E", "F", "G", "A", "B♭", "C"]);
    map.insert("D＃", vec!["D＃", "E＃", "F＃＃", "G＃", "A＃", "B＃", "C＃＃"]);
    map.insert("D＃m", vec!["D＃", "E＃", "F＃", "G＃", "A＃", "B", "C＃"]);
    map.insert("D♭", vec!["D♭", "E♭", "F", "G♭", "A♭", "B♭", "C"]);
    map.insert("D♭m", vec!["D♭", "E♭", "F♭", "G♭", "A♭", "B♭♭", "C♭"]);

    map.insert("E", vec!["E", "F＃", "G＃", "A", "B", "C＃", "D＃"]);
    map.insert("Em", vec!["E", "F＃", "G", "A", "B", "C", "D"]);
    map.insert("E＃", vec!["E＃", "F＃＃", "G＃＃", "A＃", "B＃", "C＃＃", "D＃＃"]);
    map.insert("E＃m", vec!["E＃", "F＃＃", "G＃", "A＃", "B＃", "C＃", "D＃"]);
    map.insert("E♭", vec!["E♭", "F", "G", "A♭", "B♭", "C", "D"]);
    map.insert("E♭m", vec!["E♭", "F", "G♭", "A♭", "B♭", "C♭", "D♭"]);

    map.insert("F", vec!["F", "G", "A", "B♭", "C", "D", "E"]);
    map.insert("Fm", vec!["F", "G", "A♭", "B♭", "C", "D♭", "E♭"]);
    map.insert("F＃", vec!["F＃", "G＃", "A＃", "B", "C＃", "D＃", "E＃"]);
    map.insert("F＃m", vec!["F＃", "G＃", "A", "B", "C＃", "D", "E"]);
    map.insert("F♭", vec!["F♭", "G♭", "A♭", "B♭♭", "C♭", "D♭", "E♭"]);
    map.insert("F♭m", vec!["F♭", "G♭", "A♭♭", "B♭♭", "C♭", "D♭♭", "E♭♭"]);

    map.insert("G", vec!["G", "A", "B", "C", "D", "E", "F＃"]);
    map.insert("Gm", vec!["G", "A", "B♭", "C", "D", "E♭", "F"]);
    map.insert("G＃", vec!["G＃", "A＃", "B＃", "C＃", "D＃", "E＃", "F＃＃"]);
    map.insert("G＃m", vec!["G＃", "A＃", "B", "C＃", "D＃", "E", "F＃"]);
    map.insert("G♭", vec!["G♭", "A♭", "B♭", "C♭", "D♭", "E♭", "F"]);
    map.insert("G♭m", vec!["G♭", "A♭", "B♭♭", "C♭", "D♭", "E♭♭", "F♭"]);

    map.insert("A", vec!["A", "B", "C＃", "D", "E", "F＃", "G＃"]);
    map.insert("Am", vec!["A", "B", "C", "D", "E", "F", "G"]);
    map.insert("A＃", vec!["A＃", "B＃", "C＃＃", "D＃", "E＃", "F＃＃", "G＃＃"]);
    map.insert("A＃m", vec!["A＃", "B＃", "C＃", "D＃", "E＃", "F＃", "G＃"]);
    map.insert("A♭", vec!["A♭", "B♭", "C", "D♭", "E♭", "F", "G"]);
    map.insert("A♭m", vec!["A♭", "B♭", "C♭", "D♭", "E♭", "F♭", "G♭"]);

    map.insert("B", vec!["B", "C＃", "D＃", "E", "F＃", "G＃", "A＃"]);
    map.insert("Bm", vec!["B", "C＃", "D", "E", "F＃", "G", "A"]);
    map.insert("B＃", vec!["B＃", "C＃＃", "D＃＃", "E＃", "F＃＃", "G＃＃", "A＃＃"]);
    map.insert("B＃m", vec!["B＃", "C＃＃", "D＃", "E＃", "F＃＃", "G＃", "A＃"]);
    map.insert("B♭", vec!["B♭", "C", "D", "E♭", "F", "G", "A"]);
    map.insert("B♭m", vec!["B♭", "C", "D♭", "E♭", "F", "G♭", "A♭"]);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_note_map() {
        let map = create_scale_note_map();
        let c_major = map.get("C").unwrap();
        assert_eq!(c_major.len(), 7);
        assert_eq!(c_major[0], "C");
        assert_eq!(c_major[6], "B");
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
        assert_eq!(
            parse_scale_key("C_dorian"),
            ("C".to_string(), "dorian".to_string())
        );
        assert_eq!(
            parse_scale_key("A_blues"),
            ("A".to_string(), "blues".to_string())
        );
    }
}
