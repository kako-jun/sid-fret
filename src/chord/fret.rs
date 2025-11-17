use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

/// フレット情報（インターバル名と半音数のペア）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fret {
    pub interval: String,
    pub semitones: i32,
}

/// ベースの弦情報（4弦ベース）
pub struct BassString {
    pub name: &'static str,
    pub min_fret: i32,
    pub max_fret: i32,
}

/// 4弦ベースの標準チューニング（E1, A1, D2, G2）
pub const BASS_STRINGS: [BassString; 4] = [
    BassString { name: "G", min_fret: 15, max_fret: 39 }, // 1弦
    BassString { name: "D", min_fret: 10, max_fret: 34 }, // 2弦
    BassString { name: "A", min_fret: 5, max_fret: 29 },  // 3弦
    BassString { name: "E", min_fret: 0, max_fret: 24 },  // 4弦
];

/// ルート音から半音オフセットを取得
#[wasm_bindgen]
pub fn get_fret_offset(root: &str) -> i32 {
    match root {
        "C" => 0,
        "C＃" | "D♭" => 1,
        "D" => 2,
        "D＃" | "E♭" => 3,
        "E" => 4,
        "F" => 5,
        "F＃" | "G♭" => 6,
        "G" => 7,
        "G＃" | "A♭" => 8,
        "A" => 9,
        "A＃" | "B♭" => 10,
        "B" => 11,
        _ => 0,
    }
}

/// コード構成音からフレット配列を生成
pub fn get_frets(
    has_minor_3rd: bool,
    has_sus4: bool,
    has_dim_5th: bool,
    has_maj_7th: bool,
    has_min_7th: bool,
    has_aug_7th: bool,
) -> Vec<Fret> {
    let mut frets = vec![Fret {
        interval: "1".to_string(),
        semitones: 0,
    }];

    // 3度
    if has_minor_3rd {
        frets.push(Fret {
            interval: "♭3".to_string(),
            semitones: 3,
        });
    } else if has_sus4 {
        frets.push(Fret {
            interval: "4".to_string(),
            semitones: 5,
        });
    } else {
        frets.push(Fret {
            interval: "3".to_string(),
            semitones: 4,
        });
    }

    // 5度
    if has_dim_5th {
        frets.push(Fret {
            interval: "♭5".to_string(),
            semitones: 6,
        });
    } else {
        frets.push(Fret {
            interval: "5".to_string(),
            semitones: 7,
        });
    }

    // 7度
    if has_maj_7th {
        frets.push(Fret {
            interval: "7".to_string(),
            semitones: 11,
        });
    } else if has_min_7th {
        frets.push(Fret {
            interval: "♭7".to_string(),
            semitones: 10,
        });
    } else if has_aug_7th {
        frets.push(Fret {
            interval: "♯7".to_string(),
            semitones: 12,
        });
    }

    frets
}

/// フレット配列をベースの4弦ポジションに変換
pub fn convert_frets_to_positions(frets: &[Fret], offset: i32) -> Vec<Vec<i32>> {
    let mut all_positions = Vec::new();

    // 各フレットに対して
    for fret in frets {
        let base_fret = fret.semitones;

        // 4オクターブ分展開（0, 12, 24, 36半音）
        let octave_frets: Vec<i32> = (0..4)
            .map(|octave| base_fret + offset + octave * 12)
            .filter(|&f| f >= 0 && f < 40)
            .collect();

        // 各弦でのポジションを計算
        let mut string_positions = Vec::new();
        for bass_string in &BASS_STRINGS {
            for &fret_val in &octave_frets {
                if fret_val >= bass_string.min_fret && fret_val <= bass_string.max_fret {
                    // 25フレット範囲に正規化
                    let normalized = fret_val % 25;
                    string_positions.push(normalized);
                }
            }
        }

        string_positions.sort();
        string_positions.dedup();
        all_positions.push(string_positions);
    }

    all_positions
}

/// ルート音とフレット配列から音程名を取得
pub fn get_pitches(root: &str, frets: &[Fret], offset: i32) -> Vec<Vec<String>> {
    let pitch_map = get_pitch_map(root);

    frets
        .iter()
        .map(|fret| {
            let idx = ((fret.semitones + offset) % 12) as usize;
            vec![pitch_map.get(idx).unwrap_or(&"C").to_string()]
        })
        .collect()
}

/// ルート音から半音階のピッチマップを取得
fn get_pitch_map(root: &str) -> Vec<&'static str> {
    let chromatic = vec![
        "C", "C＃/D♭", "D", "D＃/E♭", "E", "F",
        "F＃/G♭", "G", "G＃/A♭", "A", "A＃/B♭", "B",
    ];

    let offset = get_fret_offset(root) as usize;
    let mut rotated = chromatic[offset..].to_vec();
    rotated.extend_from_slice(&chromatic[..offset]);
    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fret_offset() {
        assert_eq!(get_fret_offset("C"), 0);
        assert_eq!(get_fret_offset("C＃"), 1);
        assert_eq!(get_fret_offset("D♭"), 1);
        assert_eq!(get_fret_offset("D"), 2);
        assert_eq!(get_fret_offset("G"), 7);
    }

    #[test]
    fn test_get_frets_major() {
        let frets = get_frets(false, false, false, false, false, false);
        assert_eq!(frets.len(), 3);
        assert_eq!(frets[0].semitones, 0); // ルート
        assert_eq!(frets[1].semitones, 4); // 長3度
        assert_eq!(frets[2].semitones, 7); // 完全5度
    }

    #[test]
    fn test_get_frets_minor() {
        let frets = get_frets(true, false, false, false, false, false);
        assert_eq!(frets.len(), 3);
        assert_eq!(frets[1].semitones, 3); // 短3度
    }

    #[test]
    fn test_get_frets_seventh() {
        let frets = get_frets(false, false, false, false, true, false);
        assert_eq!(frets.len(), 4);
        assert_eq!(frets[3].semitones, 10); // 短7度
    }

    #[test]
    fn test_convert_frets_to_positions() {
        let frets = vec![
            Fret { interval: "1".to_string(), semitones: 0 },
            Fret { interval: "3".to_string(), semitones: 4 },
        ];
        let positions = convert_frets_to_positions(&frets, 0);
        assert_eq!(positions.len(), 2);
        assert!(positions[0].len() > 0);
    }

    #[test]
    fn test_get_pitch_map() {
        let map = get_pitch_map("C");
        assert_eq!(map[0], "C");
        assert_eq!(map[7], "G");

        let map_g = get_pitch_map("G");
        assert_eq!(map_g[0], "G");
    }
}
