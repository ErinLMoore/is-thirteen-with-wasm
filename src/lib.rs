// extern crate wasm_bindgen;

// use wasm_bindgen::prelude::*;


pub trait IsThirteen {
    fn is_thirteen(&self) -> bool;
}

// #[wasm_bindgen]
impl IsThirteen for str {
    fn is_thirteen(&self) -> bool{
        match self.to_lowercase().as_str() {
            "13" => true,
            "thirteen" => true,
            _ => false,
        }
    }
}

impl IsThirteen for i32 {
    fn is_thirteen(&self) -> bool{
        match self {
            13 => true,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use IsThirteen;
    #[test]
    fn is_thirteen_numerals_test() {
        assert_eq!(13.is_thirteen(), true);
    }
    #[test]
    fn is_thirteen_naaaaaa_test() {
        assert_eq!("13".is_thirteen(), true);
    }
    #[test]
    fn is_thirteen_text_test() {
        assert_eq!("thirteen".is_thirteen(), true);
    }
    #[test]
    fn is_thirteen_caps_test() {
        assert_eq!("ThIrTeEn".is_thirteen(), true);
    }
    #[test]
    fn is_not_thirteen_int_test() {
        assert_eq!(12.is_thirteen(), false);
    }
    #[test]
    fn is_not_thirteen_text_test() {
        assert_eq!("twelve".is_thirteen(), false);
    }
}
