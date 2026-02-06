use super::parser::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// ギターの弦とフレットのポジション
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

/// フレットとピッチ情報
#[derive(Clone, Debug)]
struct FretWithPitch {
    interval: String,
    fret: i32,
    pitch: String,
}

/// 弦の定義
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StringDef {
    pub open_note: String,
    pub offset: i32,
}

/// チューニング定義
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tuning {
    pub name: String,
    pub strings: Vec<StringDef>,
    pub max_fret: i32,
}

impl Tuning {
    /// 4弦スタンダード (E-A-D-G)
    pub fn bass_4() -> Self {
        Tuning {
            name: "bass_4".to_string(),
            strings: vec![
                StringDef { open_note: "E".to_string(), offset: 0 },
                StringDef { open_note: "A".to_string(), offset: 5 },
                StringDef { open_note: "D".to_string(), offset: 10 },
                StringDef { open_note: "G".to_string(), offset: 15 },
            ],
            max_fret: 24,
        }
    }

    /// 5弦スタンダード (B-E-A-D-G)
    pub fn bass_5() -> Self {
        Tuning {
            name: "bass_5".to_string(),
            strings: vec![
                StringDef { open_note: "B".to_string(), offset: -5 },
                StringDef { open_note: "E".to_string(), offset: 0 },
                StringDef { open_note: "A".to_string(), offset: 5 },
                StringDef { open_note: "D".to_string(), offset: 10 },
                StringDef { open_note: "G".to_string(), offset: 15 },
            ],
            max_fret: 24,
        }
    }

    /// 6弦スタンダード (B-E-A-D-G-C)
    pub fn bass_6() -> Self {
        Tuning {
            name: "bass_6".to_string(),
            strings: vec![
                StringDef { open_note: "B".to_string(), offset: -5 },
                StringDef { open_note: "E".to_string(), offset: 0 },
                StringDef { open_note: "A".to_string(), offset: 5 },
                StringDef { open_note: "D".to_string(), offset: 10 },
                StringDef { open_note: "G".to_string(), offset: 15 },
                StringDef { open_note: "C".to_string(), offset: 20 },
            ],
            max_fret: 24,
        }
    }

    /// ドロップD (D-A-D-G)
    pub fn bass_drop_d() -> Self {
        Tuning {
            name: "bass_drop_d".to_string(),
            strings: vec![
                StringDef { open_note: "D".to_string(), offset: -2 },
                StringDef { open_note: "A".to_string(), offset: 5 },
                StringDef { open_note: "D".to_string(), offset: 10 },
                StringDef { open_note: "G".to_string(), offset: 15 },
            ],
            max_fret: 24,
        }
    }

    /// 名前からプリセットを取得
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "bass_4" => Some(Self::bass_4()),
            "bass_5" => Some(Self::bass_5()),
            "bass_6" => Some(Self::bass_6()),
            "bass_drop_d" => Some(Self::bass_drop_d()),
            _ => None,
        }
    }
}

/// チューニング対応のフレット→ポジション変換
fn convert_frets_to_positions_with_tuning(
    frets: &[FretWithPitch],
    tuning: &Tuning,
) -> Vec<Position> {
    let mut positions = Vec::new();
    let num_strings = tuning.strings.len();

    for fret_with_pitch in frets {
        let fret = fret_with_pitch.fret;
        let pitch = &fret_with_pitch.pitch;
        let interval = &fret_with_pitch.interval;

        // 弦番号は最高音弦=1（既存互換）
        for (i, string_def) in tuning.strings.iter().enumerate() {
            let string_num = (num_strings - i) as i32;
            let min_fret = string_def.offset;
            let max_fret = string_def.offset + tuning.max_fret;

            if fret >= min_fret && fret <= max_fret {
                positions.push(Position {
                    string: string_num,
                    fret: fret - string_def.offset,
                    pitch: pitch.clone(),
                    interval: interval.clone(),
                });
            }
        }
    }

    positions
}

