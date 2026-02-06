//! コード構成音定義（楽器非依存）

use wasm_bindgen::prelude::*;

/// コードの構成音（インターバルと半音数のペア）
#[derive(Clone, Debug)]
pub struct ChordTone {
    pub interval: String,
    pub semitones: i32,
}

/// コード名からルート音を抽出
#[wasm_bindgen]
pub fn get_root_note(chord: &str) -> String {
    let mut root = String::new();
    let mut chars = chord.chars();

    if let Some(c) = chars.next() {
        if ('A'..='G').contains(&c) {
            root.push(c);
        } else {
            return String::new();
        }
    }

    if let Some(c) = chars.next() {
        if c == '♭' || c == '＃' {
            root.push(c);
        }
    }

    root
}

/// コード名からルート音とコードタイプを分離
/// "Cm7" -> ("C", "m7"), "F＃dim7" -> ("F＃", "dim7")
pub fn parse_chord_type(chord: &str) -> (String, String) {
    let root = get_root_note(chord);
    if root.is_empty() {
        return (String::new(), chord.to_string());
    }
    let chord_type = &chord[root.len()..];
    let normalized = match chord_type {
        "M7" | "△7" => "maj7",
        "M9" | "△9" => "maj9",
        "mM7" | "m(maj7)" | "-M7" => "m_maj7",
        "-" => "m",
        "-7" => "m7",
        "-9" => "m9",
        "+" => "aug",
        "+7" => "aug7",
        "o" => "dim",
        "o7" | "°7" => "dim7",
        "ø" | "ø7" | "m7♭5" => "m7b5",
        "sus" => "sus4",
        "7sus" => "7sus4",
        "-6" => "m6",
        "7♭9" => "7b9",
        "7＃9" => "7#9",
        other => other,
    };
    (root, normalized.to_string())
}

/// コードタイプ文字列から構成音配列を生成
pub fn get_chord_tones(chord_type: &str) -> Vec<ChordTone> {
    let intervals: Vec<(&str, i32)> = match chord_type {
        // トライアド
        "" | "maj" => vec![("1", 0), ("3", 4), ("5", 7)],
        "m" => vec![("1", 0), ("♭3", 3), ("5", 7)],
        "dim" => vec![("1", 0), ("♭3", 3), ("♭5", 6)],
        "aug" => vec![("1", 0), ("3", 4), ("＃5", 8)],
        "sus4" => vec![("1", 0), ("4", 5), ("5", 7)],
        "sus2" => vec![("1", 0), ("2", 2), ("5", 7)],

        // 7th
        "7" => vec![("1", 0), ("3", 4), ("5", 7), ("♭7", 10)],
        "m7" => vec![("1", 0), ("♭3", 3), ("5", 7), ("♭7", 10)],
        "maj7" | "M7" => vec![("1", 0), ("3", 4), ("5", 7), ("7", 11)],
        "m_maj7" | "mM7" => vec![("1", 0), ("♭3", 3), ("5", 7), ("7", 11)],
        "dim7" => vec![("1", 0), ("♭3", 3), ("♭5", 6), ("♭♭7", 9)],
        "m7b5" => vec![("1", 0), ("♭3", 3), ("♭5", 6), ("♭7", 10)],
        "aug7" => vec![("1", 0), ("3", 4), ("＃5", 8), ("♭7", 10)],
        "7sus4" => vec![("1", 0), ("4", 5), ("5", 7), ("♭7", 10)],

        // 6th
        "6" => vec![("1", 0), ("3", 4), ("5", 7), ("6", 9)],
        "m6" => vec![("1", 0), ("♭3", 3), ("5", 7), ("6", 9)],

        // 9th
        "9" => vec![("1", 0), ("3", 4), ("5", 7), ("♭7", 10), ("9", 14)],
        "m9" => vec![("1", 0), ("♭3", 3), ("5", 7), ("♭7", 10), ("9", 14)],
        "maj9" | "M9" => vec![("1", 0), ("3", 4), ("5", 7), ("7", 11), ("9", 14)],
        "add9" => vec![("1", 0), ("3", 4), ("5", 7), ("9", 14)],

        // Altered
        "7b9" => vec![("1", 0), ("3", 4), ("5", 7), ("♭7", 10), ("♭9", 13)],
        "7#9" => vec![("1", 0), ("3", 4), ("5", 7), ("♭7", 10), ("＃9", 15)],

        // フォールバック: メジャートライアド
        _ => vec![("1", 0), ("3", 4), ("5", 7)],
    };

    intervals
        .into_iter()
        .map(|(interval, semitones)| ChordTone {
            interval: interval.to_string(),
            semitones,
        })
        .collect()
}

