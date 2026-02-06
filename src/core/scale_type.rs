//! スケール定義（楽器非依存）

use wasm_bindgen::prelude::*;

use crate::core::pitch::note_to_semitone;

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

/// 自然音名の半音値（C=0基準）
const NATURAL_SEMITONES: [i32; 7] = [0, 2, 4, 5, 7, 9, 11]; // C, D, E, F, G, A, B
/// 自然音名の文字
const NATURAL_LETTERS: [char; 7] = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];

/// ルート音の文字部分のインデックスを取得（C=0, D=1, ..., B=6）
fn letter_index(root: &str) -> Option<usize> {
    let base = root.chars().next()?;
    NATURAL_LETTERS.iter().position(|&c| c == base)
}

/// 半音差からアクシデンタル文字列を生成
fn accidental_str(diff: i32) -> &'static str {
    match diff {
        -2 => "♭♭",
        -1 => "♭",
        0 => "",
        1 => "＃",
        2 => "＃＃",
        _ => "?",
    }
}

/// 7音スケールの各度に対応するダイアトニック文字位置
/// メジャースケールは 0,1,2,3,4,5,6（各文字1つずつ）
fn diatonic_degree_positions(scale_type: &str) -> Option<Vec<usize>> {
    match scale_type {
        "" | "ionian" | "m" | "aeolian" | "dorian" | "phrygian"
        | "lydian" | "mixolydian" | "locrian" | "harm_minor" | "melo_minor" => {
            Some(vec![0, 1, 2, 3, 4, 5, 6])
        }
        // ペンタトニック: メジャー=1,2,3,5,6度 → 文字位置0,1,2,4,5
        "penta" => Some(vec![0, 1, 2, 4, 5]),
        // マイナーペンタトニック: 1,♭3,4,5,♭7度 → 文字位置0,2,3,4,6
        "m_penta" => Some(vec![0, 2, 3, 4, 6]),
        // ブルース: 1,♭3,4,♭5,5,♭7度 → ♭5は#4として文字位置3を使用
        "blues" => Some(vec![0, 2, 3, 3, 4, 6]),
        _ => None,
    }
}

/// ルート音 + 半音パターンからスケール構成音名を計算
/// 7音スケールはダイアトニックスペリング（各度に固有文字名）を使用
pub fn compute_scale_notes(root: &str, scale_type: &str) -> Vec<String> {
    let intervals = match scale_intervals(scale_type) {
        Some(i) => i,
        None => return vec![],
    };

    let root_semitone = match note_to_semitone(root) {
        Some(s) => s,
        None => return vec![],
    };

    let degree_positions = match diatonic_degree_positions(scale_type) {
        Some(pos) => pos,
        None => return vec![],
    };

    let root_letter_idx = match letter_index(root) {
        Some(i) => i,
        None => return vec![],
    };

    intervals
        .iter()
        .zip(degree_positions.iter())
        .map(|(&interval, &deg_pos)| {
            let target_semitone = (root_semitone + interval) % 12;
            let letter_idx = (root_letter_idx + deg_pos) % 7;
            let natural_semitone = NATURAL_SEMITONES[letter_idx];
            let diff = (target_semitone - natural_semitone + 18) % 12 - 6;
            let letter = NATURAL_LETTERS[letter_idx];
            let acc = accidental_str(diff);
            format!("{letter}{acc}")
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
    get_scale_note_names_internal(scale)
        .iter()
        .map(|s| JsValue::from_str(s))
        .collect()
}

/// 内部用: スケール構成音をStringのVecで返す
pub(crate) fn get_scale_note_names_internal(scale: &str) -> Vec<String> {
    let (root, scale_type) = parse_scale_key(scale);
    compute_scale_notes(&root, &scale_type)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_note_names() {
        let c_major = get_scale_note_names_internal("C");
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
    fn test_all_48_keys_scale_notes() {
        // 代表的なキーの検証
        assert_eq!(compute_scale_notes("C", ""), vec!["C", "D", "E", "F", "G", "A", "B"]);
        assert_eq!(compute_scale_notes("C", "m"), vec!["C", "D", "E♭", "F", "G", "A♭", "B♭"]);
        assert_eq!(compute_scale_notes("C＃", ""), vec!["C＃", "D＃", "E＃", "F＃", "G＃", "A＃", "B＃"]);
        assert_eq!(compute_scale_notes("F＃", "m"), vec!["F＃", "G＃", "A", "B", "C＃", "D", "E"]);
        assert_eq!(compute_scale_notes("D＃", ""), vec!["D＃", "E＃", "F＃＃", "G＃", "A＃", "B＃", "C＃＃"]);
        assert_eq!(compute_scale_notes("A♭", "m"), vec!["A♭", "B♭", "C♭", "D♭", "E♭", "F♭", "G♭"]);
        assert_eq!(compute_scale_notes("G♭", ""), vec!["G♭", "A♭", "B♭", "C♭", "D♭", "E♭", "F"]);
        assert_eq!(compute_scale_notes("B＃", ""), vec!["B＃", "C＃＃", "D＃＃", "E＃", "F＃＃", "G＃＃", "A＃＃"]);
        assert_eq!(compute_scale_notes("C♭", "m"), vec!["C♭", "D♭", "E♭♭", "F♭", "G♭", "A♭♭", "B♭♭"]);
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