/// getPitches()相当の関数
fn get_pitches(root: &str, frets: &[Fret], offset: i32) -> Vec<FretWithPitch> {
    let pitch_map = get_pitch_map(root);

    // ルート音のインデックスを見つける
    let root_index = pitch_map
        .iter()
        .position(|pitch_text| pitch_text.split('/').any(|p| p == root))
        .unwrap_or(0);

    frets
        .iter()
        .map(|fret| {
            let pitch_index = (root_index + fret.fret as usize) % 12;
            FretWithPitch {
                interval: fret.interval.clone(),
                fret: fret.fret + offset,
                pitch: pitch_map[pitch_index].clone(),
            }
        })
        .collect()
}

/// convertFretsToPositions()相当の関数（4弦デフォルト）
fn convert_frets_to_positions(frets: &[FretWithPitch]) -> Vec<Position> {
    convert_frets_to_positions_with_tuning(frets, &Tuning::bass_4())
}

/// コード名からポジション配列を取得（chordUtil.ts の getChordPositions() に相当）
#[wasm_bindgen]
pub fn get_chord_positions(chord: &str) -> JsValue {
    let positions = get_chord_positions_internal(chord);
    serde_wasm_bindgen::to_value(&positions).unwrap()
}

/// チューニング指定付きコードポジション取得
#[wasm_bindgen]
pub fn get_chord_positions_with_tuning(chord: &str, tuning_name: &str) -> JsValue {
    let tuning = Tuning::from_name(tuning_name).unwrap_or_else(Tuning::bass_4);
    let positions = get_chord_positions_with_tuning_internal(chord, &tuning);
    serde_wasm_bindgen::to_value(&positions).unwrap()
}

/// 内部用: チューニング指定付きポジション取得
fn get_chord_positions_with_tuning_internal(chord: &str, tuning: &Tuning) -> Vec<Position> {
    // 特別なコード判定
    let is_all_keys = chord == "ALL_KEYS";
    let is_white_keys = chord == "WHITE_KEYS";
    let is_power_chord = chord.ends_with('5') && !chord.contains("♭5") && !chord.contains("-5");
    let is_octave_unison = chord.contains('8')
        && !chord
            .find('8')
            .and_then(|pos| chord.chars().nth(pos + 1))
            .is_some_and(|c| c.is_numeric());

    let (frets, use_root) = if is_all_keys {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "♭2".to_string(), fret: 1 },
            Fret { interval: "2".to_string(), fret: 2 },
            Fret { interval: "♭3".to_string(), fret: 3 },
            Fret { interval: "3".to_string(), fret: 4 },
            Fret { interval: "4".to_string(), fret: 5 },
            Fret { interval: "♭5".to_string(), fret: 6 },
            Fret { interval: "5".to_string(), fret: 7 },
            Fret { interval: "＃5".to_string(), fret: 8 },
            Fret { interval: "6".to_string(), fret: 9 },
            Fret { interval: "♭7".to_string(), fret: 10 },
            Fret { interval: "7".to_string(), fret: 11 },
        ];
        (frets, "C".to_string())
    } else if is_white_keys {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "2".to_string(), fret: 2 },
            Fret { interval: "3".to_string(), fret: 4 },
            Fret { interval: "4".to_string(), fret: 5 },
            Fret { interval: "5".to_string(), fret: 7 },
            Fret { interval: "6".to_string(), fret: 9 },
            Fret { interval: "7".to_string(), fret: 11 },
        ];
        (frets, "C".to_string())
    } else if is_power_chord {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "5".to_string(), fret: 7 },
        ];
        (frets, get_root_note(chord))
    } else if is_octave_unison {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "8".to_string(), fret: 12 },
        ];
        (frets, get_root_note(chord))
    } else {
        let (root, chord_type) = parse_chord_type(chord);
        let frets = get_frets(&chord_type);
        (frets, root)
    };

    let offset = get_fret_offset(&use_root);
    let frets_with_pitch = get_pitches(&use_root, &frets, offset - 12);

    // 最大フレット範囲を計算
    let max_absolute_fret = tuning.strings.iter()
        .map(|s| s.offset + tuning.max_fret)
        .max()
        .unwrap_or(39);

    let mut current_octave = 0;
    let octave_frets: Vec<FretWithPitch> = frets_with_pitch
        .iter()
        .flat_map(|fret| {
            let pitch_name = fret.pitch.replace(char::is_numeric, "");

            if pitch_name.starts_with('C') || pitch_name.starts_with('D') {
                current_octave = 1;
            }

            (0..4)
                .map(|oct| FretWithPitch {
                    fret: fret.fret + oct * 12,
                    interval: fret.interval.clone(),
                    pitch: format!("{}{}", pitch_name, current_octave + oct),
                })
                .filter(|f| {
                    let min_fret = tuning.strings.iter().map(|s| s.offset).min().unwrap_or(0);
                    f.fret >= min_fret && f.fret <= max_absolute_fret
                })
                .collect::<Vec<_>>()
        })
        .collect();

    convert_frets_to_positions_with_tuning(&octave_frets, tuning)
}

