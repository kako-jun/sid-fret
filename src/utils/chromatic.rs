use wasm_bindgen::prelude::*;

/// クロマチックノート判定
/// 2つの音が半音（クロマチック）でつながっている場合true
#[wasm_bindgen]
pub fn is_chromatic_note(note_pitch: Option<String>, next_note_pitch: Option<String>) -> bool {
    let pitch1 = match note_pitch {
        Some(p) => p,
        None => return false,
    };

    let pitch2 = match next_note_pitch {
        Some(p) => p,
        None => return false,
    };

    let i1 = match get_absolute_pitch_index(&pitch1) {
        Some(i) => i,
        None => return false,
    };

    let i2 = match get_absolute_pitch_index(&pitch2) {
        Some(i) => i,
        None => return false,
    };

    (i1 as i32 - i2 as i32).abs() == 1
}

/// 絶対音高インデックスを取得（C0 = 0, C1 = 12, ...）
fn get_absolute_pitch_index(pitch: &str) -> Option<usize> {
    let mut chars = pitch.chars().peekable();

    // 音名を抽出（A-G）
    let mut name = String::new();
    if let Some(c) = chars.next() {
        if !('A'..='G').contains(&c) {
            return None;
        }
        name.push(c);
    } else {
        return None;
    }

    // 変化記号を抽出（＃または♭）
    if let Some(&c) = chars.peek() {
        if c == '＃' || c == '♭' {
            name.push(c);
            chars.next();
        }
    }

    // オクターブ番号を抽出
    let octave_str: String = chars.collect();
    let octave: usize = octave_str.parse().ok()?;

    // 半音階での位置を特定
    let chromatic = [
        "C",
        "C＃/D♭",
        "D",
        "D＃/E♭",
        "E",
        "F",
        "F＃/G♭",
        "G",
        "G＃/A♭",
        "A",
        "A＃/B♭",
        "B",
    ];

    let idx = chromatic
        .iter()
        .position(|x| x.split('/').any(|part| part == name))?;

    Some(octave * 12 + idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_absolute_pitch_index() {
        assert_eq!(get_absolute_pitch_index("C2"), Some(24));
        assert_eq!(get_absolute_pitch_index("C＃2"), Some(25));
        assert_eq!(get_absolute_pitch_index("D♭2"), Some(25));
        assert_eq!(get_absolute_pitch_index("D2"), Some(26));
        assert_eq!(get_absolute_pitch_index("B2"), Some(35));
    }

    #[test]
    fn test_is_chromatic_note() {
        // 半音関係
        assert!(is_chromatic_note(
            Some("C2".to_string()),
            Some("C＃2".to_string())
        ));
        assert!(is_chromatic_note(
            Some("C2".to_string()),
            Some("D♭2".to_string())
        ));

        // 全音関係（半音ではない）
        assert!(!is_chromatic_note(
            Some("C2".to_string()),
            Some("D2".to_string())
        ));

        // None の場合
        assert!(!is_chromatic_note(None, Some("C2".to_string())));
        assert!(!is_chromatic_note(Some("C2".to_string()), None));
    }
}
