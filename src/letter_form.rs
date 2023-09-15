/// The four different available letter presentation forms.
#[derive(Clone, PartialEq, Copy)]
enum LetterForm {
    /// A presentation form of a letter that begins a sequence of connected letters.
    Initial,

    /// A presentation form of a letter that is connected to other letters on both sides.
    Medial,

    /// A presentation form of a letter that ends a sequence of connected letters.
    Final,

    /// A presentation form of a letter that is not connected to other letters on either sides.
    Isolated,
}

fn get_character_by_letter_form(c: u16, f: LetterForm) -> u16 {
    if c == 0x0671 {
        if f == LetterForm::Isolated {
            return 0xFB50;
        }
        if f == LetterForm::Final {
            return 0xFB51;
        }
    }

    if c == 0x067B {
        if f == LetterForm::Isolated {
            return 0xFB52;
        }
        if f == LetterForm::Final {
            return 0xFB53;
        }
        if f == LetterForm::Initial {
            return 0xFB54;
        }
        if f == LetterForm::Medial {
            return 0xFB55;
        }
    }

    if c == 0x067E {
        if f == LetterForm::Isolated {
            return 0xFB56;
        }
        if f == LetterForm::Final {
            return 0xFB57;
        }
        if f == LetterForm::Initial {
            return 0xFB58;
        }
        if f == LetterForm::Medial {
            return 0xFB59;
        }
    }

    if c == 0x0680 {
        if f == LetterForm::Isolated {
            return 0xFB5A;
        }
        if f == LetterForm::Final {
            return 0xFB5B;
        }
        if f == LetterForm::Initial {
            return 0xFB5C;
        }
        if f == LetterForm::Medial {
            return 0xFB5D;
        }
    }

    if c == 0x067A {
        if f == LetterForm::Isolated {
            return 0xFB5E;
        }
        if f == LetterForm::Final {
            return 0xFB5F;
        }
        if f == LetterForm::Initial {
            return 0xFB60;
        }
        if f == LetterForm::Medial {
            return 0xFB61;
        }
    }

    if c == 0x067F {
        if f == LetterForm::Isolated {
            return 0xFB62;
        }
        if f == LetterForm::Final {
            return 0xFB63;
        }
        if f == LetterForm::Initial {
            return 0xFB64;
        }
        if f == LetterForm::Medial {
            return 0xFB65;
        }
    }

    if c == 0x0679 {
        if f == LetterForm::Isolated {
            return 0xFB66;
        }
        if f == LetterForm::Final {
            return 0xFB67;
        }
        if f == LetterForm::Initial {
            return 0xFB68;
        }
        if f == LetterForm::Medial {
            return 0xFB69;
        }
    }

    if c == 0x06A4 {
        if f == LetterForm::Isolated {
            return 0xFB6A;
        }
        if f == LetterForm::Final {
            return 0xFB6B;
        }
        if f == LetterForm::Initial {
            return 0xFB6C;
        }
        if f == LetterForm::Medial {
            return 0xFB6D;
        }
    }

    if c == 0x06A6 {
        if f == LetterForm::Isolated {
            return 0xFB6E;
        }
        if f == LetterForm::Final {
            return 0xFB6F;
        }
        if f == LetterForm::Initial {
            return 0xFB70;
        }
        if f == LetterForm::Medial {
            return 0xFB71;
        }
    }

    if c == 0x0684 {
        if f == LetterForm::Isolated {
            return 0xFB72;
        }
        if f == LetterForm::Final {
            return 0xFB73;
        }
        if f == LetterForm::Initial {
            return 0xFB74;
        }
        if f == LetterForm::Medial {
            return 0xFB75;
        }
    }

    if c == 0x0683 {
        if f == LetterForm::Isolated {
            return 0xFB76;
        }
        if f == LetterForm::Final {
            return 0xFB77;
        }
        if f == LetterForm::Initial {
            return 0xFB78;
        }
        if f == LetterForm::Medial {
            return 0xFB79;
        }
    }

    if c == 0x0686 {
        if f == LetterForm::Isolated {
            return 0xFB7A;
        }
        if f == LetterForm::Final {
            return 0xFB7B;
        }
        if f == LetterForm::Initial {
            return 0xFB7C;
        }
        if f == LetterForm::Medial {
            return 0xFB7D;
        }
    }

    if c == 0x0687 {
        if f == LetterForm::Isolated {
            return 0xFB7E;
        }
        if f == LetterForm::Final {
            return 0xFB7F;
        }
        if f == LetterForm::Initial {
            return 0xFB80;
        }
        if f == LetterForm::Medial {
            return 0xFB81;
        }
    }

    if c == 0x068D {
        if f == LetterForm::Isolated {
            return 0xFB82;
        }
        if f == LetterForm::Final {
            return 0xFB83;
        }
    }

    if c == 0x068C {
        if f == LetterForm::Isolated {
            return 0xFB84;
        }
        if f == LetterForm::Final {
            return 0xFB85;
        }
    }

    if c == 0x068E {
        if f == LetterForm::Isolated {
            return 0xFB86;
        }
        if f == LetterForm::Final {
            return 0xFB87;
        }
    }

    if c == 0x0688 {
        if f == LetterForm::Isolated {
            return 0xFB88;
        }
        if f == LetterForm::Final {
            return 0xFB89;
        }
    }

    if c == 0x0698 {
        if f == LetterForm::Isolated {
            return 0xFB8A;
        }
        if f == LetterForm::Final {
            return 0xFB8B;
        }
    }

    if c == 0x0691 {
        if f == LetterForm::Isolated {
            return 0xFB8C;
        }
        if f == LetterForm::Final {
            return 0xFB8D;
        }
    }

    if c == 0x06A9 {
        if f == LetterForm::Isolated {
            return 0xFB8E;
        }
        if f == LetterForm::Final {
            return 0xFB8F;
        }
        if f == LetterForm::Initial {
            return 0xFB90;
        }
        if f == LetterForm::Medial {
            return 0xFB91;
        }
    }

    if c == 0x06AF {
        if f == LetterForm::Isolated {
            return 0xFB92;
        }
        if f == LetterForm::Final {
            return 0xFB93;
        }
        if f == LetterForm::Initial {
            return 0xFB94;
        }
        if f == LetterForm::Medial {
            return 0xFB95;
        }
    }

    if c == 0x06B3 {
        if f == LetterForm::Isolated {
            return 0xFB96;
        }
        if f == LetterForm::Final {
            return 0xFB97;
        }
        if f == LetterForm::Initial {
            return 0xFB98;
        }
        if f == LetterForm::Medial {
            return 0xFB99;
        }
    }

    if c == 0x06B1 {
        if f == LetterForm::Isolated {
            return 0xFB9A;
        }
        if f == LetterForm::Final {
            return 0xFB9B;
        }
        if f == LetterForm::Initial {
            return 0xFB9C;
        }
        if f == LetterForm::Medial {
            return 0xFB9D;
        }
    }

    if c == 0x06BA {
        if f == LetterForm::Isolated {
            return 0xFB9E;
        }
        if f == LetterForm::Final {
            return 0xFB9F;
        }
    }

    if c == 0x06BB {
        if f == LetterForm::Isolated {
            return 0xFBA0;
        }
        if f == LetterForm::Final {
            return 0xFBA1;
        }
        if f == LetterForm::Initial {
            return 0xFBA2;
        }
        if f == LetterForm::Medial {
            return 0xFBA3;
        }
    }

    if c == 0x06C0 {
        if f == LetterForm::Isolated {
            return 0xFBA4;
        }
        if f == LetterForm::Final {
            return 0xFBA5;
        }
    }

    if c == 0x06C1 {
        if f == LetterForm::Isolated {
            return 0xFBA6;
        }
        if f == LetterForm::Final {
            return 0xFBA7;
        }
        if f == LetterForm::Initial {
            return 0xFBA8;
        }
        if f == LetterForm::Medial {
            return 0xFBA9;
        }
    }

    if c == 0x06BE {
        if f == LetterForm::Isolated {
            return 0xFBAA;
        }
        if f == LetterForm::Final {
            return 0xFBAB;
        }
        if f == LetterForm::Initial {
            return 0xFBAC;
        }
        if f == LetterForm::Medial {
            return 0xFBAD;
        }
    }

    if c == 0x06D2 {
        if f == LetterForm::Isolated {
            return 0xFBAE;
        }
        if f == LetterForm::Final {
            return 0xFBAF;
        }
    }

    if c == 0x06D3 {
        if f == LetterForm::Isolated {
            return 0xFBB0;
        }
        if f == LetterForm::Final {
            return 0xFBB1;
        }
    }

    if c == 0x06AD {
        if f == LetterForm::Isolated {
            return 0xFBD3;
        }
        if f == LetterForm::Final {
            return 0xFBD4;
        }
        if f == LetterForm::Initial {
            return 0xFBD5;
        }
        if f == LetterForm::Medial {
            return 0xFBD6;
        }
    }

    if c == 0x06C7 {
        if f == LetterForm::Isolated {
            return 0xFBD7;
        }
        if f == LetterForm::Final {
            return 0xFBD8;
        }
    }

    if c == 0x06C6 {
        if f == LetterForm::Isolated {
            return 0xFBD9;
        }
        if f == LetterForm::Final {
            return 0xFBDA;
        }
    }

    if c == 0x06C8 {
        if f == LetterForm::Isolated {
            return 0xFBDB;
        }
        if f == LetterForm::Final {
            return 0xFBDC;
        }
    }

    if c == 0x0677 {
        if f == LetterForm::Isolated {
            return 0xFBDD;
        }
    }
    if c == 0x06CB {
        if f == LetterForm::Isolated {
            return 0xFBDE;
        }
        if f == LetterForm::Final {
            return 0xFBDF;
        }
    }

    if c == 0x06C5 {
        if f == LetterForm::Isolated {
            return 0xFBE0;
        }
        if f == LetterForm::Final {
            return 0xFBE1;
        }
    }

    if c == 0x06C9 {
        if f == LetterForm::Isolated {
            return 0xFBE2;
        }
        if f == LetterForm::Final {
            return 0xFBE3;
        }
    }

    if c == 0x06D0 {
        if f == LetterForm::Isolated {
            return 0xFBE4;
        }
        if f == LetterForm::Final {
            return 0xFBE5;
        }
        if f == LetterForm::Initial {
            return 0xFBE6;
        }
        if f == LetterForm::Medial {
            return 0xFBE7;
        }
    }

    if c == 0x0649 {
        if f == LetterForm::Initial {
            return 0xFBE8;
        }
        if f == LetterForm::Medial {
            return 0xFBE9;
        }
    }

    if c == 0x06CC {
        if f == LetterForm::Isolated {
            return 0xFBFC;
        }
        if f == LetterForm::Final {
            return 0xFBFD;
        }
        if f == LetterForm::Initial {
            return 0xFBFE;
        }
        if f == LetterForm::Medial {
            return 0xFBFF;
        }
    }

    if c == 0x0621 {
        if f == LetterForm::Isolated {
            return 0xFE80;
        }
    }
    if c == 0x0622 {
        if f == LetterForm::Isolated {
            return 0xFE81;
        }
        if f == LetterForm::Final {
            return 0xFE82;
        }
    }

    if c == 0x0623 {
        if f == LetterForm::Isolated {
            return 0xFE83;
        }
        if f == LetterForm::Final {
            return 0xFE84;
        }
    }

    if c == 0x0624 {
        if f == LetterForm::Isolated {
            return 0xFE85;
        }
        if f == LetterForm::Final {
            return 0xFE86;
        }
    }

    if c == 0x0625 {
        if f == LetterForm::Isolated {
            return 0xFE87;
        }
        if f == LetterForm::Final {
            return 0xFE88;
        }
    }

    if c == 0x0626 {
        if f == LetterForm::Isolated {
            return 0xFE89;
        }
        if f == LetterForm::Final {
            return 0xFE8A;
        }
        if f == LetterForm::Initial {
            return 0xFE8B;
        }
        if f == LetterForm::Medial {
            return 0xFE8C;
        }
    }

    if c == 0x0627 {
        if f == LetterForm::Isolated {
            return 0xFE8D;
        }
        if f == LetterForm::Final {
            return 0xFE8E;
        }
    }

    if c == 0x0628 {
        if f == LetterForm::Isolated {
            return 0xFE8F;
        }
        if f == LetterForm::Final {
            return 0xFE90;
        }
        if f == LetterForm::Initial {
            return 0xFE91;
        }
        if f == LetterForm::Medial {
            return 0xFE92;
        }
    }

    if c == 0x0629 {
        if f == LetterForm::Isolated {
            return 0xFE93;
        }
        if f == LetterForm::Final {
            return 0xFE94;
        }
    }

    if c == 0x062A {
        if f == LetterForm::Isolated {
            return 0xFE95;
        }
        if f == LetterForm::Final {
            return 0xFE96;
        }
        if f == LetterForm::Initial {
            return 0xFE97;
        }
        if f == LetterForm::Medial {
            return 0xFE98;
        }
    }

    if c == 0x062B {
        if f == LetterForm::Isolated {
            return 0xFE99;
        }
        if f == LetterForm::Final {
            return 0xFE9A;
        }
        if f == LetterForm::Initial {
            return 0xFE9B;
        }
        if f == LetterForm::Medial {
            return 0xFE9C;
        }
    }

    if c == 0x062C {
        if f == LetterForm::Isolated {
            return 0xFE9D;
        }
        if f == LetterForm::Final {
            return 0xFE9E;
        }
        if f == LetterForm::Initial {
            return 0xFE9F;
        }
        if f == LetterForm::Medial {
            return 0xFEA0;
        }
    }

    if c == 0x062D {
        if f == LetterForm::Isolated {
            return 0xFEA1;
        }
        if f == LetterForm::Final {
            return 0xFEA2;
        }
        if f == LetterForm::Initial {
            return 0xFEA3;
        }
        if f == LetterForm::Medial {
            return 0xFEA4;
        }
    }

    if c == 0x062E {
        if f == LetterForm::Isolated {
            return 0xFEA5;
        }
        if f == LetterForm::Final {
            return 0xFEA6;
        }
        if f == LetterForm::Initial {
            return 0xFEA7;
        }
        if f == LetterForm::Medial {
            return 0xFEA8;
        }
    }

    if c == 0x062F {
        if f == LetterForm::Isolated {
            return 0xFEA9;
        }
        if f == LetterForm::Final {
            return 0xFEAA;
        }
    }

    if c == 0x0630 {
        if f == LetterForm::Isolated {
            return 0xFEAB;
        }
        if f == LetterForm::Final {
            return 0xFEAC;
        }
    }

    if c == 0x0631 {
        if f == LetterForm::Isolated {
            return 0xFEAD;
        }
        if f == LetterForm::Final {
            return 0xFEAE;
        }
    }

    if c == 0x0632 {
        if f == LetterForm::Isolated {
            return 0xFEAF;
        }
        if f == LetterForm::Final {
            return 0xFEB0;
        }
    }

    if c == 0x0633 {
        if f == LetterForm::Isolated {
            return 0xFEB1;
        }
        if f == LetterForm::Final {
            return 0xFEB2;
        }
        if f == LetterForm::Initial {
            return 0xFEB3;
        }
        if f == LetterForm::Medial {
            return 0xFEB4;
        }
    }

    if c == 0x0634 {
        if f == LetterForm::Isolated {
            return 0xFEB5;
        }
        if f == LetterForm::Final {
            return 0xFEB6;
        }
        if f == LetterForm::Initial {
            return 0xFEB7;
        }
        if f == LetterForm::Medial {
            return 0xFEB8;
        }
    }

    if c == 0x0635 {
        if f == LetterForm::Isolated {
            return 0xFEB9;
        }
        if f == LetterForm::Final {
            return 0xFEBA;
        }
        if f == LetterForm::Initial {
            return 0xFEBB;
        }
        if f == LetterForm::Medial {
            return 0xFEBC;
        }
    }

    if c == 0x0636 {
        if f == LetterForm::Isolated {
            return 0xFEBD;
        }
        if f == LetterForm::Final {
            return 0xFEBE;
        }
        if f == LetterForm::Initial {
            return 0xFEBF;
        }
        if f == LetterForm::Medial {
            return 0xFEC0;
        }
    }

    if c == 0x0637 {
        if f == LetterForm::Isolated {
            return 0xFEC1;
        }
        if f == LetterForm::Final {
            return 0xFEC2;
        }
        if f == LetterForm::Initial {
            return 0xFEC3;
        }
        if f == LetterForm::Medial {
            return 0xFEC4;
        }
    }

    if c == 0x0638 {
        if f == LetterForm::Isolated {
            return 0xFEC5;
        }
        if f == LetterForm::Final {
            return 0xFEC6;
        }
        if f == LetterForm::Initial {
            return 0xFEC7;
        }
        if f == LetterForm::Medial {
            return 0xFEC8;
        }
    }

    if c == 0x0639 {
        if f == LetterForm::Isolated {
            return 0xFEC9;
        }
        if f == LetterForm::Final {
            return 0xFECA;
        }
        if f == LetterForm::Initial {
            return 0xFECB;
        }
        if f == LetterForm::Medial {
            return 0xFECC;
        }
    }

    if c == 0x063A {
        if f == LetterForm::Isolated {
            return 0xFECD;
        }
        if f == LetterForm::Final {
            return 0xFECE;
        }
        if f == LetterForm::Initial {
            return 0xFECF;
        }
        if f == LetterForm::Medial {
            return 0xFED0;
        }
    }

    if c == 0x0641 {
        if f == LetterForm::Isolated {
            return 0xFED1;
        }
        if f == LetterForm::Final {
            return 0xFED2;
        }
        if f == LetterForm::Initial {
            return 0xFED3;
        }
        if f == LetterForm::Medial {
            return 0xFED4;
        }
    }

    if c == 0x0642 {
        if f == LetterForm::Isolated {
            return 0xFED5;
        }
        if f == LetterForm::Final {
            return 0xFED6;
        }
        if f == LetterForm::Initial {
            return 0xFED7;
        }
        if f == LetterForm::Medial {
            return 0xFED8;
        }
    }

    if c == 0x0643 {
        if f == LetterForm::Isolated {
            return 0xFED9;
        }
        if f == LetterForm::Final {
            return 0xFEDA;
        }
        if f == LetterForm::Initial {
            return 0xFEDB;
        }
        if f == LetterForm::Medial {
            return 0xFEDC;
        }
    }

    if c == 0x0644 {
        if f == LetterForm::Isolated {
            return 0xFEDD;
        }
        if f == LetterForm::Final {
            return 0xFEDE;
        }
        if f == LetterForm::Initial {
            return 0xFEDF;
        }
        if f == LetterForm::Medial {
            return 0xFEE0;
        }
    }

    if c == 0x0645 {
        if f == LetterForm::Isolated {
            return 0xFEE1;
        }
        if f == LetterForm::Final {
            return 0xFEE2;
        }
        if f == LetterForm::Initial {
            return 0xFEE3;
        }
        if f == LetterForm::Medial {
            return 0xFEE4;
        }
    }

    if c == 0x0646 {
        if f == LetterForm::Isolated {
            return 0xFEE5;
        }
        if f == LetterForm::Final {
            return 0xFEE6;
        }
        if f == LetterForm::Initial {
            return 0xFEE7;
        }
        if f == LetterForm::Medial {
            return 0xFEE8;
        }
    }

    if c == 0x0647 {
        if f == LetterForm::Isolated {
            return 0xFEE9;
        }
        if f == LetterForm::Final {
            return 0xFEEA;
        }
        if f == LetterForm::Initial {
            return 0xFEEB;
        }
        if f == LetterForm::Medial {
            return 0xFEEC;
        }
    }

    if c == 0x0648 {
        if f == LetterForm::Isolated {
            return 0xFEED;
        }
        if f == LetterForm::Final {
            return 0xFEEE;
        }
    }

    if c == 0x0649 {
        if f == LetterForm::Isolated {
            return 0xFEEF;
        }
        if f == LetterForm::Final {
            return 0xFEF0;
        }
    }

    if c == 0x064A {
        if f == LetterForm::Isolated {
            return 0xFEF1;
        }
        if f == LetterForm::Final {
            return 0xFEF2;
        }
        if f == LetterForm::Initial {
            return 0xFEF3;
        }
        if f == LetterForm::Medial {
            return 0xFEF4;
        }
    }

    return c;
}

