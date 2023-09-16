/// Types of Bidi characters (Table 4 in the Unicode Bidi Algorithm).
#[derive(PartialEq)]
enum CharacterType {
    /// Left-to-Right (LTR).
    LeftToRight,

    /// Left-to-Right Embedding (LRE).
    LeftToRightEmbedding,

    /// Left-to-Right Override (LRO).
    LeftToRightOverride,

    /// Left-to-Right Isolate (LRI).
    LeftToRightIsolate,

    /// Right-to-Left (RTL).
    RightToLeft,

    /// Right-to-Left Embedding (RLE).
    RightToLeftEmbedding,

    /// Right-to-Left Override (RLO).
    RightToLeftOverride,

    /// Right-to-Left Isolate (RLI).
    RightToLeftIsolate,

    /// First Strong Isolate (FSI).
    FirstStrongIsolate,

    /// Pop Directional Format (PDF).
    PopDirectionalFormat,

    /// Pop Directional Isolate (PDI).
    PopDirectionalIsolate,

    /// European Number (EN).
    EuropeanNumber,

    /// European Number Separator (ES).
    EuropeanNumberSeparator,

    /// European Number Terminator (ET).
    EuropeanNumberTerminator,

    /// Persian Letter (AL).
    PersianLetter,

    /// Persian Number (AN).
    PersianNumber,

    /// Common Number Separator (CS).
    CommonNumberSeparator,

    /// NonSpacing Mark (MN).
    NonSpacingMark,

    /// Boundary Neutral (BN).
    BoundaryNeutral,

    /// Paragraph Separator (B).
    ParagraphSeparator,

    /// Segment Separator (S).
    SegmentSeparator,

    /// Whitespace (WS).
    Whitespace,

    /// Other Neutrals (ON).
    OtherNeutrals,
}

