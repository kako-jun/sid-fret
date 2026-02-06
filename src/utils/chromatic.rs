use wasm_bindgen::prelude::*;

use crate::core::pitch::absolute_semitone;

/// クロマチックノート判定
/// nextNoteが存在し、かつ前後が半音（クロマチック）でつながっている場合true
#[wasm_bindgen]
pub fn is_chromatic_note(note_pitch: Option<String>, next_note_pitch: Option<String>) -> bool {
    let i1 = note_pitch.as_deref().and_then(absolute_semitone);
    let i2 = next_note_pitch.as_deref().and_then(absolute_semitone);
    matches!((i1, i2), (Some(a), Some(b)) if (a - b).abs() == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_semitone_via_pitch() {
        assert_eq!(absolute_semitone("C2"), Some(24));
        assert_eq!(absolute_semitone("C＃2"), Some(25));
        assert_eq!(absolute_semitone("D2"), Some(26));
    }

    #[test]
    fn test_is_chromatic_note() {
        // C2 -> C＃2 は半音差
        assert!(is_chromatic_note(
            Some("C2".to_string()),
            Some("C＃2".to_string())
        ));

        // C2 -> D2 は全音差
        assert!(!is_chromatic_note(
            Some("C2".to_string()),
            Some("D2".to_string())
        ));

        // None の場合
        assert!(!is_chromatic_note(None, Some("C2".to_string())));
        assert!(!is_chromatic_note(Some("C2".to_string()), None));
    }

    // ===== 仕様ベーステスト =====

    /// 半音関係の検出
    #[test]
    fn test_spec_chromatic_detection() {
        // 半音関係
        assert!(is_chromatic_note(Some("E1".to_string()), Some("F1".to_string())));
        assert!(is_chromatic_note(Some("B1".to_string()), Some("C2".to_string())));
        // 全音→false
        assert!(!is_chromatic_note(Some("C2".to_string()), Some("D2".to_string())));
        // None→false
        assert!(!is_chromatic_note(None, Some("E1".to_string())));
    }
}
