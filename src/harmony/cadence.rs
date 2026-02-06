use wasm_bindgen::prelude::*;

/// カデンツの種類を判定（12パターン対応）
#[wasm_bindgen]
pub fn cadence_text(prev_functional_harmony: i32, functional_harmony: i32) -> String {
    match (prev_functional_harmony, functional_harmony) {
        (5, 1) => "Perfect Cadence".to_string(),
        (4, 1) => "Plagal Cadence".to_string(),
        (7, 1) => "Leading-tone Cadence".to_string(),
        (5, 6) => "Deceptive Cadence".to_string(),
        (5, 4) => "Interrupted Cadence".to_string(),
        (_, 5) => "Half Cadence".to_string(),
        (_, 7) => "Phrygian Cadence".to_string(),
        _ => String::new(),
    }
}

/// 3コード進行のカデンツ判定（ii-V-I対応）
#[wasm_bindgen]
pub fn cadence_text_extended(
    prev2_harmony: i32,
    prev_harmony: i32,
    current_harmony: i32,
) -> String {
    // ii-V-I
    if prev2_harmony == 2 && prev_harmony == 5 && current_harmony == 1 {
        return "ii-V-I Cadence".to_string();
    }

    // 2コードの判定にフォールバック
    cadence_text(prev_harmony, current_harmony)
}

/// スケール度数からT/S/D機能分類を返す
#[wasm_bindgen]
pub fn functional_area(degree: i32) -> String {
    match degree {
        1 | 3 | 6 => "T".to_string(),
        2 | 4 => "S".to_string(),
        5 | 7 => "D".to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadence_text() {
        assert_eq!(cadence_text(5, 1), "Perfect Cadence");
        assert_eq!(cadence_text(4, 1), "Plagal Cadence");
        assert_eq!(cadence_text(5, 6), "Deceptive Cadence");
        assert_eq!(cadence_text(1, 5), "Half Cadence");
        assert_eq!(cadence_text(2, 7), "Phrygian Cadence");
        assert_eq!(cadence_text(1, 2), "");
    }

    #[test]
    fn test_extended_cadences() {
        assert_eq!(cadence_text(5, 4), "Interrupted Cadence");
        assert_eq!(cadence_text(7, 1), "Leading-tone Cadence");
    }

    #[test]
    fn test_cadence_text_extended_ii_v_i() {
        assert_eq!(cadence_text_extended(2, 5, 1), "ii-V-I Cadence");
        // フォールバック
        assert_eq!(cadence_text_extended(1, 4, 1), "Plagal Cadence");
    }

    #[test]
    fn test_functional_area() {
        assert_eq!(functional_area(1), "T");
        assert_eq!(functional_area(2), "S");
        assert_eq!(functional_area(3), "T");
        assert_eq!(functional_area(4), "S");
        assert_eq!(functional_area(5), "D");
        assert_eq!(functional_area(6), "T");
        assert_eq!(functional_area(7), "D");
        assert_eq!(functional_area(0), "");
    }

    // ===== 仕様ベーステスト =====

    /// 全7パターン網羅
    #[test]
    fn test_spec_all_cadence_patterns() {
        assert_eq!(cadence_text(5, 1), "Perfect Cadence");
        assert_eq!(cadence_text(4, 1), "Plagal Cadence");
        assert_eq!(cadence_text(7, 1), "Leading-tone Cadence");
        assert_eq!(cadence_text(5, 6), "Deceptive Cadence");
        assert_eq!(cadence_text(5, 4), "Interrupted Cadence");
        // *→5 は全てHalf Cadence
        assert_eq!(cadence_text(1, 5), "Half Cadence");
        assert_eq!(cadence_text(2, 5), "Half Cadence");
        assert_eq!(cadence_text(3, 5), "Half Cadence");
        assert_eq!(cadence_text(4, 5), "Half Cadence");
        assert_eq!(cadence_text(6, 5), "Half Cadence");
        // *→7 は全てPhrygian Cadence
        assert_eq!(cadence_text(1, 7), "Phrygian Cadence");
        assert_eq!(cadence_text(4, 7), "Phrygian Cadence");
        assert_eq!(cadence_text(2, 7), "Phrygian Cadence");
    }

    /// カデンツなし
    #[test]
    fn test_spec_cadence_no_match() {
        assert_eq!(cadence_text(1, 2), "");
        assert_eq!(cadence_text(3, 4), "");
        assert_eq!(cadence_text(6, 2), "");
        assert_eq!(cadence_text(0, 0), "");
    }

    /// ii-V-I 拡張
    #[test]
    fn test_spec_ii_v_i_extended() {
        assert_eq!(cadence_text_extended(2, 5, 1), "ii-V-I Cadence");
        // ii-V-I ではないのでフォールバック
        assert_eq!(cadence_text_extended(3, 5, 1), "Perfect Cadence");
        assert_eq!(cadence_text_extended(2, 5, 6), "Deceptive Cadence");
    }

    /// 全度数の機能分類
    #[test]
    fn test_spec_functional_area_all_degrees() {
        assert_eq!(functional_area(1), "T");
        assert_eq!(functional_area(2), "S");
        assert_eq!(functional_area(3), "T");
        assert_eq!(functional_area(4), "S");
        assert_eq!(functional_area(5), "D");
        assert_eq!(functional_area(6), "T");
        assert_eq!(functional_area(7), "D");
        assert_eq!(functional_area(0), "");
        assert_eq!(functional_area(8), "");
    }
}
