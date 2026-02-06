//! ダイアトニックコード生成

use wasm_bindgen::prelude::*;

use crate::core::scale_type::{compute_scale_notes, parse_scale_key};

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
        _ => vec![],
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

/// ダイアトニックコード（トライアド）を取得
#[wasm_bindgen]
pub fn get_scale_diatonic_chords(scale: &str) -> Vec<JsValue> {
    let chords = get_scale_diatonic_chords_internal(scale);
    chords.iter().map(|s| JsValue::from_str(s)).collect()
}

/// 内部用: ダイアトニックコードをStringのVecで返す
pub(crate) fn get_scale_diatonic_chords_internal(scale: &str) -> Vec<String> {
    let (root, scale_type) = parse_scale_key(scale);
    let notes = compute_scale_notes(&root, &scale_type);
    if notes.is_empty() {
        return vec![];
    }

    let qualities = diatonic_triad_qualities(&scale_type);
    if qualities.is_empty() {
        return vec![];
    }

    notes
        .iter()
        .zip(qualities.iter())
        .map(|(note, quality)| format!("{note}{quality}"))
        .collect()
}

/// ダイアトニックコード（7th）を取得
#[wasm_bindgen]
pub fn get_scale_diatonic_chords_with_7th(scale: &str) -> Vec<JsValue> {
    let chords = get_scale_diatonic_chords_7th_internal(scale);
    chords.iter().map(|s| JsValue::from_str(s)).collect()
}

/// 内部用: ダイアトニック7thコードをStringのVecで返す
pub(crate) fn get_scale_diatonic_chords_7th_internal(scale: &str) -> Vec<String> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diatonic_chords() {
        let c_major = get_scale_diatonic_chords_internal("C");
        assert_eq!(c_major.len(), 7);
        assert_eq!(c_major[0], "C");
        assert_eq!(c_major[4], "G");
    }

    #[test]
    fn test_diatonic_chords_computed() {
        let chords = get_scale_diatonic_chords_internal("C_dorian");
        assert_eq!(chords.len(), 7);
        assert_eq!(chords[0], "Cm");
        assert_eq!(chords[3], "F");
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
        assert_eq!(chords[0], "Cm7");
        assert_eq!(chords[3], "F7");
    }
}
