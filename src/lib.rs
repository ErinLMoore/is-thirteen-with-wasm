extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_thirteen(possibly_thirteen: &str) -> bool{
    match possibly_thirteen.to_lowercase().as_str() {
        "13" => true,
        "thirteen" => true,
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use is_thirteen;
    #[test]
    fn is_thirteen_numerals_test() {
        assert_eq!(is_thirteen("13"), true);
    }
    #[test]
    fn is_thirteen_text_test() {
        assert_eq!(is_thirteen("thirteen"), true);
    }
    #[test]
    fn is_thirteen_caps_test() {
        assert_eq!(is_thirteen("ThIrTeEn"), true);
    }
    #[test]
    fn is_not_thirteen_test() {
        assert_eq!(is_thirteen("12"), false);
    }
}
