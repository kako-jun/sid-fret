//! フレットボード計算（楽器固有）

use crate::core::chord_type::{get_chord_tones, get_root_note, parse_chord_type, ChordTone};
use crate::core::pitch::{pitch_map_for_root, fret_offset};
use crate::instrument::tuning::Tuning;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// フレットボード上のポジション
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    string: i32,
    fret: i32,
    pitch: String,
    interval: String,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(getter)]
    pub fn string(&self) -> i32 {
        self.string
    }

    #[wasm_bindgen(getter)]
    pub fn fret(&self) -> i32 {
        self.fret
    }

    #[wasm_bindgen(getter)]
    pub fn pitch(&self) -> String {
        self.pitch.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn interval(&self) -> String {
        self.interval.clone()
    }
}

/// ピッチ情報付きフレット（内部用）
#[derive(Clone, Debug)]
struct FretWithPitch {
    interval: String,
    fret: i32,
    pitch: String,
}

/// 構成音からピッチ情報付きフレット配列を生成
fn get_pitches(root: &str, tones: &[ChordTone], offset: i32) -> Vec<FretWithPitch> {
    let pitch_map = pitch_map_for_root(root);

    let root_index = pitch_map
        .iter()
        .position(|pitch_text| pitch_text.split('/').any(|p| p == root))
        .unwrap_or(0);

    tones
        .iter()
        .map(|tone| {
            let pitch_index = (root_index + tone.semitones as usize) % 12;
            FretWithPitch {
                interval: tone.interval.clone(),
                fret: tone.semitones + offset,
                pitch: pitch_map[pitch_index].clone(),
            }
        })
        .collect()
}

/// フレットからポジションへの変換（チューニング対応）
fn convert_to_positions(frets: &[FretWithPitch], tuning: &Tuning) -> Vec<Position> {
    let mut positions = Vec::new();
    let num_strings = tuning.strings.len();

    for fwp in frets {
        for (i, string_def) in tuning.strings.iter().enumerate() {
            let string_num = (num_strings - i) as i32;
            let min_fret = string_def.offset;
            let max_fret = string_def.offset + tuning.max_fret;

            if fwp.fret >= min_fret && fwp.fret <= max_fret {
                positions.push(Position {
                    string: string_num,
                    fret: fwp.fret - string_def.offset,
                    pitch: fwp.pitch.clone(),
                    interval: fwp.interval.clone(),
                });
            }
        }
    }

    positions
}

/// コード名とチューニングからフレットボードポジションを計算
pub fn chord_positions(chord: &str, tuning: &Tuning) -> Vec<Position> {
    let is_all_keys = chord == "ALL_KEYS";
    let is_white_keys = chord == "WHITE_KEYS";
    let is_power_chord = chord.ends_with('5') && !chord.contains("♭5") && !chord.contains("-5");
    let is_octave_unison = chord.contains('8')
        && !chord
            .find('8')
            .and_then(|pos| chord.chars().nth(pos + 1))
            .is_some_and(|c| c.is_numeric());

    let (chord_tones, use_root) = if is_all_keys {
        let tones = vec![
            ChordTone { interval: "1".to_string(), semitones: 0 },
            ChordTone { interval: "♭2".to_string(), semitones: 1 },
            ChordTone { interval: "2".to_string(), semitones: 2 },
            ChordTone { interval: "♭3".to_string(), semitones: 3 },
            ChordTone { interval: "3".to_string(), semitones: 4 },
            ChordTone { interval: "4".to_string(), semitones: 5 },
            ChordTone { interval: "♭5".to_string(), semitones: 6 },
            ChordTone { interval: "5".to_string(), semitones: 7 },
            ChordTone { interval: "＃5".to_string(), semitones: 8 },
            ChordTone { interval: "6".to_string(), semitones: 9 },
            ChordTone { interval: "♭7".to_string(), semitones: 10 },
            ChordTone { interval: "7".to_string(), semitones: 11 },
        ];
        (tones, "C".to_string())
    } else if is_white_keys {
        let tones = vec![
            ChordTone { interval: "1".to_string(), semitones: 0 },
            ChordTone { interval: "2".to_string(), semitones: 2 },
            ChordTone { interval: "3".to_string(), semitones: 4 },
            ChordTone { interval: "4".to_string(), semitones: 5 },
            ChordTone { interval: "5".to_string(), semitones: 7 },
            ChordTone { interval: "6".to_string(), semitones: 9 },
            ChordTone { interval: "7".to_string(), semitones: 11 },
        ];
        (tones, "C".to_string())
    } else if is_power_chord {
        let tones = vec![
            ChordTone { interval: "1".to_string(), semitones: 0 },
            ChordTone { interval: "5".to_string(), semitones: 7 },
        ];
        (tones, get_root_note(chord))
    } else if is_octave_unison {
        let tones = vec![
            ChordTone { interval: "1".to_string(), semitones: 0 },
            ChordTone { interval: "8".to_string(), semitones: 12 },
        ];
        (tones, get_root_note(chord))
    } else {
        let (root, chord_type) = parse_chord_type(chord);
        let tones = get_chord_tones(&chord_type);
        (tones, root)
    };

    let offset = fret_offset(&use_root);
    let frets_with_pitch = get_pitches(&use_root, &chord_tones, offset - 12);

    let max_absolute_fret = tuning
        .strings
        .iter()
        .map(|s| s.offset + tuning.max_fret)
        .max()
        .unwrap_or(39);

    let mut current_octave = 0;
    let octave_frets: Vec<FretWithPitch> = frets_with_pitch
        .iter()
        .flat_map(|fwp| {
            let pitch_name = fwp.pitch.replace(char::is_numeric, "");

            if pitch_name.starts_with('C') || pitch_name.starts_with('D') {
                current_octave = 1;
            }

            (0..4)
                .map(|oct| FretWithPitch {
                    fret: fwp.fret + oct * 12,
                    interval: fwp.interval.clone(),
                    pitch: format!("{}{}", pitch_name, current_octave + oct),
                })
                .filter(|f| {
                    let min_fret = tuning.strings.iter().map(|s| s.offset).min().unwrap_or(0);
                    f.fret >= min_fret && f.fret <= max_absolute_fret
                })
                .collect::<Vec<_>>()
        })
        .collect();

    convert_to_positions(&octave_frets, tuning)
}

