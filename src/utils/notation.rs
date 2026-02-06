use wasm_bindgen::prelude::*;

use crate::core::pitch::parse_pitch;

/// 五線譜のライン番号を計算（E1=0.0基準）
/// ダイアトニック位置 + オクターブ×7 からE1のオフセットを引く
/// シャープは+0.5、フラットは-0.5（E＃/B＃/F♭/C♭は異名同音扱い）
#[wasm_bindgen]
pub fn get_line(pitch: &str) -> Option<f32> {
    let (note, octave) = parse_pitch(pitch)?;

    let base = note.chars().next()?;
    let accidental = &note[base.len_utf8()..];

    // ダイアトニック位置: C=0, D=1, E=2, F=3, G=4, A=5, B=6
    let diatonic = match base {
        'C' => 0,
        'D' => 1,
        'E' => 2,
        'F' => 3,
        'G' => 4,
        'A' => 5,
        'B' => 6,
        _ => return None,
    };

    // E＃→F位置、B＃→C(次オクターブ)位置、F♭→E位置、C♭→B(前オクターブ)位置
    let (d, oct_adj, acc) = match (base, accidental) {
        ('E', "＃") => (3, 0, 0.0),
        ('B', "＃") => (0, 1, 0.0),
        ('F', "♭") => (2, 0, 0.0),
        ('C', "♭") => (6, -1, 0.0),
        (_, "＃") => (diatonic, 0, 0.5),
        (_, "♭") => (diatonic, 0, -0.5),
        (_, "") => (diatonic, 0, 0.0),
        _ => return None,
    };

    // E1 = 0.0 (オフセット = 1*7 + 2 = 9)
    let line = (octave + oct_adj) as f32 * 7.0 + d as f32 + acc - 9.0;

    // E1 (0.0) 〜 G4 (23.0) の範囲
    if (0.0..=23.0).contains(&line) {
        Some(line)
    } else {
        None
    }
}

/// 五度圏での位置
#[wasm_bindgen]
pub struct KeyPosition {
    circle: String,
    index: i32,
}

#[wasm_bindgen]
impl KeyPosition {
    #[wasm_bindgen(getter)]
    pub fn circle(&self) -> String {
        self.circle.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn index(&self) -> i32 {
        self.index
    }
}

#[wasm_bindgen]
pub fn get_key_position(scale: &str) -> KeyPosition {
    let major_keys = [
        "C", "G", "D", "A", "E", "B", "F＃", "D♭", "A♭", "E♭", "B♭", "F",
    ];
    let minor_keys = [
        "Am", "Em", "Bm", "F＃m", "C＃m", "G＃m", "D＃m", "B♭m", "Fm", "Cm", "Gm", "Dm",
    ];

    if let Some(idx) = major_keys.iter().position(|&k| k == scale) {
        KeyPosition {
            circle: "outer".to_string(),
            index: idx as i32,
        }
    } else if let Some(idx) = minor_keys.iter().position(|&k| k == scale) {
        KeyPosition {
            circle: "inner".to_string(),
            index: idx as i32,
        }
    } else {
        KeyPosition {
            circle: "none".to_string(),
            index: -1,
        }
    }
}

/// 音符の値を英語テキストに変換
#[wasm_bindgen]
pub fn value_text(value: &str) -> String {
    match value {
        "whole" => "Whole Note",
        "dotted_whole" => "Dotted Whole Note",
        "half" => "Half Note",
        "dotted_half" => "Dotted Half Note",
        "quarter" => "Quarter Note",
        "dotted_quarter" => "Dotted Quarter Note",
        "8th" => "8th Note",
        "dotted_8th" => "Dotted 8th Note",
        "16th" => "16th Note",
        "dotted_16th" => "Dotted 16th Note",
        "triplet_quarter" => "Quarter Triplet",
        "triplet_8th" => "8th Triplet",
        "triplet_16th" => "16th Triplet",
        _ => "",
    }
    .to_string()
}

/// スケール名の英語表記を取得
#[wasm_bindgen]
pub fn scale_text(scale: &str) -> String {
    use crate::core::scale_type::parse_scale_key;

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
    fn test_get_line() {
        assert_eq!(get_line("E1"), Some(0.0));
        assert_eq!(get_line("C2"), Some(5.0));
        assert_eq!(get_line("G4"), Some(23.0));
        assert_eq!(get_line("X1"), None);

        // シャープ/フラット
        assert_eq!(get_line("F＃1"), Some(1.5));
        assert_eq!(get_line("G♭1"), Some(1.5));
        assert_eq!(get_line("A♭2"), Some(9.5));

        // 異名同音の特殊ケース
        assert_eq!(get_line("E＃1"), Some(1.0));
        assert_eq!(get_line("F♭1"), Some(0.0));
        assert_eq!(get_line("B＃1"), Some(5.0));
        assert_eq!(get_line("C♭2"), Some(4.0));

        // 範囲外
        assert_eq!(get_line("D1"), None);
        assert_eq!(get_line("A4"), None);
    }

    #[test]
    fn test_value_text() {
        assert_eq!(value_text("whole"), "Whole Note");
        assert_eq!(value_text("quarter"), "Quarter Note");
        assert_eq!(value_text("unknown"), "");
    }

    #[test]
    fn test_scale_text() {
        assert_eq!(scale_text("C"), "C Major Scale");
        assert_eq!(scale_text("Am"), "A Minor Scale");
        assert_eq!(scale_text("C_dorian"), "C Dorian Scale");
        assert_eq!(scale_text("A_blues"), "A Blues Scale");
    }
}
