use serde::{Deserialize, Serialize};
use super::position::FingeringPattern;

/// 運指アルゴリズムの重み設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmWeights {
    /// 移動距離の重み（小さいほど良い）
    pub movement_weight: f32,
    /// ポジション変更の重み（小さいほど良い）
    pub position_change_weight: f32,
    /// 開放弦使用の重み（大きいほど開放弦を優先）
    pub open_string_weight: f32,
    /// 弦移動の重み（小さいほど弦移動を避ける）
    pub string_change_weight: f32,
}

impl Default for AlgorithmWeights {
    fn default() -> Self {
        Self {
            movement_weight: 1.0,
            position_change_weight: 2.0,
            open_string_weight: -1.0, // マイナスは良いことを示す
            string_change_weight: 0.5,
        }
    }
}

impl AlgorithmWeights {
    /// 最短移動優先の重み設定
    pub fn shortest() -> Self {
        Self {
            movement_weight: 10.0,
            position_change_weight: 1.0,
            open_string_weight: 0.0,
            string_change_weight: 0.5,
        }
    }

    /// ポジション固定優先の重み設定
    pub fn position_stable() -> Self {
        Self {
            movement_weight: 1.0,
            position_change_weight: 10.0,
            open_string_weight: 0.0,
            string_change_weight: 0.5,
        }
    }

    /// 開放弦活用の重み設定
    pub fn open_string() -> Self {
        Self {
            movement_weight: 1.0,
            position_change_weight: 2.0,
            open_string_weight: -5.0, // 大きくマイナス = 強く優先
            string_change_weight: 0.3,
        }
    }

    /// 弦移動優先の重み設定（横移動より縦移動）
    pub fn string_priority() -> Self {
        Self {
            movement_weight: 0.5,
            position_change_weight: 1.0,
            open_string_weight: 0.0,
            string_change_weight: -1.0, // 弦移動を優先
        }
    }

    /// バランス型の重み設定
    pub fn balanced() -> Self {
        Self::default()
    }

    /// 運指パターンのスコアを計算（低いほど良い）
    pub fn calculate_score(&self, pattern: &FingeringPattern) -> f32 {
        let movement_score = pattern.total_movement() as f32 * self.movement_weight;
        let position_score = pattern.position_changes() as f32 * self.position_change_weight;
        let open_string_score = pattern.open_string_count() as f32 * self.open_string_weight;
        let string_change_score = pattern.string_changes() as f32 * self.string_change_weight;

        movement_score + position_score + open_string_score + string_change_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingering::position::FretPosition;

    #[test]
    fn test_weights_shortest() {
        let weights = AlgorithmWeights::shortest();
        assert!(weights.movement_weight > weights.position_change_weight);
    }

    #[test]
    fn test_weights_position_stable() {
        let weights = AlgorithmWeights::position_stable();
        assert!(weights.position_change_weight > weights.movement_weight);
    }

    #[test]
    fn test_weights_open_string() {
        let weights = AlgorithmWeights::open_string();
        assert!(weights.open_string_weight < 0.0); // 開放弦は良いのでマイナス
    }

    #[test]
    fn test_calculate_score() {
        let weights = AlgorithmWeights::balanced();

        let pattern1 = FingeringPattern::new(
            vec![
                FretPosition::new(4, 0), // 開放弦
                FretPosition::new(4, 2),
            ],
            "test".to_string(),
        );

        let pattern2 = FingeringPattern::new(
            vec![
                FretPosition::new(4, 5),
                FretPosition::new(4, 10), // 大きく移動
            ],
            "test".to_string(),
        );

        let score1 = weights.calculate_score(&pattern1);
        let score2 = weights.calculate_score(&pattern2);

        // pattern1の方がスコアが低い（良い）はず
        assert!(score1 < score2);
    }
}
