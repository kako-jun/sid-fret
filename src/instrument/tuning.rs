//! チューニング定義

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_tuning_from_name() {
        assert!(Tuning::from_name("bass_4").is_some());
        assert!(Tuning::from_name("bass_5").is_some());
        assert!(Tuning::from_name("bass_6").is_some());
        assert!(Tuning::from_name("bass_drop_d").is_some());
        assert!(Tuning::from_name("unknown").is_none());
    }
}
