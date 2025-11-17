use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// コード名のエイリアスを取得（日本語記譜対応）
#[wasm_bindgen]
pub fn get_chord_name_aliases(chord_name: &str) -> Vec<JsValue> {
    let aliases = get_chord_name_aliases_internal(chord_name);
    aliases.into_iter().map(|s| JsValue::from_str(&s)).collect()
}

/// コード名エイリアスの内部実装
fn get_chord_name_aliases_internal(chord_name: &str) -> Vec<String> {
    // ルート音を抽出
    let mut chars = chord_name.chars().peekable();
    let mut root = String::new();

    if let Some(c) = chars.next() {
        if !('A'..='G').contains(&c) {
            return vec![chord_name.to_string()];
        }
        root.push(c);
    }

    // 変化記号（#, ＃, ♯, b, ♭）を抽出
    if let Some(&c) = chars.peek() {
        if c == '#' || c == '＃' || c == '♯' || c == 'b' || c == '♭' {
            root.push(c);
            chars.next();
        }
    }

    // コードタイプ部分を抽出
    let chord_type: String = chars.collect();

    // タイプエイリアスマップを取得
    let type_alias_map = create_type_alias_map();

    if let Some(aliases) = type_alias_map.get(chord_type.as_str()) {
        aliases
            .iter()
            .map(|alias| format!("{}{}", root, alias))
            .collect()
    } else {
        vec![chord_name.to_string()]
    }
}

/// コードタイプのエイリアスマップを作成
fn create_type_alias_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // メジャーセブンス
    map.insert("maj7", vec!["maj7", "M7", "△7"]);
    map.insert("M7", vec!["maj7", "M7", "△7"]);
    map.insert("△7", vec!["maj7", "M7", "△7"]);

    // マイナー
    map.insert("m", vec!["m", "min", "-"]);
    map.insert("min", vec!["m", "min", "-"]);
    map.insert("-", vec!["m", "min", "-"]);

    // マイナーセブンス
    map.insert("m7", vec!["m7", "min7", "-7"]);
    map.insert("min7", vec!["m7", "min7", "-7"]);
    map.insert("-7", vec!["m7", "min7", "-7"]);

    // ドミナントセブンス
    map.insert("7", vec!["7", "dom7"]);
    map.insert("dom7", vec!["7", "dom7"]);

    // サスペンデッド
    map.insert("sus4", vec!["sus4", "sus"]);
    map.insert("sus", vec!["sus4", "sus"]);
    map.insert("sus2", vec!["sus2"]);

    // ディミニッシュ
    map.insert("dim", vec!["dim", "°"]);
    map.insert("°", vec!["dim", "°"]);

    // オーギュメント
    map.insert("aug", vec!["aug", "+"]);
    map.insert("+", vec!["aug", "+"]);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chord_name_aliases_maj7() {
        let aliases = get_chord_name_aliases_internal("Cmaj7");
        assert_eq!(aliases.len(), 3);
        assert!(aliases.contains(&"Cmaj7".to_string()));
        assert!(aliases.contains(&"CM7".to_string()));
        assert!(aliases.contains(&"C△7".to_string()));
    }

    #[test]
    fn test_get_chord_name_aliases_minor() {
        let aliases = get_chord_name_aliases_internal("Cm");
        assert_eq!(aliases.len(), 3);
        assert!(aliases.contains(&"Cm".to_string()));
        assert!(aliases.contains(&"Cmin".to_string()));
        assert!(aliases.contains(&"C-".to_string()));
    }

    #[test]
    fn test_get_chord_name_aliases_sharp() {
        let aliases = get_chord_name_aliases_internal("C＃m7");
        assert_eq!(aliases.len(), 3);
        assert!(aliases.contains(&"C＃m7".to_string()));
        assert!(aliases.contains(&"C＃min7".to_string()));
        assert!(aliases.contains(&"C＃-7".to_string()));
    }

    #[test]
    fn test_get_chord_name_aliases_unknown() {
        let aliases = get_chord_name_aliases_internal("Cxyz");
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0], "Cxyz");
    }
}
