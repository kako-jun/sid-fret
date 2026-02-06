use crate::chord::get_interval;
use crate::scale::diatonic::create_diatonic_chord_map;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// 機能和声のディグリー番号を取得（harmonyUtil.ts の getFunctionalHarmony() に相当）
#[wasm_bindgen]
pub fn get_functional_harmony(scale: &str, chord: &str) -> i32 {
    let chord_map = create_diatonic_chord_map();
    let empty_vec = vec![];
    let chords = chord_map.get(scale).unwrap_or(&empty_vec);

    if let Some(index) = chords.iter().position(|c| c == &chord) {
        (index + 1) as i32
    } else {
        0
    }
}

/// 機能和声のテキスト表示
#[wasm_bindgen]
pub fn functional_harmony_text(degree: i32) -> String {
    match degree {
        1 => "Ⅰ Tonic",
        2 => "Ⅱ Supertonic",
        3 => "Ⅲ Mediant",
        4 => "Ⅳ Subdominant",
        5 => "Ⅴ Dominant",
        6 => "Ⅵ Submediant",
        7 => "Ⅶ Leading Tone",
        _ => "",
    }
    .to_string()
}

/// 機能和声情報
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HarmonyInfo {
    roman: String,
    desc: String,
}

#[wasm_bindgen]
impl HarmonyInfo {
    #[wasm_bindgen(getter)]
    pub fn roman(&self) -> String {
        self.roman.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn desc(&self) -> String {
        self.desc.clone()
    }
}

/// 音階度の情報を取得
#[wasm_bindgen]
pub fn functional_harmony_info(degree: i32) -> HarmonyInfo {
    match degree {
        1 => HarmonyInfo {
            roman: "Ⅰ".to_string(),
            desc: "Tonic (主音): 安心・落ち着き".to_string(),
        },
        2 => HarmonyInfo {
            roman: "Ⅱ".to_string(),
            desc: "Supertonic (上主音): 期待・問い".to_string(),
        },
        3 => HarmonyInfo {
            roman: "Ⅲ".to_string(),
            desc: "Mediant (中音): 穏やか・中間".to_string(),
        },
        4 => HarmonyInfo {
            roman: "Ⅳ".to_string(),
            desc: "Subdominant (下属音): 広がり・始まり".to_string(),
        },
        5 => HarmonyInfo {
            roman: "Ⅴ".to_string(),
            desc: "Dominant (属音): 緊張・推進".to_string(),
        },
        6 => HarmonyInfo {
            roman: "Ⅵ".to_string(),
            desc: "Submediant (下中音): 儚さ・哀愁".to_string(),
        },
        7 => HarmonyInfo {
            roman: "Ⅶ".to_string(),
            desc: "Leading Tone (導音): 不安・未解決".to_string(),
        },
        _ => HarmonyInfo {
            roman: "".to_string(),
            desc: "".to_string(),
        },
    }
}

/// トライアド和音のローマ数字表記情報
#[wasm_bindgen]
pub fn roman_numeral_harmony_info(degree: i32) -> HarmonyInfo {
    match degree {
        1 => HarmonyInfo {
            roman: "Ⅰ".to_string(),
            desc: "Tonic (主和音・長三和音): 安心・落ち着き".to_string(),
        },
        2 => HarmonyInfo {
            roman: "Ⅱm".to_string(),
            desc: "Supertonic (上主和音・短三和音): 期待・問い".to_string(),
        },
        3 => HarmonyInfo {
            roman: "Ⅲm".to_string(),
            desc: "Mediant (中和音・短三和音): 穏やか・中間".to_string(),
        },
        4 => HarmonyInfo {
            roman: "Ⅳ".to_string(),
            desc: "Subdominant (下属和音・長三和音): 広がり・始まり".to_string(),
        },
        5 => HarmonyInfo {
            roman: "Ⅴ".to_string(),
            desc: "Dominant (属和音・長三和音): 緊張・推進".to_string(),
        },
        6 => HarmonyInfo {
            roman: "Ⅵm".to_string(),
            desc: "Submediant (下中和音・短三和音): 儚さ・哀愁".to_string(),
        },
        7 => HarmonyInfo {
            roman: "Ⅶdim".to_string(),
            desc: "Leading Tone (導和音・減三和音): 不安・未解決".to_string(),
        },
        _ => HarmonyInfo {
            roman: "".to_string(),
            desc: "".to_string(),
        },
    }
}

/// 7thコードのローマ数字表記情報
#[wasm_bindgen]
pub fn roman_numeral_7th_harmony_info(degree: i32) -> HarmonyInfo {
    match degree {
        1 => HarmonyInfo {
            roman: "ⅠM7".to_string(),
            desc: "Tonic Seventh (主和音・長七の和音): 安心・落ち着き".to_string(),
        },
        2 => HarmonyInfo {
            roman: "Ⅱm7".to_string(),
            desc: "Supertonic Seventh (上主和音・短七の和音): 期待・問い".to_string(),
        },
        3 => HarmonyInfo {
            roman: "Ⅲm7".to_string(),
            desc: "Mediant Seventh (中和音・短七の和音): 穏やか・中間".to_string(),
        },
        4 => HarmonyInfo {
            roman: "ⅣM7".to_string(),
            desc: "Subdominant Seventh (下属和音・長七の和音): 広がり・始まり".to_string(),
        },
        5 => HarmonyInfo {
            roman: "Ⅴ7".to_string(),
            desc: "Dominant Seventh (属和音・属七の和音): 緊張・推進".to_string(),
        },
        6 => HarmonyInfo {
            roman: "Ⅵm7".to_string(),
            desc: "Submediant Seventh (下中和音・短七の和音): 儚さ・哀愁".to_string(),
        },
        7 => HarmonyInfo {
            roman: "Ⅶm7♭5".to_string(),
            desc: "Leading Tone Seventh (導和音・半減七の和音): 不安・未解決".to_string(),
        },
        _ => HarmonyInfo {
            roman: "".to_string(),
            desc: "".to_string(),
        },
    }
}

/// コードトーンのラベルを取得
#[wasm_bindgen]
pub fn get_chord_tone_label(scale: &str, chord: &str, target_pitch: &str) -> String {
    let interval = get_interval(chord, target_pitch);

    if interval == "1" {
        let chord_function = get_functional_harmony(scale, chord);
        match chord_function {
            1 => "Tonic Note",
            2 => "Supertonic Note",
            3 => "Mediant Note",
            4 => "Subdominant Note",
            5 => "Dominant Note",
            6 => "Submediant Note",
            7 => "Leading Tone Note",
            _ => "",
        }
        .to_string()
    } else {
        String::new()
    }
}

/// 進行分析結果
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressionInfo {
    degree: i32,
    roman: String,
    function: String,
    cadence: String,
    is_secondary_dominant: bool,
    secondary_target: String,
}

#[wasm_bindgen]
impl ProgressionInfo {
    #[wasm_bindgen(getter)]
    pub fn degree(&self) -> i32 {
        self.degree
    }

