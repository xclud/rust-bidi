/// Performs in-place character shaping of the string.
/// Replaces each letter with it's relating initial, medial, final or isolated form.
pub fn perform_shaping(text: &mut Vec<u16>) {
    let mut last_jt = ShapeJoiningType::NonJoining;
    let mut last_form = LetterForm::Isolated;
    let mut last_pos = 0usize;

    let length = text.len();
    let mut letter_forms = vec![LetterForm::Initial; length];

    for i in 0..length {
        let ch = text[i];
        let jt = get_shape_joining_type(ch);

        if (jt == ShapeJoiningType::Right
            || jt == ShapeJoiningType::Dual
            || jt == ShapeJoiningType::Causing)
            && (last_jt == ShapeJoiningType::Left
            || last_jt == ShapeJoiningType::Dual
            || last_jt == ShapeJoiningType::Causing)
        {
            if last_form == LetterForm::Isolated
                && (last_jt == ShapeJoiningType::Dual || last_jt == ShapeJoiningType::Left)
            {
                letter_forms[last_pos] = LetterForm::Initial;
            } else if last_form == LetterForm::Final && last_jt == ShapeJoiningType::Dual {
                letter_forms[last_pos] = LetterForm::Medial;
            }
            letter_forms[i] = LetterForm::Final;
            last_form = LetterForm::Final;
            last_jt = jt;
            last_pos = i;
        } else if jt != ShapeJoiningType::Transparent {
            letter_forms[i] = LetterForm::Isolated;
            last_form = LetterForm::Isolated;
            last_jt = jt;
            last_pos = i;
        } else {
            letter_forms[i] = LetterForm::Isolated;
        }
    }

    for i in 0..length {
        let ch = text[i];
        // let jt = get_shape_joining_type(ch);

        text[i] = get_character_by_letter_form(ch, letter_forms[i]);
    }
}