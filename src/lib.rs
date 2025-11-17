use wasm_bindgen::prelude::*;

pub mod chord;
pub mod core;
pub mod harmony;
pub mod scale;
pub mod utils;

#[wasm_bindgen(start)]
pub fn init() {
    // パニック時のエラーメッセージを改善（開発時のみ）
    // console_error_panic_hookが必要な場合はCargo.tomlに追加してください
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }
}
