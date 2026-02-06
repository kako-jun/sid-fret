use wasm_bindgen::prelude::*;

use crate::core::chord_type::{get_chord_tones, parse_chord_type};
use crate::core::pitch::{absolute_semitone, pitch_map_for_root, strip_octave};

/// 2つのピッチ間の半音距離を計算
#[wasm_bindgen]
pub fn semitone_distance(pitch1: &str, pitch2: &str) -> i32 {
    let s1 = absolute_semitone(pitch1).unwrap_or(0);
    let s2 = absolute_semitone(pitch2).unwrap_or(0);
    s2 - s1
}

/// 半音数からインターバル名称を返す
#[wasm_bindgen]
pub fn interval_name(semitones: i32) -> String {
    if semitones < 0 {
        return format!("-{}", interval_name(-semitones));
    }

    match semitones {
        0 => "P1".to_string(),
        1 => "m2".to_string(),
        2 => "M2".to_string(),
        3 => "m3".to_string(),
        4 => "M3".to_string(),
        5 => "P4".to_string(),
        6 => "TT".to_string(),
        7 => "P5".to_string(),
        8 => "m6".to_string(),
        9 => "M6".to_string(),
        10 => "m7".to_string(),
        11 => "M7".to_string(),
        12 => "P8".to_string(),
        13 => "m9".to_string(),
        14 => "M9".to_string(),
        15 => "m10".to_string(),
        16 => "M10".to_string(),
        17 => "P11".to_string(),
        18 => "A11".to_string(),
        19 => "P12".to_string(),
        20 => "m13".to_string(),
        21 => "M13".to_string(),
        _ => format!("{semitones}st"),
    }
}

/// コードの転回形を判定
#[wasm_bindgen]
pub fn detect_inversion(chord: &str, bass_pitch: &str) -> i32 {
    let (root, chord_type) = parse_chord_type(chord);
    if root.is_empty() {
        return -1;
    }

    let frets = get_chord_tones(&chord_type);
    let pitch_map = pitch_map_for_root(&root);

    let bass_name = strip_octave(bass_pitch);

    for (i, tone) in frets.iter().enumerate() {
        let fret_semitone = tone.semitones as usize % 12;
        if fret_semitone < pitch_map.len() {
            let pitch_names = &pitch_map[fret_semitone];
            if pitch_names.split('/').any(|p| p == bass_name) {
                return i as i32;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semitone_distance() {
        assert_eq!(semitone_distance("E1", "A1"), 5);
        assert_eq!(semitone_distance("C3", "C4"), 12);
        assert_eq!(semitone_distance("G2", "E2"), -3);
        assert_eq!(semitone_distance("B0", "C1"), 1);
    }

    #[test]
    fn test_interval_name() {
        assert_eq!(interval_name(0), "P1");
        assert_eq!(interval_name(7), "P5");
        assert_eq!(interval_name(12), "P8");
        assert_eq!(interval_name(14), "M9");
        assert_eq!(interval_name(-7), "-P5");
    }

    #[test]
    fn test_detect_inversion() {
        assert_eq!(detect_inversion("C", "C1"), 0);
        assert_eq!(detect_inversion("C", "E1"), 1);
        assert_eq!(detect_inversion("C", "G1"), 2);
        assert_eq!(detect_inversion("Cmaj7", "B2"), 3);
        assert_eq!(detect_inversion("C", "F1"), -1);
    }
}
