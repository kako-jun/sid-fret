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
}