/// 12音すべての ChordTone（ALL_KEYS用）
pub fn chromatic_chord_tones() -> Vec<ChordTone> {
    [
        ("1", 0), ("♭2", 1), ("2", 2), ("♭3", 3), ("3", 4), ("4", 5),
        ("♭5", 6), ("5", 7), ("＃5", 8), ("6", 9), ("♭7", 10), ("7", 11),
    ]
    .into_iter()
    .map(|(interval, semitones)| ChordTone { interval: interval.to_string(), semitones })
    .collect()
}

/// 白鍵7音の ChordTone（WHITE_KEYS用）
pub fn diatonic_chord_tones() -> Vec<ChordTone> {
    [
        ("1", 0), ("2", 2), ("3", 4), ("4", 5), ("5", 7), ("6", 9), ("7", 11),
    ]
    .into_iter()
    .map(|(interval, semitones)| ChordTone { interval: interval.to_string(), semitones })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root_note() {
        assert_eq!(get_root_note("C"), "C");
        assert_eq!(get_root_note("C＃maj7"), "C＃");
        assert_eq!(get_root_note("A♭m"), "A♭");
        assert_eq!(get_root_note("Dm7"), "D");
    }

    #[test]
    fn test_parse_chord_type() {
        assert_eq!(parse_chord_type("Cm7"), ("C".to_string(), "m7".to_string()));
        assert_eq!(parse_chord_type("F＃dim7"), ("F＃".to_string(), "dim7".to_string()));
        assert_eq!(parse_chord_type("B♭7sus4"), ("B♭".to_string(), "7sus4".to_string()));
        assert_eq!(parse_chord_type("C"), ("C".to_string(), "".to_string()));
        assert_eq!(parse_chord_type("CM7"), ("C".to_string(), "maj7".to_string()));
        assert_eq!(parse_chord_type("C+"), ("C".to_string(), "aug".to_string()));
        assert_eq!(parse_chord_type("Co7"), ("C".to_string(), "dim7".to_string()));
        assert_eq!(parse_chord_type("Csus"), ("C".to_string(), "sus4".to_string()));
    }

    #[test]
    fn test_get_chord_tones() {
        let tones = get_chord_tones("");
        assert_eq!(tones.len(), 3);
        assert_eq!(tones[0].interval, "1");
        assert_eq!(tones[1].interval, "3");
        assert_eq!(tones[1].semitones, 4);

        let tones = get_chord_tones("m7");
        assert_eq!(tones.len(), 4);
        assert_eq!(tones[1].semitones, 3); // ♭3

        let tones = get_chord_tones("dim7");
        assert_eq!(tones.len(), 4);
        assert_eq!(tones[3].semitones, 9); // ♭♭7
    }

    // ===== 仕様ベーステスト =====

    /// 22コードタイプ全ての半音値を検証
    #[test]
    fn test_spec_all_22_chord_tones() {
        fn semitones(ct: &str) -> Vec<i32> {
            get_chord_tones(ct).iter().map(|t| t.semitones).collect()
        }
        // トライアド
        assert_eq!(semitones(""), vec![0, 4, 7]);
        assert_eq!(semitones("m"), vec![0, 3, 7]);
        assert_eq!(semitones("dim"), vec![0, 3, 6]);
        assert_eq!(semitones("aug"), vec![0, 4, 8]);
        assert_eq!(semitones("sus4"), vec![0, 5, 7]);
        assert_eq!(semitones("sus2"), vec![0, 2, 7]);
        // 7th
        assert_eq!(semitones("7"), vec![0, 4, 7, 10]);
        assert_eq!(semitones("m7"), vec![0, 3, 7, 10]);
        assert_eq!(semitones("maj7"), vec![0, 4, 7, 11]);
        assert_eq!(semitones("m_maj7"), vec![0, 3, 7, 11]);
        assert_eq!(semitones("dim7"), vec![0, 3, 6, 9]);
        assert_eq!(semitones("m7b5"), vec![0, 3, 6, 10]);
        assert_eq!(semitones("aug7"), vec![0, 4, 8, 10]);
        assert_eq!(semitones("7sus4"), vec![0, 5, 7, 10]);
        // 6th
        assert_eq!(semitones("6"), vec![0, 4, 7, 9]);
        assert_eq!(semitones("m6"), vec![0, 3, 7, 9]);
        // 9th
        assert_eq!(semitones("9"), vec![0, 4, 7, 10, 14]);
        assert_eq!(semitones("m9"), vec![0, 3, 7, 10, 14]);
        assert_eq!(semitones("maj9"), vec![0, 4, 7, 11, 14]);
        assert_eq!(semitones("add9"), vec![0, 4, 7, 14]);
        // Altered
        assert_eq!(semitones("7b9"), vec![0, 4, 7, 10, 13]);
        assert_eq!(semitones("7#9"), vec![0, 4, 7, 10, 15]);
    }

    /// インターバル文字列の検証
    #[test]
    fn test_spec_chord_tone_intervals() {
        fn intervals(ct: &str) -> Vec<String> {
            get_chord_tones(ct).iter().map(|t| t.interval.clone()).collect()
        }
        assert_eq!(intervals("m7"), vec!["1", "♭3", "5", "♭7"]);
        assert_eq!(intervals("dim7"), vec!["1", "♭3", "♭5", "♭♭7"]);
        assert_eq!(intervals("aug"), vec!["1", "3", "＃5"]);
        assert_eq!(intervals("7#9"), vec!["1", "3", "5", "♭7", "＃9"]);
        assert_eq!(intervals("m_maj7"), vec!["1", "♭3", "5", "7"]);
        assert_eq!(intervals("sus4"), vec!["1", "4", "5"]);
        assert_eq!(intervals("m7b5"), vec!["1", "♭3", "♭5", "♭7"]);
        assert_eq!(intervals("7b9"), vec!["1", "3", "5", "♭7", "♭9"]);
    }

    /// エイリアス正規化（sid-noteで使う全表記）
    #[test]
    fn test_spec_parse_chord_aliases() {
        fn ct(chord: &str) -> String {
            parse_chord_type(chord).1
        }
        // maj7系
        assert_eq!(ct("CM7"), "maj7");
        assert_eq!(ct("C△7"), "maj7");
        // minor系
        assert_eq!(ct("C-"), "m");
        assert_eq!(ct("C-7"), "m7");
        // aug系
        assert_eq!(ct("C+"), "aug");
        assert_eq!(ct("C+7"), "aug7");
        // dim系
        assert_eq!(ct("Co"), "dim");
        assert_eq!(ct("Co7"), "dim7");
        assert_eq!(ct("C°7"), "dim7");
        // half-dim
        assert_eq!(ct("Cø"), "m7b5");
        assert_eq!(ct("Cø7"), "m7b5");
        assert_eq!(ct("Cm7♭5"), "m7b5");
        // sus
        assert_eq!(ct("Csus"), "sus4");
        assert_eq!(ct("C7sus"), "7sus4");
        // m6
        assert_eq!(ct("C-6"), "m6");
        // m_maj7
        assert_eq!(ct("CmM7"), "m_maj7");
        assert_eq!(ct("Cm(maj7)"), "m_maj7");
        assert_eq!(ct("C-M7"), "m_maj7");
        // 9th
        assert_eq!(ct("CM9"), "maj9");
        assert_eq!(ct("C△9"), "maj9");
        assert_eq!(ct("C-9"), "m9");
        // altered
        assert_eq!(ct("C7♭9"), "7b9");
        assert_eq!(ct("C7＃9"), "7#9");
    }

    /// ＃/♭ルート音のパース
    #[test]
    fn test_spec_parse_sharp_flat_roots() {
        assert_eq!(parse_chord_type("F＃m7"), ("F＃".to_string(), "m7".to_string()));
        assert_eq!(parse_chord_type("B♭7"), ("B♭".to_string(), "7".to_string()));
        assert_eq!(parse_chord_type("A♭maj7"), ("A♭".to_string(), "maj7".to_string()));
        assert_eq!(parse_chord_type("D♭m"), ("D♭".to_string(), "m".to_string()));
        assert_eq!(parse_chord_type("G＃dim"), ("G＃".to_string(), "dim".to_string()));
        assert_eq!(parse_chord_type("E♭aug"), ("E♭".to_string(), "aug".to_string()));
        assert_eq!(parse_chord_type("C＃m7b5"), ("C＃".to_string(), "m7b5".to_string()));
    }

    /// ルート抽出の境界
    #[test]
    fn test_spec_get_root_note_edge_cases() {
        assert_eq!(get_root_note(""), "");
        assert_eq!(get_root_note("m7"), "");     // ルートなし
        assert_eq!(get_root_note("7"), "");       // 数字始まり
        assert_eq!(get_root_note("C"), "C");
        assert_eq!(get_root_note("C＃"), "C＃");
        assert_eq!(get_root_note("D♭"), "D♭");
    }

    /// 未知タイプ→メジャートライアドにフォールバック
    #[test]
    fn test_spec_unknown_chord_type_fallback() {
        let tones = get_chord_tones("xyz");
        assert_eq!(tones.len(), 3);
        let semis: Vec<i32> = tones.iter().map(|t| t.semitones).collect();
        assert_eq!(semis, vec![0, 4, 7]);
    }
}
