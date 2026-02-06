use super::position::{FingeringPattern, FretPosition};
use super::scoring::AlgorithmWeights;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// 運指アルゴリズムの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FingeringMode {
    Shortest,       // 最短移動優先
    PositionStable, // ポジション固定優先
    StringPriority, // 弦移動優先（横移動より縦移動）
    OpenString,     // 開放弦活用
    Balanced,       // バランス型（スコアリング方式）
}

impl FromStr for FingeringMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shortest" => Ok(Self::Shortest),
            "position" | "position-stable" => Ok(Self::PositionStable),
            "string" | "string-priority" => Ok(Self::StringPriority),
            "open" | "open-string" => Ok(Self::OpenString),
            "balanced" => Ok(Self::Balanced),
            _ => Err(()),
        }
    }
}

impl FingeringMode {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Shortest => "shortest",
            Self::PositionStable => "position-stable",
            Self::StringPriority => "string-priority",
            Self::OpenString => "open-string",
            Self::Balanced => "balanced",
        }
    }

    pub fn weights(&self) -> AlgorithmWeights {
        match self {
            Self::Shortest => AlgorithmWeights::shortest(),
            Self::PositionStable => AlgorithmWeights::position_stable(),
            Self::StringPriority => AlgorithmWeights::string_priority(),
            Self::OpenString => AlgorithmWeights::open_string(),
            Self::Balanced => AlgorithmWeights::balanced(),
        }
    }
}

/// 音程（半音階の絶対位置）から可能なフレットポジションを全て生成
pub fn generate_all_positions(pitch: u8) -> Vec<FretPosition> {
    let mut positions = Vec::new();

    // 4弦ベースの各弦でのポジションを計算
    // E弦（4弦）: 0-24フレット
    if pitch <= 24 {
        positions.push(FretPosition::new(4, pitch));
    }

    // A弦（3弦）: 開放=5半音
    if (5..=29).contains(&pitch) {
        positions.push(FretPosition::new(3, pitch - 5));
    }

    // D弦（2弦）: 開放=10半音
    if (10..=34).contains(&pitch) {
        positions.push(FretPosition::new(2, pitch - 10));
    }

    // G弦（1弦）: 開放=15半音
    if (15..=39).contains(&pitch) {
        positions.push(FretPosition::new(1, pitch - 15));
    }

    positions
}

/// 最短移動アルゴリズム
pub fn calculate_shortest_path(pitches: &[u8]) -> FingeringPattern {
    if pitches.is_empty() {
        return FingeringPattern::new(vec![], "shortest".to_string());
    }

    let mut selected = Vec::new();
    let weights = AlgorithmWeights::shortest();

    for (i, &pitch) in pitches.iter().enumerate() {
        let candidates = generate_all_positions(pitch);

        if i == 0 {
            // 最初の音は開放弦を優先、なければ最も低いフレット
            let best = candidates
                .iter()
                .min_by_key(|p| (p.fret, p.string))
                .unwrap();
            selected.push(*best);
        } else {
            // 前の音からの移動距離が最小のものを選択
            let prev = &selected[i - 1];
            let best = candidates
                .iter()
                .min_by_key(|p| {
                    let fret_dist = (prev.fret as i32 - p.fret as i32).abs();
                    let string_dist = (prev.string as i32 - p.string as i32).abs();
                    fret_dist + string_dist * 2 // 弦移動にペナルティ
                })
                .unwrap();
            selected.push(*best);
        }
    }

    let mut pattern = FingeringPattern::new(selected, "shortest".to_string());
    pattern.score = weights.calculate_score(&pattern);
    pattern
}

/// ポジション固定優先アルゴリズム
pub fn calculate_position_stable(pitches: &[u8], base_position: u8) -> FingeringPattern {
    if pitches.is_empty() {
        return FingeringPattern::new(vec![], "position-stable".to_string());
    }

    let mut selected = Vec::new();
    let weights = AlgorithmWeights::position_stable();

    for &pitch in pitches {
        let candidates = generate_all_positions(pitch);

        // base_position付近のポジションを優先
        let best = candidates
            .iter()
            .min_by_key(|p| {
                let pos = p.position();
                let pos_dist = (base_position as i32 - pos as i32).abs();
                (pos_dist, p.fret) // ポジション距離優先、次にフレット番号
            })
            .unwrap();
        selected.push(*best);
    }

    let mut pattern = FingeringPattern::new(selected, "position-stable".to_string());
    pattern.score = weights.calculate_score(&pattern);
    pattern
}

/// 開放弦活用アルゴリズム
pub fn calculate_open_string(pitches: &[u8]) -> FingeringPattern {
    if pitches.is_empty() {
        return FingeringPattern::new(vec![], "open-string".to_string());
    }

    let mut selected = Vec::new();
    let weights = AlgorithmWeights::open_string();

    for &pitch in pitches {
        let candidates = generate_all_positions(pitch);

        // 開放弦を最優先、次に低いフレット
        let best = candidates
            .iter()
            .min_by_key(|p| {
                if p.fret == 0 {
                    (0, 0) // 開放弦は最優先
                } else {
                    (1, p.fret) // それ以外は低いフレット優先
                }
            })
            .unwrap();
        selected.push(*best);
    }

    let mut pattern = FingeringPattern::new(selected, "open-string".to_string());
    pattern.score = weights.calculate_score(&pattern);
    pattern
}

