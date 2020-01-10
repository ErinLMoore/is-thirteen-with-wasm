extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_thirteen(possibly_thirteen: i32) -> bool{
    possibly_thirteen == 13
}


#[cfg(test)]
mod tests {
    use is_thirteen;
    #[test]
    fn is_thirteen_test() {
        assert_eq!(is_thirteen(13), true);
    }
    #[test]
    fn is_not_thirteen_test() {
        assert_eq!(is_thirteen(12), false);
    }
}