fn get_character_type(c: u16) -> CharacterType {
    if
    /*c >= 0 &&*/
    c <= 8 {
        return CharacterType::BoundaryNeutral;
    }
    if c == 9 {
        return CharacterType::SegmentSeparator;
    }
    if c == 10 {
        return CharacterType::ParagraphSeparator;
    }
    if c == 11 {
        return CharacterType::SegmentSeparator;
    }
    if c == 12 {
        return CharacterType::Whitespace;
    }
    if c == 13 {
        return CharacterType::ParagraphSeparator;
    }
    if c >= 14 && c <= 27 {
        return CharacterType::BoundaryNeutral;
    }
    if c >= 28 && c <= 30 {
        return CharacterType::ParagraphSeparator;
    }
    if c == 31 {
        return CharacterType::SegmentSeparator;
    }
    if c == 32 {
        return CharacterType::Whitespace;
    }
    if c >= 33 && c <= 34 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 35 && c <= 37 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 38 && c <= 42 {
        return CharacterType::OtherNeutrals;
    }
    if c == 43 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c == 44 {
        return CharacterType::CommonNumberSeparator;
    }
    if c == 45 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c >= 46 && c <= 47 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 48 && c <= 57 {
        return CharacterType::EuropeanNumber;
    }
    if c == 58 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 59 && c <= 126 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 127 && c <= 132 {
        return CharacterType::BoundaryNeutral;
    }
    if c == 133 {
        return CharacterType::ParagraphSeparator;
    }
    if c >= 134 && c <= 159 {
        return CharacterType::BoundaryNeutral;
    }
    if c == 160 {
        return CharacterType::CommonNumberSeparator;
    }
    if c == 161 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 162 && c <= 165 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 166 && c <= 172 {
        return CharacterType::OtherNeutrals;
    }
    if c == 173 {
        return CharacterType::BoundaryNeutral;
    }
    if c >= 174 && c <= 175 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 176 && c <= 177 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 178 && c <= 179 {
        return CharacterType::EuropeanNumber;
    }
    if c >= 180 && c <= 184 {
        return CharacterType::OtherNeutrals;
    }
    if c == 185 {
        return CharacterType::EuropeanNumber;
    }
    if c >= 187 && c <= 767 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 768 && c <= 879 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 884 && c <= 1014 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 1155 && c <= 1161 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1418 && c <= 1422 {
        return CharacterType::OtherNeutrals;
    }
    if c == 1423 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 1425 && c <= 1469 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1470 {
        return CharacterType::RightToLeft;
    }
    if c == 1471 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1472 {
        return CharacterType::RightToLeft;
    }
    if c >= 1473 && c <= 1474 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1475 {
        return CharacterType::RightToLeft;
    }
    if c >= 1476 && c <= 1477 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1478 {
        return CharacterType::RightToLeft;
    }
    if c == 1479 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1488 && c <= 1524 {
        return CharacterType::RightToLeft;
    }
    if c >= 1536 && c <= 1541 {
        return CharacterType::PersianNumber;
    }
    if c >= 1542 && c <= 1543 {
        return CharacterType::OtherNeutrals;
    }
    if c == 1544 {
        return CharacterType::PersianLetter;
    }
    if c >= 1545 && c <= 1546 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c == 1547 {
        return CharacterType::PersianLetter;
    }
    if c == 1548 {
        return CharacterType::CommonNumberSeparator;
    }
    if c == 1549 {
        return CharacterType::PersianLetter;
    }
    if c >= 1550 && c <= 1551 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 1552 && c <= 1562 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1563 && c <= 1610 {
        return CharacterType::PersianLetter;
    }
    if c >= 1611 && c <= 1631 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1632 && c <= 1641 {
        return CharacterType::PersianNumber;
    }
    if c == 1642 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 1643 && c <= 1644 {
        return CharacterType::PersianNumber;
    }
    if c >= 1645 && c <= 1647 {
        return CharacterType::PersianLetter;
    }
    if c == 1648 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1649 && c <= 1749 {
        return CharacterType::PersianLetter;
    }
    if c >= 1750 && c <= 1756 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1757 {
        return CharacterType::PersianNumber;
    }
    if c == 1758 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 1759 && c <= 1764 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1765 && c <= 1766 {
        return CharacterType::PersianLetter;
    }
    if c >= 1767 && c <= 1768 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1769 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 1770 && c <= 1773 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1774 && c <= 1775 {
        return CharacterType::PersianLetter;
    }
    if c >= 1776 && c <= 1785 {
        return CharacterType::EuropeanNumber;
    }
    if c >= 1786 && c <= 1808 {
        return CharacterType::PersianLetter;
    }
    if c == 1809 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1810 && c <= 1839 {
        return CharacterType::PersianLetter;
    }
    if c >= 1840 && c <= 1866 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 1869 && c <= 1957 {
        return CharacterType::PersianLetter;
    }
    if c >= 1958 && c <= 1968 {
        return CharacterType::NonSpacingMark;
    }
    if c == 1969 {
        return CharacterType::PersianLetter;
    }
    if c >= 1984 && c <= 2026 {
        return CharacterType::RightToLeft;
    }
    if c >= 2027 && c <= 2035 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 2036 && c <= 2037 {
        return CharacterType::RightToLeft;
    }
    if c >= 2038 && c <= 2041 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 2042 && c <= 2069 {
        return CharacterType::RightToLeft;
    }
    if c >= 2070 && c <= 2073 {
        return CharacterType::NonSpacingMark;
    }
    if c == 2074 {
        return CharacterType::RightToLeft;
    }
    if c >= 2075 && c <= 2083 {
        return CharacterType::NonSpacingMark;
    }
    if c == 2084 {
        return CharacterType::RightToLeft;
    }
    if c >= 2085 && c <= 2087 {
        return CharacterType::NonSpacingMark;
    }
    if c == 2088 {
        return CharacterType::RightToLeft;
    }
    if c >= 2089 && c <= 2093 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 2096 && c <= 2136 {
        return CharacterType::RightToLeft;
    }
    if c >= 2137 && c <= 2139 {
        return CharacterType::NonSpacingMark;
    }
    if c == 2142 {
        return CharacterType::RightToLeft;
    }
    if c >= 2208 && c <= 2226 {
        return CharacterType::PersianLetter;
    }
    if c >= 2276 && c <= 2531 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 2546 && c <= 2555 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 2561 && c <= 2787 {
        return CharacterType::NonSpacingMark;
    }
    if c == 2801 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 2817 && c <= 3021 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 3059 && c <= 3064 {
        return CharacterType::OtherNeutrals;
    }
    if c == 3065 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c == 3066 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 3072 && c <= 3171 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 3192 && c <= 3198 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 3201 && c <= 3642 {
        return CharacterType::NonSpacingMark;
    }
    if c == 3647 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 3655 && c <= 3897 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 3898 && c <= 3901 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 3953 && c <= 4959 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 5008 && c <= 5120 {
        return CharacterType::OtherNeutrals;
    }
    if c == 5760 {
        return CharacterType::Whitespace;
    }
    if c >= 5787 && c <= 5788 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 5906 && c <= 6099 {
        return CharacterType::NonSpacingMark;
    }
    if c == 6107 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c == 6109 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 6128 && c <= 6154 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 6155 && c <= 6157 {
        return CharacterType::NonSpacingMark;
    }
    if c == 6158 {
        return CharacterType::BoundaryNeutral;
    }
    if c >= 6313 && c <= 6459 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 6464 && c <= 6655 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 6679 && c <= 7679 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 8125 && c <= 8190 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 8192 && c <= 8202 {
        return CharacterType::Whitespace;
    }
    if c >= 8203 && c <= 8205 {
        return CharacterType::BoundaryNeutral;
    }
    if c == 8207 {
        return CharacterType::RightToLeft;
    }
    if c >= 8208 && c <= 8231 {
        return CharacterType::OtherNeutrals;
    }
    if c == 8232 {
        return CharacterType::Whitespace;
    }
    if c == 8233 {
        return CharacterType::ParagraphSeparator;
    }
    if c == 8234 {
        return CharacterType::LeftToRightEmbedding;
    }
    if c == 8235 {
        return CharacterType::RightToLeftEmbedding;
    }
    if c == 8236 {
        return CharacterType::PopDirectionalFormat;
    }
    if c == 8237 {
        return CharacterType::LeftToRightOverride;
    }
    if c == 8238 {
        return CharacterType::RightToLeftOverride;
    }
    if c == 8239 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 8240 && c <= 8244 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 8245 && c <= 8259 {
        return CharacterType::OtherNeutrals;
    }
    if c == 8260 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 8261 && c <= 8286 {
        return CharacterType::OtherNeutrals;
    }
    if c == 8287 {
        return CharacterType::Whitespace;
    }
    if c >= 8288 && c <= 8292 {
        return CharacterType::BoundaryNeutral;
    }
    if c == 8294 {
        return CharacterType::LeftToRightIsolate;
    }
    if c == 8295 {
        return CharacterType::RightToLeftIsolate;
    }
    if c == 8296 {
        return CharacterType::FirstStrongIsolate;
    }
    if c == 8297 {
        return CharacterType::PopDirectionalIsolate;
    }
    if c >= 8298 && c <= 8303 {
        return CharacterType::BoundaryNeutral;
    }
    if c >= 8304 && c <= 8313 {
        return CharacterType::EuropeanNumber;
    }
    if c >= 8314 && c <= 8315 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c >= 8316 && c <= 8318 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 8320 && c <= 8329 {
        return CharacterType::EuropeanNumber;
    }
    if c >= 8330 && c <= 8331 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c >= 8332 && c <= 8334 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 8352 && c <= 8381 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 8400 && c <= 8432 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 8448 && c <= 8489 {
        return CharacterType::OtherNeutrals;
    }
    if c == 8494 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 8506 && c <= 8721 {
        return CharacterType::OtherNeutrals;
    }
    if c == 8722 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c == 8723 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 8724 && c <= 9351 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 9352 && c <= 9371 {
        return CharacterType::EuropeanNumber;
    }
    if c >= 9450 && c <= 11498 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 11503 && c <= 11505 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 11513 && c <= 11519 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 11647 && c <= 11775 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 11776 && c <= 12283 {
        return CharacterType::OtherNeutrals;
    }
    if c == 12288 {
        return CharacterType::Whitespace;
    }
    if c >= 12289 && c <= 12320 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 12330 && c <= 12333 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 12336 && c <= 12351 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 12441 && c <= 12442 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 12443 && c <= 42511 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 42607 && c <= 42610 {
        return CharacterType::NonSpacingMark;
    }
    if c == 42611 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 42612 && c <= 42621 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 42622 && c <= 42623 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 42655 && c <= 42737 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 42752 && c <= 42888 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 43010 && c <= 43046 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 43048 && c <= 43051 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 43064 && c <= 43065 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 43124 && c <= 43127 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 43204 && c <= 44013 {
        return CharacterType::NonSpacingMark;
    }
    if c == 64285 {
        return CharacterType::RightToLeft;
    }
    if c == 64286 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 64287 && c <= 64296 {
        return CharacterType::RightToLeft;
    }
    if c == 64297 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c >= 64298 && c <= 64335 {
        return CharacterType::RightToLeft;
    }
    if c >= 64336 && c <= 64829 {
        return CharacterType::PersianLetter;
    }
    if c >= 64830 && c <= 64831 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 64848 && c <= 65020 {
        return CharacterType::PersianLetter;
    }
    if c == 65021 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65024 && c <= 65039 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 65040 && c <= 65049 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65056 && c <= 65069 {
        return CharacterType::NonSpacingMark;
    }
    if c >= 65072 && c <= 65103 {
        return CharacterType::OtherNeutrals;
    }
    if c == 65104 {
        return CharacterType::CommonNumberSeparator;
    }
    if c == 65105 {
        return CharacterType::OtherNeutrals;
    }
    if c == 65106 {
        return CharacterType::CommonNumberSeparator;
    }
    if c == 65108 {
        return CharacterType::OtherNeutrals;
    }
    if c == 65109 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 65110 && c <= 65118 {
        return CharacterType::OtherNeutrals;
    }
    if c == 65119 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 65120 && c <= 65121 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65122 && c <= 65123 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c >= 65124 && c <= 65128 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65129 && c <= 65130 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c == 65131 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65136 && c <= 65276 {
        return CharacterType::PersianLetter;
    }
    if c == 65279 {
        return CharacterType::BoundaryNeutral;
    }
    if c >= 65281 && c <= 65282 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65283 && c <= 65285 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 65286 && c <= 65290 {
        return CharacterType::OtherNeutrals;
    }
    if c == 65291 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c == 65292 {
        return CharacterType::CommonNumberSeparator;
    }
    if c == 65293 {
        return CharacterType::EuropeanNumberSeparator;
    }
    if c >= 65294 && c <= 65295 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 65296 && c <= 65305 {
        return CharacterType::EuropeanNumber;
    }
    if c == 65306 {
        return CharacterType::CommonNumberSeparator;
    }
    if c >= 65307 && c <= 65381 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65504 && c <= 65505 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 65506 && c <= 65508 {
        return CharacterType::OtherNeutrals;
    }
    if c >= 65509 && c <= 65510 {
        return CharacterType::EuropeanNumberTerminator;
    }
    if c >= 65512 && c <= 65533 {
        return CharacterType::OtherNeutrals;
    }

    return CharacterType::LeftToRight;
}
