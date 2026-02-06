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

    // ===== 仕様ベーステスト =====

    /// Cメジャーのトライアド
    #[test]
    fn test_spec_c_major_diatonic_triads() {
        let chords = get_scale_diatonic_chords_internal("C");
        assert_eq!(chords, vec!["C", "Dm", "Em", "F", "G", "Am", "Bdim"]);
    }

    /// Aマイナーのトライアド
    #[test]
    fn test_spec_a_minor_diatonic_triads() {
        let chords = get_scale_diatonic_chords_internal("Am");
        assert_eq!(chords, vec!["Am", "Bdim", "C", "Dm", "Em", "F", "G"]);
    }

    /// 各モードの品質パターン検証
    #[test]
    fn test_spec_all_mode_diatonic_triads() {
        assert_eq!(
            get_scale_diatonic_chords_internal("C_dorian"),
            vec!["Cm", "Dm", "E♭", "F", "Gm", "Adim", "B♭"]
        );
        assert_eq!(
            get_scale_diatonic_chords_internal("C_phrygian"),
            vec!["Cm", "D♭", "E♭", "Fm", "Gdim", "A♭", "B♭m"]
        );
        assert_eq!(
            get_scale_diatonic_chords_internal("C_lydian"),
            vec!["C", "D", "Em", "F＃dim", "G", "Am", "Bm"]
        );
        assert_eq!(
            get_scale_diatonic_chords_internal("C_mixolydian"),
            vec!["C", "Dm", "Edim", "F", "Gm", "Am", "B♭"]
        );
        assert_eq!(
            get_scale_diatonic_chords_internal("C_locrian"),
            vec!["Cdim", "D♭", "E♭m", "Fm", "G♭", "A♭", "B♭m"]
        );
    }

    /// Cメジャーの7thコード
    #[test]
    fn test_spec_c_major_diatonic_7ths() {
        let chords = get_scale_diatonic_chords_7th_internal("C");
        assert_eq!(chords, vec!["Cmaj7", "Dm7", "Em7", "Fmaj7", "G7", "Am7", "Bm7♭5"]);
    }

    /// ハーモニックマイナーのダイアトニック
    #[test]
    fn test_spec_harm_minor_diatonic() {
        let triads = get_scale_diatonic_chords_internal("C_harm_minor");
        assert_eq!(triads, vec!["Cm", "Ddim", "E♭aug", "Fm", "G", "A♭", "Bdim"]);

        let sevenths = get_scale_diatonic_chords_7th_internal("C_harm_minor");
        assert_eq!(sevenths, vec!["Cm(maj7)", "Dm7♭5", "E♭aug(maj7)", "Fm7", "G7", "A♭maj7", "Bdim7"]);
    }

    /// メロディックマイナーのダイアトニック
    #[test]
    fn test_spec_melo_minor_diatonic() {
        let triads = get_scale_diatonic_chords_internal("C_melo_minor");
        assert_eq!(triads, vec!["Cm", "Dm", "E♭aug", "F", "G", "Adim", "Bdim"]);

        let sevenths = get_scale_diatonic_chords_7th_internal("C_melo_minor");
        assert_eq!(sevenths, vec!["Cm(maj7)", "Dm7", "E♭aug(maj7)", "F7", "G7", "Am7♭5", "Bm7♭5"]);
    }

    /// ＃キーのダイアトニック
    #[test]
    fn test_spec_sharp_key_diatonic() {
        let chords = get_scale_diatonic_chords_internal("F＃");
        assert_eq!(chords, vec!["F＃", "G＃m", "A＃m", "B", "C＃", "D＃m", "E＃dim"]);
    }

    /// ♭キーのダイアトニック
    #[test]
    fn test_spec_flat_key_diatonic() {
        assert_eq!(
            get_scale_diatonic_chords_internal("B♭"),
            vec!["B♭", "Cm", "Dm", "E♭", "F", "Gm", "Adim"]
        );
        assert_eq!(
            get_scale_diatonic_chords_internal("E♭"),
            vec!["E♭", "Fm", "Gm", "A♭", "B♭", "Cm", "Ddim"]
        );
    }

    /// ペンタ/ブルース→空Vec
    #[test]
    fn test_spec_pentatonic_blues_no_diatonic() {
        assert!(get_scale_diatonic_chords_internal("C_penta").is_empty());
        assert!(get_scale_diatonic_chords_internal("C_m_penta").is_empty());
        assert!(get_scale_diatonic_chords_internal("C_blues").is_empty());
    }
}
