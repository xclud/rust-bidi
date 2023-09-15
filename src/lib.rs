include!("character_type.rs");
include!("letter_form.rs");
include!("shape_joining_type.rs");
include!("shaping.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shaping() {
        let text: &str = "مهدی";
        let mut text_u16 = text.encode_utf16().collect::<Vec<u16>>();

        perform_shaping(&mut text_u16);

        let result = String::from_utf16(text_u16.as_slice()).unwrap();
        assert_eq!(result, "ﻣﻬﺪﯼ");
    }
}
