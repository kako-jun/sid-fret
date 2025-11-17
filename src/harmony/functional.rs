use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 機能和声情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonyInfo {
    roman: String,
    function: String,
}

impl HarmonyInfo {
    pub fn new(roman: String, function: String) -> Self {
        Self { roman, function }
    }

    pub fn roman(&self) -> &str {
        &self.roman
    }

    pub fn function(&self) -> &str {
        &self.function
    }
}

/// スケールのダイアトニックコードマップを作成
fn create_diatonic_chord_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // メジャースケール
    map.insert("C", vec!["C", "Dm", "Em", "F", "G", "Am", "Bdim"]);
    map.insert("D", vec!["D", "Em", "F＃m", "G", "A", "Bm", "C＃dim"]);
    map.insert("E", vec!["E", "F＃m", "G＃m", "A", "B", "C＃m", "D＃dim"]);
    map.insert("F", vec!["F", "Gm", "Am", "B♭", "C", "Dm", "Edim"]);
    map.insert("G", vec!["G", "Am", "Bm", "C", "D", "Em", "F＃dim"]);
    map.insert("A", vec!["A", "Bm", "C＃m", "D", "E", "F＃m", "G＃dim"]);
    map.insert("B", vec!["B", "C＃m", "D＃m", "E", "F＃", "G＃m", "A＃dim"]);

    // マイナースケール
    map.insert("Cm", vec!["Cm", "Ddim", "E♭", "Fm", "Gm", "A♭", "B♭"]);
    map.insert("Dm", vec!["Dm", "Edim", "F", "Gm", "Am", "B♭", "C"]);
    map.insert("Em", vec!["Em", "F＃dim", "G", "Am", "Bm", "C", "D"]);
    map.insert("Fm", vec!["Fm", "Gdim", "A♭", "B♭m", "Cm", "D♭", "E♭"]);
    map.insert("Gm", vec!["Gm", "Adim", "B♭", "Cm", "Dm", "E♭", "F"]);
    map.insert("Am", vec!["Am", "Bdim", "C", "Dm", "Em", "F", "G"]);
    map.insert("Bm", vec!["Bm", "C＃dim", "D", "Em", "F＃m", "G", "A"]);

    map
}

/// 機能和声の度数を取得（I-VII: 1-7、見つからない場合: 0）
#[wasm_bindgen]
pub fn get_functional_harmony(scale: &str, chord: &str) -> i32 {
    let chord_map = create_diatonic_chord_map();

    if let Some(chords) = chord_map.get(scale) {
        chords
            .iter()
            .position(|&c| c == chord)
            .map(|pos| (pos + 1) as i32)
            .unwrap_or(0)
    } else {
        0
    }
}

/// 機能和声のテキスト表現を取得
#[wasm_bindgen]
pub fn functional_harmony_text(degree: i32) -> String {
    match degree {
        1 => "Ⅰ Tonic".to_string(),
        2 => "Ⅱ Supertonic".to_string(),
        3 => "Ⅲ Mediant".to_string(),
        4 => "Ⅳ Subdominant".to_string(),
        5 => "Ⅴ Dominant".to_string(),
        6 => "Ⅵ Submediant".to_string(),
        7 => "Ⅶ Leading Tone".to_string(),
        _ => String::new(),
    }
}

/// ローマ数字記譜と機能名を取得
#[wasm_bindgen]
pub fn roman_numeral_harmony_info(degree: i32) -> JsValue {
    let info = match degree {
        1 => HarmonyInfo::new("Ⅰ".to_string(), "Tonic".to_string()),
        2 => HarmonyInfo::new("Ⅱ".to_string(), "Supertonic".to_string()),
        3 => HarmonyInfo::new("Ⅲ".to_string(), "Mediant".to_string()),
        4 => HarmonyInfo::new("Ⅳ".to_string(), "Subdominant".to_string()),
        5 => HarmonyInfo::new("Ⅴ".to_string(), "Dominant".to_string()),
        6 => HarmonyInfo::new("Ⅵ".to_string(), "Submediant".to_string()),
        7 => HarmonyInfo::new("Ⅶ".to_string(), "Leading Tone".to_string()),
        _ => HarmonyInfo::new(String::new(), String::new()),
    };

    serde_wasm_bindgen::to_value(&info).unwrap()
}

/// コード内でのピッチの役割を判定（ルート音かどうか）
#[wasm_bindgen]
pub fn get_chord_tone_label(_scale: &str, chord: &str, pitch: &str) -> String {
    // 簡易実装：コード名の最初の文字とピッチが一致すればルート
    if chord.starts_with(pitch) {
        "Root".to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_functional_harmony() {
        assert_eq!(get_functional_harmony("C", "C"), 1);
        assert_eq!(get_functional_harmony("C", "G"), 5);
        assert_eq!(get_functional_harmony("C", "Am"), 6);
        assert_eq!(get_functional_harmony("Am", "Am"), 1);
    }

    #[test]
    fn test_functional_harmony_text() {
        assert_eq!(functional_harmony_text(1), "Ⅰ Tonic");
        assert_eq!(functional_harmony_text(5), "Ⅴ Dominant");
        assert_eq!(functional_harmony_text(0), "");
    }

    #[test]
    fn test_get_chord_tone_label() {
        assert_eq!(get_chord_tone_label("C", "C", "C"), "Root");
        assert_eq!(get_chord_tone_label("C", "Dm", "D"), "Root");
    }
}
