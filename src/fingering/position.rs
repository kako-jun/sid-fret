use serde::{Deserialize, Serialize};

/// ベースのフレットポジション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FretPosition {
    /// 弦番号（1=G弦, 2=D弦, 3=A弦, 4=E弦）
    pub string: u8,
    /// フレット番号（0=開放弦）
    pub fret: u8,
    /// 推奨される指番号（1=人差し指, 2=中指, 3=薬指, 4=小指）
    pub finger: Option<u8>,
}

impl FretPosition {
    pub fn new(string: u8, fret: u8) -> Self {
        Self {
            string,
            fret,
            finger: None,
        }
    }

    pub fn with_finger(mut self, finger: u8) -> Self {
        self.finger = Some(finger);
        self
    }

    /// ポジション（フレット範囲）を取得（例：5フレット付近 = ポジション5）
    pub fn position(&self) -> u8 {
        if self.fret == 0 {
            0
        } else {
            ((self.fret - 1) / 4) * 4 + 1
        }
    }

    /// 半音階での絶対位置を取得（E弦0フレット = 0）
    pub fn absolute_pitch(&self) -> u8 {
        let string_offset = match self.string {
            1 => 15, // G弦
            2 => 10, // D弦
            3 => 5,  // A弦
            4 => 0,  // E弦
            _ => 0,
        };
        string_offset + self.fret
    }
}

/// 運指パターン
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingeringPattern {
    pub positions: Vec<FretPosition>,
    pub score: f32,
    pub algorithm: String,
}

impl FingeringPattern {
    pub fn new(positions: Vec<FretPosition>, algorithm: String) -> Self {
        Self {
            positions,
            score: 0.0,
            algorithm,
        }
    }

    pub fn with_score(mut self, score: f32) -> Self {
        self.score = score;
        self
    }

    /// フレット移動の総距離を計算
    pub fn total_movement(&self) -> u32 {
        let mut total = 0u32;
        for i in 1..self.positions.len() {
            let prev = &self.positions[i - 1];
            let curr = &self.positions[i];

            // 同じ弦の場合はフレット間距離
            if prev.string == curr.string {
                total += (prev.fret as i32 - curr.fret as i32).abs() as u32;
            } else {
                // 異なる弦の場合は弦移動ペナルティ + フレット差
                total += 1; // 弦移動ペナルティ
                total += (prev.fret as i32 - curr.fret as i32).abs() as u32 / 2;
            }
        }
        total
    }

    /// ポジション変更の回数を計算
    pub fn position_changes(&self) -> u32 {
        let mut changes = 0u32;
        for i in 1..self.positions.len() {
            let prev_pos = self.positions[i - 1].position();
            let curr_pos = self.positions[i].position();
            if prev_pos != curr_pos && curr_pos != 0 && prev_pos != 0 {
                changes += 1;
            }
        }
        changes
    }

    /// 開放弦の使用回数を計算
    pub fn open_string_count(&self) -> u32 {
        self.positions.iter().filter(|p| p.fret == 0).count() as u32
    }

    /// 弦移動の回数を計算
    pub fn string_changes(&self) -> u32 {
        let mut changes = 0u32;
        for i in 1..self.positions.len() {
            if self.positions[i - 1].string != self.positions[i].string {
                changes += 1;
            }
        }
        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fret_position_absolute_pitch() {
        let pos = FretPosition::new(4, 0); // E弦開放
        assert_eq!(pos.absolute_pitch(), 0);

        let pos = FretPosition::new(4, 5); // E弦5フレット（A音）
        assert_eq!(pos.absolute_pitch(), 5);

        let pos = FretPosition::new(3, 0); // A弦開放
        assert_eq!(pos.absolute_pitch(), 5);

        let pos = FretPosition::new(1, 0); // G弦開放
        assert_eq!(pos.absolute_pitch(), 15);
    }

    #[test]
    fn test_fret_position_position() {
        assert_eq!(FretPosition::new(4, 0).position(), 0); // 開放
        assert_eq!(FretPosition::new(4, 1).position(), 1); // 1stポジション
        assert_eq!(FretPosition::new(4, 4).position(), 1); // 1stポジション
        assert_eq!(FretPosition::new(4, 5).position(), 5); // 5thポジション
        assert_eq!(FretPosition::new(4, 8).position(), 5); // 5thポジション
        assert_eq!(FretPosition::new(4, 9).position(), 9); // 9thポジション
    }

    #[test]
    fn test_fingering_pattern_total_movement() {
        let pattern = FingeringPattern::new(
            vec![
                FretPosition::new(4, 3),
                FretPosition::new(4, 5), // 同じ弦、2フレット移動
                FretPosition::new(3, 5), // 弦移動
            ],
            "test".to_string(),
        );

        // 2 (フレット移動) + 1 (弦移動ペナルティ) = 3
        assert!(pattern.total_movement() >= 2);
    }

    #[test]
    fn test_fingering_pattern_metrics() {
        let pattern = FingeringPattern::new(
            vec![
                FretPosition::new(4, 0), // 開放弦
                FretPosition::new(4, 3),
                FretPosition::new(3, 5), // 弦移動
                FretPosition::new(3, 0), // 開放弦
            ],
            "test".to_string(),
        );

        assert_eq!(pattern.open_string_count(), 2);
        assert_eq!(pattern.string_changes(), 1);
    }
}
