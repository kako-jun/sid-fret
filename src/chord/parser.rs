use wasm_bindgen::prelude::*;

/// コード名からルート音を抽出（chordUtil.ts の getRootNote() に相当）
#[wasm_bindgen]
pub fn get_root_note(chord: &str) -> String {
    let mut root = String::new();
    let mut chars = chord.chars();

    // 最初の文字（A-G）
    if let Some(c) = chars.next() {
        if ('A'..='G').contains(&c) {
            root.push(c);
        } else {
            return String::new();
        }
    }

    // 2文字目がアクシデンタルか確認
    if let Some(c) = chars.next() {
        if c == '♭' || c == '＃' {
            root.push(c);
        }
    }

    root
}

/// フレットオフセットを取得（chordUtil.ts の getFretOffset() に相当）
/// E=0基準（ベース4弦開放）
#[wasm_bindgen]
pub fn get_fret_offset(root: &str) -> i32 {
    match root {
        "E" => 0,
        "E＃" => 1,
        "F♭" => 0,
        "F" => 1,
        "F＃" => 2,
        "G♭" => 2,
        "G" => 3,
        "G＃" => 4,
        "A♭" => 4,
        "A" => 5,
        "A＃" => 6,
        "B♭" => 6,
        "B" => 7,
        "B＃" => 8,
        "C♭" => 7,
        "C" => 8,
        "C＃" => 9,
        "D♭" => 9,
        "D" => 10,
        "D＃" => 11,
        "E♭" => 11,
        _ => 0,
    }
}

/// インターバルとフレット番号のペア
#[derive(Clone, Debug)]
pub struct Fret {
    pub interval: String,
    pub fret: i32,
}

/// コードタイプ文字列からフレット配列を生成
/// chord_type: "", "m", "7", "m7", "maj7", "dim", "aug", "sus4", "6", "m6",
///             "9", "m9", "maj9", "add9", "sus2", "dim7", "m7b5",
///             "aug7", "7sus4", "m_maj7", "7b9", "7#9"
pub fn get_frets(chord_type: &str) -> Vec<Fret> {
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
        .map(|(interval, fret)| Fret {
            interval: interval.to_string(),
            fret,
        })
        .collect()
}

/// コード名からルート音とコードタイプを分離
/// "Cm7" -> ("C", "m7"), "F＃dim7" -> ("F＃", "dim7"), "B♭7sus4" -> ("B♭", "7sus4")
pub fn parse_chord_type(chord: &str) -> (String, String) {
    let root = get_root_note(chord);
    if root.is_empty() {
        return (String::new(), chord.to_string());
    }
    let chord_type = &chord[root.len()..];
    // 正規化: 一般的な表記をマッチ用に変換
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

/// ピッチマップ（全12キー）
pub fn get_pitch_map(root: &str) -> Vec<String> {
    let map: Vec<Vec<&str>> = vec![
        vec!["C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B"],
        vec!["C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B", "C"],
        vec!["D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭"],
        vec!["D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭", "D"],
        vec!["E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭"],
        vec!["F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭", "E"],
        vec!["F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭", "E", "F"],
        vec!["G", "G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭"],
        vec!["G＃/A♭", "A", "A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G"],
        vec!["A", "A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭"],
        vec!["A＃/B♭", "B", "C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A"],
        vec!["B", "C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭"],
    ];

    // ルート音に対応するマップを返す
    let roots = ["C", "C＃/D♭", "D", "D＃/E♭", "E", "F", "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B"];

    for (i, &r) in roots.iter().enumerate() {
        if r.split('/').any(|s| s == root) {
            return map[i].iter().map(|s| s.to_string()).collect();
        }
    }

    // デフォルトはCのマップ
    map[0].iter().map(|s| s.to_string()).collect()
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
    fn test_get_fret_offset() {
        assert_eq!(get_fret_offset("C"), 8);
        assert_eq!(get_fret_offset("E"), 0);
        assert_eq!(get_fret_offset("G"), 3);
    }

    #[test]
    fn test_get_frets() {
        // Major triad
        let frets = get_frets("");
        assert_eq!(frets.len(), 3);
        assert_eq!(frets[0].interval, "1");
        assert_eq!(frets[1].interval, "3");
        assert_eq!(frets[2].interval, "5");

        // Minor triad
        let frets = get_frets("m");
        assert_eq!(frets.len(), 3);
        assert_eq!(frets[1].fret, 3); // ♭3

        // dim7 (4 notes)
        let frets = get_frets("dim7");
        assert_eq!(frets.len(), 4);
        assert_eq!(frets[3].fret, 9); // ♭♭7

        // 9th (5 notes)
        let frets = get_frets("9");
        assert_eq!(frets.len(), 5);
        assert_eq!(frets[4].fret, 14); // 9

        // aug triad
        let frets = get_frets("aug");
        assert_eq!(frets.len(), 3);
        assert_eq!(frets[2].fret, 8); // ＃5

        // sus2
        let frets = get_frets("sus2");
        assert_eq!(frets.len(), 3);
        assert_eq!(frets[1].fret, 2); // 2

        // m7b5 (half-diminished)
        let frets = get_frets("m7b5");
        assert_eq!(frets.len(), 4);
        assert_eq!(frets[2].fret, 6); // ♭5
        assert_eq!(frets[3].fret, 10); // ♭7
    }

    #[test]
    fn test_parse_chord_type() {
        assert_eq!(parse_chord_type("Cm7"), ("C".to_string(), "m7".to_string()));
        assert_eq!(parse_chord_type("F＃dim7"), ("F＃".to_string(), "dim7".to_string()));
        assert_eq!(parse_chord_type("B♭7sus4"), ("B♭".to_string(), "7sus4".to_string()));
        assert_eq!(parse_chord_type("C"), ("C".to_string(), "".to_string()));
        // 別表記の正規化
        assert_eq!(parse_chord_type("CM7"), ("C".to_string(), "maj7".to_string()));
        assert_eq!(parse_chord_type("C+"), ("C".to_string(), "aug".to_string()));
        assert_eq!(parse_chord_type("Co7"), ("C".to_string(), "dim7".to_string()));
        assert_eq!(parse_chord_type("Csus"), ("C".to_string(), "sus4".to_string()));
    }
}
