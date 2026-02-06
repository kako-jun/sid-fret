//! 音名・ピッチの統一基盤
//! note_to_semitone, parse_pitch, absolute_semitone を一元化

use wasm_bindgen::prelude::*;

/// 12音配列（シャープ系）
pub const CHROMATIC_SHARP: [&str; 12] = [
    "C", "C＃", "D", "D＃", "E", "F", "F＃", "G", "G＃", "A", "A＃", "B",
];

/// 12音配列（フラット系）
pub const CHROMATIC_FLAT: [&str; 12] = [
    "C", "D♭", "D", "E♭", "E", "F", "G♭", "G", "A♭", "A", "B♭", "B",
];

/// 12音配列（両表記、"C＃/D♭" 形式）
pub const CHROMATIC_BOTH: [&str; 12] = [
    "C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B",
];

/// 音名から半音値を取得（C=0基準）
pub fn note_to_semitone(note: &str) -> Option<i32> {
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
pub fn parse_pitch(pitch: &str) -> Option<(String, i32)> {
    let mut name_end = pitch.len();
    for (i, c) in pitch.char_indices().rev() {
        if c.is_ascii_digit() || c == '-' {
            name_end = i;
        } else {
            break;
        }
    }

    if name_end == pitch.len() {
        return None;
    }

    let note_name = &pitch[..name_end];
    let octave_str = &pitch[name_end..];
    let octave = octave_str.parse::<i32>().ok()?;

    Some((note_name.to_string(), octave))
}

/// ピッチの絶対半音値を計算（C0 = 0）
pub fn absolute_semitone(pitch: &str) -> Option<i32> {
    let (note_name, octave) = parse_pitch(pitch)?;
    let semitone = note_to_semitone(&note_name)?;
    Some(octave * 12 + semitone)
}

/// ルート音がフラット系かを判定
pub fn is_flat_key(root: &str) -> bool {
    root.contains('♭') || matches!(root, "F")
}

/// ルート音に基づくピッチマップを計算で生成
/// CHROMATIC_BOTH をルートの位置でローテーション
pub fn pitch_map_for_root(root: &str) -> Vec<String> {
    let root_semitone = note_to_semitone(root).unwrap_or(0) as usize;
    (0..12)
        .map(|i| CHROMATIC_BOTH[(root_semitone + i) % 12].to_string())
        .collect()
}

/// E=0基準のフレットオフセットを計算
#[wasm_bindgen]
pub fn fret_offset(root: &str) -> i32 {
    let root_semi = note_to_semitone(root).unwrap_or(0);
    let e_semi = 4; // E = 4
    (root_semi - e_semi + 12) % 12
}

/// ピッチの異名同音比較（例: C＃2 == D♭2）
#[wasm_bindgen]
pub fn compare_pitch(pitch1: &str, pitch2: &str) -> bool {
    let p1 = pitch_identity(pitch1);
    let p2 = pitch_identity(pitch2);
    matches!((p1, p2), (Some(a), Some(b)) if a == b)
}

fn pitch_identity(pitch: &str) -> Option<(i32, i32)> {
    let (note, octave) = parse_pitch(pitch)?;
    let semitone = note_to_semitone(&note)?;
    Some((octave, semitone))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_semitone() {
        assert_eq!(note_to_semitone("C"), Some(0));
        assert_eq!(note_to_semitone("E"), Some(4));
        assert_eq!(note_to_semitone("B"), Some(11));
        assert_eq!(note_to_semitone("C＃"), Some(1));
        assert_eq!(note_to_semitone("D♭"), Some(1));
        assert_eq!(note_to_semitone("X"), None);
    }

    #[test]
    fn test_parse_pitch() {
        assert_eq!(parse_pitch("C3"), Some(("C".to_string(), 3)));
        assert_eq!(parse_pitch("E♭1"), Some(("E♭".to_string(), 1)));
        assert_eq!(parse_pitch("F＃2"), Some(("F＃".to_string(), 2)));
        assert_eq!(parse_pitch("C"), None);
    }

    #[test]
    fn test_absolute_semitone() {
        assert_eq!(absolute_semitone("C0"), Some(0));
        assert_eq!(absolute_semitone("C1"), Some(12));
        assert_eq!(absolute_semitone("E1"), Some(16));
        assert_eq!(absolute_semitone("A1"), Some(21));
    }

    #[test]
    fn test_pitch_map_for_root() {
        let map = pitch_map_for_root("C");
        assert_eq!(map[0], "C");
        assert_eq!(map[4], "E");
        assert_eq!(map[7], "G");

        let map = pitch_map_for_root("E");
        assert_eq!(map[0], "E");
        assert_eq!(map[1], "F");
    }

    #[test]
    fn test_compare_pitch() {
        assert!(compare_pitch("C2", "C2"));
        assert!(compare_pitch("C＃2", "D♭2"));
        assert!(!compare_pitch("C2", "D2"));
        assert!(!compare_pitch("C2", "C3"));
    }

    #[test]
    fn test_fret_offset() {
        assert_eq!(fret_offset("E"), 0);
        assert_eq!(fret_offset("C"), 8);
        assert_eq!(fret_offset("G"), 3);
        assert_eq!(fret_offset("A"), 5);
        assert_eq!(fret_offset("F"), 1);
        assert_eq!(fret_offset("D"), 10);
        assert_eq!(fret_offset("B"), 7);
        assert_eq!(fret_offset("E♭"), 11);
    }
}
