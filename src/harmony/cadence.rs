use wasm_bindgen::prelude::*;

/// カデンツの種類を判定
/// - Perfect Cadence (V → I): 完全終止
/// - Plagal Cadence (IV → I): アーメン終止
/// - Deceptive Cadence (V → VI): 偽終止
/// - Half Cadence (→ V): 半終止
/// - Phrygian Cadence (→ VII): フリギア終止
#[wasm_bindgen]
pub fn cadence_text(prev_functional_harmony: i32, functional_harmony: i32) -> String {
    if prev_functional_harmony == 5 && functional_harmony == 1 {
        "Perfect Cadence".to_string()
    } else if prev_functional_harmony == 4 && functional_harmony == 1 {
        "Plagal Cadence".to_string()
    } else if prev_functional_harmony == 5 && functional_harmony == 6 {
        "Deceptive Cadence".to_string()
    } else if functional_harmony == 5 {
        "Half Cadence".to_string()
    } else if functional_harmony == 7 {
        "Phrygian Cadence".to_string()
    } else {
        String::new()
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
}