/// チューニング情報を返す
#[wasm_bindgen]
pub fn get_tuning_info(tuning_name: &str) -> JsValue {
    let tuning = Tuning::from_name(tuning_name).unwrap_or_else(Tuning::bass_4);
    serde_wasm_bindgen::to_value(&tuning).unwrap_or(JsValue::NULL)
}

/// 利用可能なチューニングプリセット一覧を返す
#[wasm_bindgen]
pub fn list_tunings() -> JsValue {
    let names = vec!["bass_4", "bass_5", "bass_6", "bass_drop_d"];
    serde_wasm_bindgen::to_value(&names).unwrap_or(JsValue::NULL)
}

/// 内部用のポジション取得関数
fn get_chord_positions_internal(chord: &str) -> Vec<Position> {
    // 特別なコード判定
    let is_all_keys = chord == "ALL_KEYS";
    let is_white_keys = chord == "WHITE_KEYS";
    let is_power_chord = chord.ends_with('5') && !chord.contains("♭5") && !chord.contains("-5");
    let is_octave_unison = chord.contains('8')
        && !chord
            .find('8')
            .and_then(|pos| chord.chars().nth(pos + 1))
            .is_some_and(|c| c.is_numeric());

    let (frets, use_root) = if is_all_keys {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "♭2".to_string(), fret: 1 },
            Fret { interval: "2".to_string(), fret: 2 },
            Fret { interval: "♭3".to_string(), fret: 3 },
            Fret { interval: "3".to_string(), fret: 4 },
            Fret { interval: "4".to_string(), fret: 5 },
            Fret { interval: "♭5".to_string(), fret: 6 },
            Fret { interval: "5".to_string(), fret: 7 },
            Fret { interval: "＃5".to_string(), fret: 8 },
            Fret { interval: "6".to_string(), fret: 9 },
            Fret { interval: "♭7".to_string(), fret: 10 },
            Fret { interval: "7".to_string(), fret: 11 },
        ];
        (frets, "C".to_string())
    } else if is_white_keys {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "2".to_string(), fret: 2 },
            Fret { interval: "3".to_string(), fret: 4 },
            Fret { interval: "4".to_string(), fret: 5 },
            Fret { interval: "5".to_string(), fret: 7 },
            Fret { interval: "6".to_string(), fret: 9 },
            Fret { interval: "7".to_string(), fret: 11 },
        ];
        (frets, "C".to_string())
    } else if is_power_chord {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "5".to_string(), fret: 7 },
        ];
        (frets, get_root_note(chord))
    } else if is_octave_unison {
        let frets = vec![
            Fret { interval: "1".to_string(), fret: 0 },
            Fret { interval: "8".to_string(), fret: 12 },
        ];
        (frets, get_root_note(chord))
    } else {
        // parse_chord_type で分離し、get_frets で構成音取得
        let (root, chord_type) = parse_chord_type(chord);
        let frets = get_frets(&chord_type);
        (frets, root)
    };

    let offset = get_fret_offset(&use_root);
    let frets_with_pitch = get_pitches(&use_root, &frets, offset - 12);

    // オクターブ番号をCで切り替える
    let mut current_octave = 0;
    let octave_frets: Vec<FretWithPitch> = frets_with_pitch
        .iter()
        .flat_map(|fret| {
            let pitch_name = fret.pitch.replace(char::is_numeric, "");

            // CまたはDで始まる場合オクターブを1に
            if pitch_name.starts_with('C') || pitch_name.starts_with('D') {
                current_octave = 1;
            }

            vec![
                FretWithPitch {
                    fret: fret.fret,
                    interval: fret.interval.clone(),
                    pitch: format!("{pitch_name}{current_octave}"),
                },
                FretWithPitch {
                    fret: fret.fret + 12,
                    interval: fret.interval.clone(),
                    pitch: format!("{}{}", pitch_name, current_octave + 1),
                },
                FretWithPitch {
                    fret: fret.fret + 24,
                    interval: fret.interval.clone(),
                    pitch: format!("{}{}", pitch_name, current_octave + 2),
                },
                FretWithPitch {
                    fret: fret.fret + 36,
                    interval: fret.interval.clone(),
                    pitch: format!("{}{}", pitch_name, current_octave + 3),
                },
            ]
            .into_iter()
            .filter(|f| f.fret >= 0 && f.fret <= 39)
            .collect::<Vec<_>>()
        })
        .collect();

    convert_frets_to_positions(&octave_frets)
}

