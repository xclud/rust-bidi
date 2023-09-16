include!("character_type.rs");

pub struct Paragraph {
    pub text: Vec<u16>,
    pub separator: u16,
}

/// Split the text into separate paragraphs.
///
/// A paragraph separator is kept with the previous paragraph.
/// Within each paragraph, apply all the other rules of this algorithm.
///
/// Reference: http://www.unicode.org/reports/tr9/#P1.
pub fn split_string_to_paragraphs(logical_string: &Vec<u16>) -> Vec<Paragraph> {
    let mut paragraphs: Vec<Paragraph> = Vec::new();
    let length = logical_string.len();

    let mut text: Vec<u16> = Vec::new();

    for i in 0..length {
        let ch = logical_string[i];
        let tp = get_character_type(ch);
        if tp == CharacterType::ParagraphSeparator {
            let paragraph = Paragraph {
                text,
                separator: ch,
            };
            paragraphs.push(paragraph);
            text = Vec::new();
        } else {
            text.push(ch);
        }
    }

    // if string ended without a paragraph separator.
    if text.len() != 0
    {
        paragraphs.push(Paragraph {
            text,
            separator: 0xffff,
        });
    }
    return paragraphs;
}
