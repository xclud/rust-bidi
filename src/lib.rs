include!("paragraph.rs");
include!("letter_form.rs");
include!("shape_joining_type.rs");
include!("shaping.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Shaping test.
    fn shaping() {
        let text: &str = "مهدی";
        let mut text_u16 = text.encode_utf16().collect::<Vec<u16>>();

        perform_shaping(&mut text_u16);

        let result = String::from_utf16(text_u16.as_slice()).unwrap();
        assert_eq!(result, "ﻣﻬﺪﯼ");
        assert_eq!(text_u16, vec!['ﻣ' as u16, 'ﻬ' as u16, 'ﺪ' as u16, 'ﯼ' as u16]);
    }

    #[test]
    /// Paragraph test.
    fn paragraph() {
        let text: &str = "Two\nParagraphs";
        let text_u16 = text.encode_utf16().collect::<Vec<u16>>();

        let paragraphs = split_string_to_paragraphs(&text_u16);

        assert_eq!(paragraphs.len(), 2);
        assert_eq!(paragraphs[0].text, vec![84, 119, 111]);
    }
}