/// インターバル記号を取得（chordUtil.ts の getInterval() に相当）
#[wasm_bindgen]
pub fn get_interval(chord: &str, target_pitch: &str) -> String {
    let target_name = target_pitch.replace(char::is_numeric, "");
    let root = get_root_note(chord);
    let pitches = get_pitch_map(&root);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chord_positions() {
        let positions = get_chord_positions_internal("C");
        assert!(!positions.is_empty());
    }

    #[test]
    fn test_get_interval() {
        assert_eq!(get_interval("C", "C2"), "1");
        assert_eq!(get_interval("C", "E2"), "3");
        assert_eq!(get_interval("C", "G2"), "5");
    }

    #[test]
    fn test_tuning_presets() {
        let bass4 = Tuning::bass_4();
        assert_eq!(bass4.strings.len(), 4);
        assert_eq!(bass4.strings[0].offset, 0);

        let bass5 = Tuning::bass_5();
        assert_eq!(bass5.strings.len(), 5);
        assert_eq!(bass5.strings[0].offset, -5);

        let bass6 = Tuning::bass_6();
        assert_eq!(bass6.strings.len(), 6);

        let drop_d = Tuning::bass_drop_d();
        assert_eq!(drop_d.strings[0].offset, -2);
        assert_eq!(drop_d.strings[0].open_note, "D");
    }

    #[test]
    fn test_chord_positions_with_tuning() {
        // 4弦で既存と同じ結果
        let pos_4 = get_chord_positions_internal("C");
        let pos_4t = get_chord_positions_with_tuning_internal("C", &Tuning::bass_4());
        assert_eq!(pos_4.len(), pos_4t.len());

        // 5弦では追加ポジションが存在するはず
        let pos_5 = get_chord_positions_with_tuning_internal("C", &Tuning::bass_5());
        assert!(pos_5.len() >= pos_4.len());
    }

    #[test]
    fn test_tuning_from_name() {
        assert!(Tuning::from_name("bass_4").is_some());
        assert!(Tuning::from_name("bass_5").is_some());
        assert!(Tuning::from_name("bass_6").is_some());
        assert!(Tuning::from_name("bass_drop_d").is_some());
        assert!(Tuning::from_name("unknown").is_none());
    }
}