    #[wasm_bindgen(getter)]
    pub fn roman(&self) -> String {
        self.roman.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn function(&self) -> String {
        self.function.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn cadence(&self) -> String {
        self.cadence.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn is_secondary_dominant(&self) -> bool {
        self.is_secondary_dominant
    }

    #[wasm_bindgen(getter)]
    pub fn secondary_target(&self) -> String {
        self.secondary_target.clone()
    }
}

/// 複数コードの進行を分析
#[wasm_bindgen]
pub fn analyze_progression(scale: &str, chords: Vec<JsValue>) -> JsValue {
    let chord_strs: Vec<String> = chords
        .iter()
        .filter_map(|v| v.as_string())
        .collect();

    let results = analyze_progression_internal(scale, &chord_strs);
    serde_wasm_bindgen::to_value(&results).unwrap_or(JsValue::NULL)
}

/// 内部用の進行分析
pub fn analyze_progression_internal(scale: &str, chords: &[String]) -> Vec<ProgressionInfo> {
    use crate::harmony::cadence::{cadence_text, functional_area};

    let diatonic_map = create_diatonic_chord_map();
    let diatonic_chords = diatonic_map.get(scale).cloned().unwrap_or_default();

    let mut results = Vec::new();
    let mut prev_degree = 0;

    for (i, chord) in chords.iter().enumerate() {
        let degree = get_functional_harmony(scale, chord);
        let info = functional_harmony_info(degree);
        let func_area = if degree > 0 {
            functional_area(degree)
        } else {
            String::new()
        };

        let cadence = if i > 0 {
            cadence_text(prev_degree, degree)
        } else {
            String::new()
        };

        // セカンダリードミナント検出
        let (is_sec_dom, sec_target) = if degree == 0 {
            detect_secondary_dominant(chord, &diatonic_chords)
        } else {
            (false, String::new())
        };

        results.push(ProgressionInfo {
            degree,
            roman: info.roman,
            function: func_area,
            cadence,
            is_secondary_dominant: is_sec_dom,
            secondary_target: sec_target,
        });

        prev_degree = degree;
    }

    results
}

/// セカンダリードミナント検出
/// 非ダイアトニックのドミナント7thコードがダイアトニックコードのV7かを判定
fn detect_secondary_dominant(chord: &str, diatonic_chords: &[&str]) -> (bool, String) {
    use crate::chord::parser::{get_fret_offset, parse_chord_type};

    let (root, chord_type) = parse_chord_type(chord);
    if root.is_empty() {
        return (false, String::new());
    }

    // ドミナント7thの構造を持つかチェック
    let is_dom7 = chord_type == "7";
    if !is_dom7 {
        return (false, String::new());
    }

    // ルートの完全5度下（7半音下）がダイアトニックコードのルートに一致するか
    let root_offset = get_fret_offset(&root);
    let target_offset = (root_offset + 12 - 7) % 12; // 5度下

    // ダイアトニック度数名
    let degree_names = ["I", "II", "III", "IV", "V", "VI", "VII"];

    for (i, &diatonic) in diatonic_chords.iter().enumerate() {
        let diatonic_root = crate::chord::parser::get_root_note(diatonic);
        let diatonic_offset = get_fret_offset(&diatonic_root);
        if diatonic_offset == target_offset && i < degree_names.len() {
            let target = degree_names[i].to_lowercase();
            return (true, format!("V/{target}"));
        }
    }

    (false, String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_functional_harmony() {
        // CメジャースケールのC = Ⅰ
        assert_eq!(get_functional_harmony("C", "C"), 1);
        // CメジャースケールのG = Ⅴ
        assert_eq!(get_functional_harmony("C", "G"), 5);
    }

    #[test]
    fn test_functional_harmony_text() {
        assert_eq!(functional_harmony_text(1), "Ⅰ Tonic");
        assert_eq!(functional_harmony_text(5), "Ⅴ Dominant");
    }

    #[test]
    fn test_functional_harmony_info() {
        let info = functional_harmony_info(1);
        assert_eq!(info.roman, "Ⅰ");
        assert!(info.desc.contains("Tonic"));
    }

    #[test]
    fn test_analyze_progression_basic() {
        // キーCで C -> Am -> F -> G の進行
        let chords = vec![
            "C".to_string(),
            "Am".to_string(),
            "F".to_string(),
            "G".to_string(),
        ];
        let results = analyze_progression_internal("C", &chords);
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].degree, 1); // I
        assert_eq!(results[0].function, "T");
        assert_eq!(results[1].degree, 6); // vi
        assert_eq!(results[1].function, "T");
        assert_eq!(results[2].degree, 4); // IV
        assert_eq!(results[2].function, "S");
        assert_eq!(results[3].degree, 5); // V
        assert_eq!(results[3].function, "D");
        // G (V) が最後なので half cadence は前→現在で判定
        assert_eq!(results[3].cadence, "Half Cadence");
    }

    #[test]
    fn test_analyze_progression_cadence() {
        let chords = vec!["G".to_string(), "C".to_string()];
        let results = analyze_progression_internal("C", &chords);
        assert_eq!(results[1].cadence, "Perfect Cadence");
    }

    #[test]
    fn test_secondary_dominant_detection() {
        // キーCでA7はV/ii
        let chords = vec!["A7".to_string()];
        let results = analyze_progression_internal("C", &chords);
        assert_eq!(results[0].degree, 0); // non-diatonic
        assert!(results[0].is_secondary_dominant);
        assert_eq!(results[0].secondary_target, "V/ii");
    }
}