/// インターバル記号を取得
pub fn interval_for_pitch(chord: &str, target_pitch: &str) -> String {
    let target_name = target_pitch.replace(char::is_numeric, "");
    let root = get_root_note(chord);
    let pitches = pitch_map_for_root(&root);

    let index = pitches
        .iter()
        .position(|pitch| {
            pitch
                .split('/')
                .any(|p| p.replace(char::is_numeric, "") == target_name)
        })
        .unwrap_or(0);

    let interval_map = [
        "1", "♭2", "2", "♭3", "3", "4", "＃4/♭5", "5", "＃5", "6", "♭7", "7",
    ];

    interval_map[index].to_string()
}

/// WASM: コード名からポジション配列を取得
#[wasm_bindgen]
pub fn get_chord_positions(chord: &str) -> JsValue {
    let positions = chord_positions(chord, &Tuning::bass_4());
    serde_wasm_bindgen::to_value(&positions).unwrap()
}

/// WASM: チューニング指定付きコードポジション取得
#[wasm_bindgen]
pub fn get_chord_positions_with_tuning(chord: &str, tuning_name: &str) -> JsValue {
    let tuning = Tuning::from_name(tuning_name).unwrap_or_else(Tuning::bass_4);
    let positions = chord_positions(chord, &tuning);
    serde_wasm_bindgen::to_value(&positions).unwrap()
}

/// WASM: チューニング情報を返す
#[wasm_bindgen]
pub fn get_tuning_info(tuning_name: &str) -> JsValue {
    let tuning = Tuning::from_name(tuning_name).unwrap_or_else(Tuning::bass_4);
    serde_wasm_bindgen::to_value(&tuning).unwrap_or(JsValue::NULL)
}

/// WASM: 利用可能なチューニングプリセット一覧を返す
#[wasm_bindgen]
pub fn list_tunings() -> JsValue {
    let names = vec!["bass_4", "bass_5", "bass_6", "bass_drop_d"];
    serde_wasm_bindgen::to_value(&names).unwrap_or(JsValue::NULL)
}

/// WASM: インターバル記号を取得
#[wasm_bindgen]
pub fn get_interval(chord: &str, target_pitch: &str) -> String {
    interval_for_pitch(chord, target_pitch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_positions() {
        let positions = chord_positions("C", &Tuning::bass_4());
        assert!(!positions.is_empty());
    }

    #[test]
    fn test_get_interval() {
        assert_eq!(interval_for_pitch("C", "C2"), "1");
        assert_eq!(interval_for_pitch("C", "E2"), "3");
        assert_eq!(interval_for_pitch("C", "G2"), "5");
    }

    #[test]
    fn test_chord_positions_with_tuning() {
        let pos_4 = chord_positions("C", &Tuning::bass_4());
        let pos_5 = chord_positions("C", &Tuning::bass_5());
        assert!(pos_5.len() >= pos_4.len());
    }
}