/// 弦移動優先アルゴリズム（横移動より縦移動）
pub fn calculate_string_priority(pitches: &[u8]) -> FingeringPattern {
    if pitches.is_empty() {
        return FingeringPattern::new(vec![], "string-priority".to_string());
    }

    let mut selected = Vec::new();
    let weights = AlgorithmWeights::string_priority();

    for (i, &pitch) in pitches.iter().enumerate() {
        let candidates = generate_all_positions(pitch);

        if i == 0 {
            // 最初は中央弦（A弦かD弦）を優先
            let best = candidates
                .iter()
                .min_by_key(|p| {
                    let string_center_dist = (p.string as i32 - 3).abs(); // 3=A弦
                    (string_center_dist, p.fret)
                })
                .unwrap();
            selected.push(*best);
        } else {
            // 弦移動を優先、フレット移動を避ける
            let prev = &selected[i - 1];
            let best = candidates
                .iter()
                .min_by_key(|p| {
                    if p.string != prev.string {
                        // 弦が違う場合は優先（低コスト）
                        let string_dist = (prev.string as i32 - p.string as i32).abs();
                        (0, string_dist)
                    } else {
                        // 同じ弦の場合はフレット移動コスト高
                        let fret_dist = (prev.fret as i32 - p.fret as i32).abs();
                        (fret_dist * 2, 0)
                    }
                })
                .unwrap();
            selected.push(*best);
        }
    }

    let mut pattern = FingeringPattern::new(selected, "string-priority".to_string());
    pattern.score = weights.calculate_score(&pattern);
    pattern
}

/// バランス型アルゴリズム（複数要素をスコアリング）
pub fn calculate_balanced(pitches: &[u8]) -> FingeringPattern {
    if pitches.is_empty() {
        return FingeringPattern::new(vec![], "balanced".to_string());
    }

    // 各アルゴリズムを試してスコアを計算
    let shortest = calculate_shortest_path(pitches);
    let position = calculate_position_stable(pitches, 5); // 5フレット付近
    let open = calculate_open_string(pitches);

    // 最もスコアが低いものを選択
    let weights = AlgorithmWeights::balanced();
    vec![shortest, position, open]
        .into_iter()
        .min_by(|a, b| {
            weights
                .calculate_score(a)
                .partial_cmp(&weights.calculate_score(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|mut p| {
            p.algorithm = "balanced".to_string();
            p.score = weights.calculate_score(&p);
            p
        })
        .unwrap()
}

/// WASM公開API: 運指計算
#[wasm_bindgen]
pub fn calculate_fingering(pitches: Vec<u8>, mode: &str) -> JsValue {
    let fingering_mode = mode.parse().unwrap_or(FingeringMode::Balanced);

    let pattern = match fingering_mode {
        FingeringMode::Shortest => calculate_shortest_path(&pitches),
        FingeringMode::PositionStable => calculate_position_stable(&pitches, 5),
        FingeringMode::StringPriority => calculate_string_priority(&pitches),
        FingeringMode::OpenString => calculate_open_string(&pitches),
        FingeringMode::Balanced => calculate_balanced(&pitches),
    };

    serde_wasm_bindgen::to_value(&pattern).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_all_positions() {
        // E1音（0半音）= E弦開放のみ
        let positions = generate_all_positions(0);
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].string, 4);
        assert_eq!(positions[0].fret, 0);

        // A1音（5半音）= E弦5フレット or A弦開放
        let positions = generate_all_positions(5);
        assert_eq!(positions.len(), 2);

        // C2音（8半音）= E弦8フレット or A弦3フレット
        let positions = generate_all_positions(8);
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn test_calculate_shortest_path() {
        // E-F-G のシーケンス（0, 1, 3半音）
        let pitches = vec![0, 1, 3];
        let pattern = calculate_shortest_path(&pitches);

        assert_eq!(pattern.positions.len(), 3);
        assert!(pattern.total_movement() < 10); // 最短移動のはず
    }

    #[test]
    fn test_calculate_open_string() {
        // A音（5半音）を含むシーケンス
        let pitches = vec![5, 7, 5];
        let pattern = calculate_open_string(&pitches);

        // 開放弦（A弦）を使用しているはず
        let open_count = pattern.positions.iter().filter(|p| p.fret == 0).count();
        assert!(open_count >= 1);
    }

    #[test]
    fn test_fingering_mode_from_str() {
        assert_eq!(
            "shortest".parse::<FingeringMode>(),
            Ok(FingeringMode::Shortest)
        );
        assert_eq!(
            "position".parse::<FingeringMode>(),
            Ok(FingeringMode::PositionStable)
        );
        assert_eq!(
            "open-string".parse::<FingeringMode>(),
            Ok(FingeringMode::OpenString)
        );
        assert!("invalid".parse::<FingeringMode>().is_err());
    }
}
