use wasm_bindgen::prelude::*;

use crate::chord::parser::{get_frets, get_pitch_map, parse_chord_type};

/// 音名から半音値を取得（C=0基準）
fn note_name_to_semitone(note: &str) -> Option<i32> {
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

/// ピッチ文字列（例: "C3", "E♭1"）を音名とオクターブに分割
fn parse_pitch(pitch: &str) -> Option<(String, i32)> {
    // 末尾の数字をオクターブとして抽出
    let mut name_end = pitch.len();
    for (i, c) in pitch.char_indices().rev() {
        if c.is_ascii_digit() || c == '-' {
            name_end = i;
        } else {
            break;
        }
    }

    if name_end == pitch.len() {
        return None; // オクターブ番号なし
    }

    let note_name = &pitch[..name_end];
    let octave_str = &pitch[name_end..];
    let octave = octave_str.parse::<i32>().ok()?;

    Some((note_name.to_string(), octave))
}

/// ピッチの絶対半音値を計算
fn absolute_semitone(pitch: &str) -> Option<i32> {
    let (note_name, octave) = parse_pitch(pitch)?;
    let semitone = note_name_to_semitone(&note_name)?;
    Some(octave * 12 + semitone)
}

/// 2つのピッチ間の半音距離を計算
/// pitch1: "E1", pitch2: "A1" -> 5
/// pitch1: "C3", pitch2: "E1" -> -20 (下方向は負)
#[wasm_bindgen]
pub fn semitone_distance(pitch1: &str, pitch2: &str) -> i32 {
    let s1 = absolute_semitone(pitch1).unwrap_or(0);
    let s2 = absolute_semitone(pitch2).unwrap_or(0);
    s2 - s1
}

/// 半音数からインターバル名称を返す
/// 0 -> "P1", 7 -> "P5", 14 -> "M9"
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
/// chord: "C", bass_pitch: "C1" -> 0 (基本形)
/// chord: "C", bass_pitch: "E1" -> 1 (第1転回形)
/// chord: "C", bass_pitch: "G1" -> 2 (第2転回形)
/// chord: "Cmaj7", bass_pitch: "B2" -> 3 (第3転回形)
/// 構成音にない場合 -> -1
#[wasm_bindgen]
pub fn detect_inversion(chord: &str, bass_pitch: &str) -> i32 {
    let (root, chord_type) = parse_chord_type(chord);
    if root.is_empty() {
        return -1;
    }

    let frets = get_frets(&chord_type);
    let pitch_map = get_pitch_map(&root);

    // バス音の音名（オクターブ除去）
    let bass_name = match parse_pitch(bass_pitch) {
        Some((name, _)) => name,
        None => bass_pitch.replace(char::is_numeric, ""),
    };

    // 各構成音と比較
    for (i, fret) in frets.iter().enumerate() {
        let fret_semitone = fret.fret as usize % 12;
        if fret_semitone < pitch_map.len() {
            let pitch_names = &pitch_map[fret_semitone];
            // pitch_map の各エントリは "C＃/D♭" のような形式
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

    #[test]
    fn test_parse_pitch() {
        assert_eq!(parse_pitch("C3"), Some(("C".to_string(), 3)));
        assert_eq!(parse_pitch("E♭1"), Some(("E♭".to_string(), 1)));
        assert_eq!(parse_pitch("F＃2"), Some(("F＃".to_string(), 2)));
    }
}
