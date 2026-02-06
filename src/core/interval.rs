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

    // ===== 仕様ベーステスト =====

    /// 0〜21の全インターバル名
    #[test]
    fn test_spec_all_interval_names() {
        let expected = [
            "P1", "m2", "M2", "m3", "M3", "P4", "TT",
            "P5", "m6", "M6", "m7", "M7", "P8",
            "m9", "M9", "m10", "M10", "P11", "A11",
            "P12", "m13", "M13",
        ];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(interval_name(i as i32), *name, "semitone={i}");
        }
    }

    /// 範囲外のインターバル名
    #[test]
    fn test_spec_interval_name_overflow() {
        assert_eq!(interval_name(22), "22st");
        assert_eq!(interval_name(24), "24st");
        assert_eq!(interval_name(-3), "-m3");
        assert_eq!(interval_name(-12), "-P8");
    }

    /// 異名同音ピッチの距離
    #[test]
    fn test_spec_semitone_distance_enharmonic() {
        assert_eq!(semitone_distance("C＃2", "D♭2"), 0);
        assert_eq!(semitone_distance("E1", "F♭1"), 0);
        assert_eq!(semitone_distance("B＃1", "C1"), 0); // B#=C, both semi=0
    }

    /// 実際の音程
    #[test]
    fn test_spec_semitone_distance_real_intervals() {
        assert_eq!(semitone_distance("E1", "G1"), 3);   // 短3度
        assert_eq!(semitone_distance("C2", "G2"), 7);   // 完全5度
        assert_eq!(semitone_distance("A1", "E2"), 7);   // 完全5度
        assert_eq!(semitone_distance("E1", "E2"), 12);  // オクターブ
        assert_eq!(semitone_distance("E1", "G＃1"), 4); // 長3度
        assert_eq!(semitone_distance("E1", "B♭1"), 6);  // トライトーン
    }

    /// 各コードタイプの転回形
    #[test]
    fn test_spec_detect_inversion_all_types() {
        // minor triad
        assert_eq!(detect_inversion("Cm", "E♭1"), 1);
        assert_eq!(detect_inversion("Cm", "G1"), 2);
        // dom7
        assert_eq!(detect_inversion("C7", "B♭2"), 3);
        // dim
        assert_eq!(detect_inversion("Cdim", "G♭1"), 2);
        // maj7
        assert_eq!(detect_inversion("Cmaj7", "E1"), 1);
        assert_eq!(detect_inversion("Cmaj7", "G2"), 2);
        assert_eq!(detect_inversion("Cmaj7", "B2"), 3);
        // m7
        assert_eq!(detect_inversion("Cm7", "E♭1"), 1);
        assert_eq!(detect_inversion("Cm7", "B♭2"), 3);
        // sus4
        assert_eq!(detect_inversion("Csus4", "F2"), 1);
        assert_eq!(detect_inversion("Csus4", "G2"), 2);
    }

    /// ＃/♭コードの転回形
    #[test]
    fn test_spec_detect_inversion_sharp_flat_chords() {
        assert_eq!(detect_inversion("F＃", "A＃1"), 1);
        assert_eq!(detect_inversion("B♭m", "D♭2"), 1);
        assert_eq!(detect_inversion("E♭maj7", "D2"), 3);
    }

    /// 非構成音→-1
    #[test]
    fn test_spec_detect_inversion_non_member() {
        assert_eq!(detect_inversion("C", "F1"), -1);
        assert_eq!(detect_inversion("C", "B♭2"), -1);
        assert_eq!(detect_inversion("Cm7", "F＃1"), -1);
    }
}
