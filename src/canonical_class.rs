#[derive(PartialEq, Debug)]
enum CanonicalClass {
    /// Not Reordered (NR).
    /// Spacing, split, enclosing, reordrant, and Tibetan subjoined.
    NotReordered = 0,

    /// Overlays and interior (OV).
    OverlaysInterior = 1,

    /// Nuktas (NK).
    Nuktas = 7,

    /// Hiragana/Katakana voicing marks (KV).
    KatakanaVoicing = 8,

    /// Viramas (VR).
    Viramas = 9,

    /// General class level 10.
    Class10 = 10,

    /// General class level 11.
    Class11 = 11,

    /// General class level 12.
    Class12 = 12,

    /// General class level 13.
    Class13 = 13,

    /// General class level 14.
    Class14 = 14,

    /// General class level 15.
    Class15 = 15,

    /// General class level 16.
    Class16 = 16,

    /// General class level 17.
    Class17 = 17,

    /// General class level 18.
    Class18 = 18,

    /// General class level 19.
    Class19 = 19,

    /// General class level 20.
    Class20 = 20,

    /// General class level 21.
    Class21 = 21,

    /// General class level 22.
    Class22 = 22,

    /// General class level 23.
    Class23 = 23,

    /// General class level 24.
    Class24 = 24,

    /// General class level 25.
    Class25 = 25,

    /// General class level 26.
    Class26 = 26,

    /// General class level 27.
    Class27 = 27,

    /// General class level 28.
    Class28 = 28,

    /// General class level 29.
    Class29 = 29,

    /// General class level 30.
    Class30 = 30,

    /// General class level 31.
    Class31 = 31,

    /// General class level 32.
    Class32 = 32,

    /// General class level 33.
    Class33 = 33,

    /// General class level 34.
    Class34 = 34,

    /// General class level 35.
    Class35 = 35,

    /// General class level 36.
    Class36 = 36,

    // General class level 37.
    // Class37 = 37,

    // General class level 38.
    // Class38 = 38,

    // General class level 39.
    // Class39 = 39,

    // General class level 40.
    // Class40 = 40,

    // General class level 41.
    // Class41 = 41,

    // General class level 42.
    // Class42 = 42,

    // General class level 43.
    // Class43 = 43,

    // General class level 44.
    // Class44 = 44,

    // General class level 45.
    // Class45 = 45,

    // General class level 46.
    // Class46 = 46,

    // General class level 47.
    // Class47 = 47,

    // General class level 48.
    // Class48 = 48,

    // General class level 49.
    // Class49 = 49,

    // General class level 50.
    // Class50 = 50,

    // General class level 51.
    // Class51 = 51,

    // General class level 52.
    // Class52 = 52,

    // General class level 53.
    // Class53 = 53,

    // General class level 54.
    // Class54 = 54,

    // General class level 55.
    // Class55 = 55,

    // General class level 56.
    // Class56 = 56,

    // General class level 57.
    // Class57 = 57,

    // General class level 58.
    // Class58 = 58,

    // General class level 59.
    // Class59 = 59,

    // General class level 60.
    // Class60 = 60,

    // General class level 61.
    // Class61 = 61,

    // General class level 62.
    // Class62 = 62,

    // General class level 63.
    // Class63 = 63,

    // General class level 64.
    // Class64 = 64,

    // General class level 65.
    // Class65 = 65,

    // General class level 66.
    // Class66 = 66,

    // General class level 67.
    // Class67 = 67,

    // General class level 68.
    // Class68 = 68,

    // General class level 69.
    // Class69 = 69,

    // General class level 70.
    // Class70 = 70,

    // General class level 71.
    // Class71 = 71,

    // General class level 72.
    // Class72 = 72,

    // General class level 73.
    // Class73 = 73,

    // General class level 74.
    // Class74 = 74,

    // General class level 75.
    // Class75 = 75,

    // General class level 76.
    // Class76 = 76,

    // General class level 77.
    // Class77 = 77,

    // General class level 78.
    // Class78 = 78,

    // General class level 79.
    // Class79 = 79,

    // General class level 80.
    // Class80 = 80,

    // General class level 81.
    // Class81 = 81,

    // General class level 82.
    // Class82 = 82,

    // General class level 83.
    // Class83 = 83,
    /// General class level 84.
    Class84 = 84,

    // General class level 85.
    // Class85 = 85,

    // General class level 86.
    // Class86 = 86,

    // General class level 87.
    // Class87 = 87,

    // General class level 88.
    // Class88 = 88,

    // General class level 89.
    // Class89 = 89,

    // General class level 90.
    // Class90 = 90,
    /// General class level 91.
    Class91 = 91,

    // General class level 92.
    // Class92 = 92,

    // General class level 93.
    // Class93 = 93,

    // General class level 94.
    // Class94 = 94,

    // General class level 95.
    // Class95 = 95,

    // General class level 96.
    // Class96 = 96,

    // General class level 97.
    // Class97 = 97,

    // General class level 98.
    // Class98 = 98,

    // General class level 99.
    // Class99 = 99,

    // General class level 100.
    // Class100 = 100,

    // General class level 101.
    // Class101 = 101,

    // General class level 102.
    // Class102 = 102,
    /// General class level 103.
    Class103 = 103,

    // General class level 104.
    // Class104 = 104,

    // General class level 105.
    // Class105 = 105,

    // General class level 106.
    // Class106 = 106,
    /// General class level 107.
    Class107 = 107,

    // General class level 108.
    // Class108 = 108,

    // General class level 109.
    // Class109 = 109,

    // General class level 110.
    // Class110 = 110,

    // General class level 111.
    // Class111 = 111,

    // General class level 112.
    // Class112 = 112,

    // General class level 113.
    // Class113 = 113,

    // General class level 114.
    // Class114 = 114,

    // General class level 115.
    // Class115 = 115,

    // General class level 116.
    // Class116 = 116,

    // General class level 117.
    // Class117 = 117,
    /// General class level 118.
    Class118 = 118,

    // General class level 119.
    // Class119 = 119,

    // General class level 120.
    // Class120 = 120,

    // General class level 121.
    // Class121 = 121,

    // General class level 122.
    // Class122 = 122,

    // General class level 123.
    // Class123 = 123,

    // General class level 124.
    // Class124 = 124,

    // General class level 125.
    // Class125 = 125,

    // General class level 126.
    // Class126 = 126,

    // General class level 127.
    // Class127 = 127,

    // General class level 128.
    // Class128 = 128,
    /// General class level 129.
    Class129 = 129,

    /// General class level 130.
    Class130 = 130,

    // General class level 131.
    // Class131 = 131,
    /// General class level 132.
    Class132 = 132,

    // General class level 133.
    // Class133 = 133,

    // General class level 134.
    // Class134 = 134,

    // General class level 135.
    // Class135 = 135,

    // General class level 136.
    // Class136 = 136,

    // General class level 137.
    // Class137 = 137,

    // General class level 138.
    // Class138 = 138,

    // General class level 139.
    // Class139 = 139,

    // General class level 140.
    // Class140 = 140,

    // General class level 141.
    // Class141 = 141,

    // General class level 142.
    // Class142 = 142,

    // General class level 143.
    // Class143 = 143,

    // General class level 144.
    // Class144 = 144,

    // General class level 145.
    // Class145 = 145,

    // General class level 146.
    // Class146 = 146,

    // General class level 147.
    // Class147 = 147,

    // General class level 148.
    // Class148 = 148,

    // General class level 149.
    // Class149 = 149,

    // General class level 150.
    // Class150 = 150,

    // General class level 151.
    // Class151 = 151,

    // General class level 152.
    // Class152 = 152,

    // General class level 153.
    // Class153 = 153,

    // General class level 154.
    // Class154 = 154,

    // General class level 155.
    // Class155 = 155,

    // General class level 156.
    // Class156 = 156,

    // General class level 157.
    // Class157 = 157,

    // General class level 158.
    // Class158 = 158,

    // General class level 159.
    // Class159 = 159,

    // General class level 160.
    // Class160 = 160,

    // General class level 161.
    // Class161 = 161,
    /// General class level 162.
    Class162 = 122,

    // General class level 163.
    // Class163 = 166,

    // General class level 164.
    // Class164 = 164,

    // General class level 165.
    // Class165 = 165,

    // General class level 166.
    // Class166 = 166,

    // General class level 167.
    // Class167 = 167,

    // General class level 168.
    // Class168 = 168,

    // General class level 169.
    // Class169 = 169,

    // General class level 170.
    // Class170 = 170,

    // General class level 171.
    // Class171 = 171,

    // General class level 172.
    // Class172 = 172,

    // General class level 173.
    // Class173 = 173,

    // General class level 174.
    // Class174 = 174,

    // General class level 175.
    // Class175 = 175,

    // General class level 176.
    // Class176 = 176,

    // General class level 177.
    // Class177 = 177,

    // General class level 178.
    // Class178 = 178,

    // General class level 179.
    // Class179 = 179,

    // General class level 180.
    // Class180 = 180,

    // General class level 181.
    // Class181 = 181,

    // General class level 182.
    // Class182 = 182,

    // General class level 183.
    // Class183 = 183,

    // General class level 184.
    // Class184 = 184,

    // General class level 185.
    // Class185 = 185,

    // General class level 186.
    // Class186 = 186,

    // General class level 187.
    // Class187 = 187,

    // General class level 188.
    // Class188 = 188,

    // General class level 189.
    // Class189 = 189,

    // General class level 190.
    // Class190 = 190,

    // General class level 191.
    // Class191 = 191,

    // General class level 192.
    // Class192 = 192,

    // General class level 193.
    // Class193 = 193,

    // General class level 194.
    // Class194 = 194,

    // General class level 195.
    // Class195 = 195,

    // General class level 196.
    // Class196 = 196,

    // General class level 197.
    // Class197 = 197,

    // General class level 198.
    // Class198 = 198,

    // General class level 199.
    // Class199 = 199,

    // Attached Below Left (ATBL).
    // AttachedBelowLeft = 200,

    /// Attached Below (ATB).
    AttachedBelow = 202,

    // Attached Below Right (ATBR).
    // AttachedBelowRight = 204,

    // Attached Left (ATL).
    // AttachedLeft = 208,

    // Attached Right (ATR).
    // AttachedRight = 210,

    // Attached Above Left (ATAL).
    // AttachedAboveLeft = 212,

    /// Attached Above (ATA).
    AttachedAbove = 214,

    /// Attached Above Right (ATAR).
    AttachedAboveRight = 216,

    /// Below Left (BL).
    BelowLeft = 218,

    /// Below (B).
    Below = 220,

    /// Below Right (BR).
    BelowRight = 222,

    /// Left (L).
    Left = 224,

    // Right (R).
    // Right = 226,
    /// Above Left (AL).
    AboveLeft = 228,

    /// Above (A).
    Above = 230,

    /// Above Right (AR).
    AboveRight = 232,

    /// Double Below (DB).
    DoubleBelow = 233,

    /// Double Above (DA).
    DoubleAbove = 234,

    /// Iota Subscript (IS).
    IotaSubscript = 240,
}

/// Returns the Unicode canonical class for a given character.
///
/// [c] A Unicode character for which to get the Unicode canonical class.
///
/// Returns the character Unicode canonical class.
fn get_canonical_class(c: u16) -> CanonicalClass {
    if /*c >= 0 &&*/ c <= 788 {
        return CanonicalClass::Above;
    }
    if c == 789 {
        return CanonicalClass::AboveRight;
    }
    if c >= 790 && c <= 793 {
        return CanonicalClass::Below;
    }
    if c == 794 {
        return CanonicalClass::AboveRight;
    }
    if c == 795 {
        return CanonicalClass::AttachedAboveRight;
    }
    if c >= 796 && c <= 800 {
        return CanonicalClass::Below;
    }
    if c >= 801 && c <= 802 {
        return CanonicalClass::AttachedBelow;
    }
    if c >= 803 && c <= 806 {
        return CanonicalClass::Below;
    }
    if c >= 807 && c <= 808 {
        return CanonicalClass::AttachedBelow;
    }
    if c >= 809 && c <= 819 {
        return CanonicalClass::Below;
    }
    if c >= 820 && c <= 824 {
        return CanonicalClass::OverlaysInterior;
    }
    if c >= 825 && c <= 828 {
        return CanonicalClass::Below;
    }
    if c >= 829 && c <= 836 {
        return CanonicalClass::Above;
    }
    if c == 837 {
        return CanonicalClass::IotaSubscript;
    }
    if c == 838 {
        return CanonicalClass::Above;
    }
    if c >= 839 && c <= 841 {
        return CanonicalClass::Below;
    }
    if c >= 842 && c <= 844 {
        return CanonicalClass::Above;
    }
    if c >= 845 && c <= 846 {
        return CanonicalClass::Below;
    }
    if c >= 848 && c <= 850 {
        return CanonicalClass::Above;
    }
    if c >= 851 && c <= 854 {
        return CanonicalClass::Below;
    }
    if c == 855 {
        return CanonicalClass::Above;
    }
    if c == 856 {
        return CanonicalClass::AboveRight;
    }
    if c >= 857 && c <= 858 {
        return CanonicalClass::Below;
    }
    if c == 859 {
        return CanonicalClass::Above;
    }
    if c == 860 {
        return CanonicalClass::DoubleBelow;
    }
    if c >= 861 && c <= 862 {
        return CanonicalClass::DoubleAbove;
    }
    if c == 863 {
        return CanonicalClass::DoubleBelow;
    }
    if c >= 864 && c <= 865 {
        return CanonicalClass::DoubleAbove;
    }
    if c == 866 {
        return CanonicalClass::DoubleBelow;
    }
    if c >= 867 && c <= 1159 {
        return CanonicalClass::Above;
    }
    if c == 1425 {
        return CanonicalClass::Below;
    }
    if c >= 1426 && c <= 1429 {
        return CanonicalClass::Above;
    }
    if c == 1430 {
        return CanonicalClass::Below;
    }
    if c >= 1431 && c <= 1433 {
        return CanonicalClass::Above;
    }
    if c == 1434 {
        return CanonicalClass::BelowRight;
    }
    if c == 1435 {
        return CanonicalClass::Below;
    }
    if c >= 1436 && c <= 1441 {
        return CanonicalClass::Above;
    }
    if c >= 1442 && c <= 1447 {
        return CanonicalClass::Below;
    }
    if c >= 1448 && c <= 1449 {
        return CanonicalClass::Above;
    }
    if c == 1450 {
        return CanonicalClass::Below;
    }
    if c >= 1451 && c <= 1452 {
        return CanonicalClass::Above;
    }
    if c == 1453 {
        return CanonicalClass::BelowRight;
    }
    if c == 1454 {
        return CanonicalClass::AboveLeft;
    }
    if c == 1455 {
        return CanonicalClass::Above;
    }
    if c == 1456 {
        return CanonicalClass::Class10;
    }
    if c == 1457 {
        return CanonicalClass::Class11;
    }
    if c == 1458 {
        return CanonicalClass::Class12;
    }
    if c == 1459 {
        return CanonicalClass::Class13;
    }
    if c == 1460 {
        return CanonicalClass::Class14;
    }
    if c == 1461 {
        return CanonicalClass::Class15;
    }
    if c == 1462 {
        return CanonicalClass::Class16;
    }
    if c == 1463 {
        return CanonicalClass::Class17;
    }
    if c == 1464 {
        return CanonicalClass::Class18;
    }
    if c >= 1465 && c <= 1466 {
        return CanonicalClass::Class19;
    }
    if c == 1467 {
        return CanonicalClass::Class20;
    }
    if c == 1468 {
        return CanonicalClass::Class21;
    }
    if c == 1469 {
        return CanonicalClass::Class22;
    }
    if c == 1471 {
        return CanonicalClass::Class23;
    }
    if c == 1473 {
        return CanonicalClass::Class24;
    }
    if c == 1474 {
        return CanonicalClass::Class25;
    }
    if c == 1476 {
        return CanonicalClass::Above;
    }
    if c == 1477 {
        return CanonicalClass::Below;
    }
    if c == 1479 {
        return CanonicalClass::Class18;
    }
    if c >= 1552 && c <= 1559 {
        return CanonicalClass::Above;
    }
    if c == 1560 {
        return CanonicalClass::Class30;
    }
    if c == 1561 {
        return CanonicalClass::Class31;
    }
    if c == 1562 {
        return CanonicalClass::Class32;
    }
    if c == 1611 {
        return CanonicalClass::Class27;
    }
    if c == 1612 {
        return CanonicalClass::Class28;
    }
    if c == 1613 {
        return CanonicalClass::Class29;
    }
    if c == 1614 {
        return CanonicalClass::Class30;
    }
    if c == 1615 {
        return CanonicalClass::Class31;
    }
    if c == 1616 {
        return CanonicalClass::Class32;
    }
    if c == 1617 {
        return CanonicalClass::Class33;
    }
    if c == 1618 {
        return CanonicalClass::Class34;
    }
    if c >= 1619 && c <= 1620 {
        return CanonicalClass::Above;
    }
    if c >= 1621 && c <= 1622 {
        return CanonicalClass::Below;
    }
    if c >= 1623 && c <= 1627 {
        return CanonicalClass::Above;
    }
    if c == 1628 {
        return CanonicalClass::Below;
    }
    if c >= 1629 && c <= 1630 {
        return CanonicalClass::Above;
    }
    if c == 1631 {
        return CanonicalClass::Below;
    }
    if c == 1648 {
        return CanonicalClass::Class35;
    }
    if c >= 1750 && c <= 1762 {
        return CanonicalClass::Above;
    }
    if c == 1763 {
        return CanonicalClass::Below;
    }
    if c >= 1764 && c <= 1768 {
        return CanonicalClass::Above;
    }
    if c == 1770 {
        return CanonicalClass::Below;
    }
    if c >= 1771 && c <= 1772 {
        return CanonicalClass::Above;
    }
    if c == 1773 {
        return CanonicalClass::Below;
    }
    if c == 1809 {
        return CanonicalClass::Class36;
    }
    if c == 1840 {
        return CanonicalClass::Above;
    }
    if c == 1841 {
        return CanonicalClass::Below;
    }
    if c >= 1842 && c <= 1843 {
        return CanonicalClass::Above;
    }
    if c == 1844 {
        return CanonicalClass::Below;
    }
    if c >= 1845 && c <= 1846 {
        return CanonicalClass::Above;
    }
    if c >= 1847 && c <= 1849 {
        return CanonicalClass::Below;
    }
    if c == 1850 {
        return CanonicalClass::Above;
    }
    if c >= 1851 && c <= 1852 {
        return CanonicalClass::Below;
    }
    if c == 1853 {
        return CanonicalClass::Above;
    }
    if c == 1854 {
        return CanonicalClass::Below;
    }
    if c >= 1855 && c <= 1857 {
        return CanonicalClass::Above;
    }
    if c == 1858 {
        return CanonicalClass::Below;
    }
    if c == 1859 {
        return CanonicalClass::Above;
    }
    if c == 1860 {
        return CanonicalClass::Below;
    }
    if c == 1861 {
        return CanonicalClass::Above;
    }
    if c == 1862 {
        return CanonicalClass::Below;
    }
    if c == 1863 {
        return CanonicalClass::Above;
    }
    if c == 1864 {
        return CanonicalClass::Below;
    }
    if c >= 1865 && c <= 2033 {
        return CanonicalClass::Above;
    }
    if c == 2034 {
        return CanonicalClass::Below;
    }
    if c >= 2035 && c <= 2093 {
        return CanonicalClass::Above;
    }
    if c >= 2137 && c <= 2139 {
        return CanonicalClass::Below;
    }
    if c >= 2276 && c <= 2277 {
        return CanonicalClass::Above;
    }
    if c == 2278 {
        return CanonicalClass::Below;
    }
    if c >= 2279 && c <= 2280 {
        return CanonicalClass::Above;
    }
    if c == 2281 {
        return CanonicalClass::Below;
    }
    if c >= 2282 && c <= 2284 {
        return CanonicalClass::Above;
    }
    if c >= 2285 && c <= 2287 {
        return CanonicalClass::Below;
    }
    if c == 2288 {
        return CanonicalClass::Class27;
    }
    if c == 2289 {
        return CanonicalClass::Class28;
    }
    if c == 2290 {
        return CanonicalClass::Class29;
    }
    if c >= 2291 && c <= 2293 {
        return CanonicalClass::Above;
    }
    if c == 2294 {
        return CanonicalClass::Below;
    }
    if c >= 2295 && c <= 2296 {
        return CanonicalClass::Above;
    }
    if c >= 2297 && c <= 2298 {
        return CanonicalClass::Below;
    }
    if c >= 2299 && c <= 2303 {
        return CanonicalClass::Above;
    }
    if c == 2364 {
        return CanonicalClass::Nuktas;
    }
    if c == 2381 {
        return CanonicalClass::Viramas;
    }
    if c == 2385 {
        return CanonicalClass::Above;
    }
    if c == 2386 {
        return CanonicalClass::Below;
    }
    if c >= 2387 && c <= 2388 {
        return CanonicalClass::Above;
    }
    if c == 2492 {
        return CanonicalClass::Nuktas;
    }
    if c == 2509 {
        return CanonicalClass::Viramas;
    }
    if c == 2620 {
        return CanonicalClass::Nuktas;
    }
    if c == 2637 {
        return CanonicalClass::Viramas;
    }
    if c == 2748 {
        return CanonicalClass::Nuktas;
    }
    if c == 2765 {
        return CanonicalClass::Viramas;
    }
    if c == 2876 {
        return CanonicalClass::Nuktas;
    }
    if c >= 2893 && c <= 3149 {
        return CanonicalClass::Viramas;
    }
    if c == 3157 {
        return CanonicalClass::Class84;
    }
    if c == 3158 {
        return CanonicalClass::Class91;
    }
    if c == 3260 {
        return CanonicalClass::Nuktas;
    }
    if c >= 3277 && c <= 3530 {
        return CanonicalClass::Viramas;
    }
    if c >= 3640 && c <= 3641 {
        return CanonicalClass::Class103;
    }
    if c == 3642 {
        return CanonicalClass::Viramas;
    }
    if c >= 3656 && c <= 3659 {
        return CanonicalClass::Class107;
    }
    if c >= 3768 && c <= 3769 {
        return CanonicalClass::Class118;
    }
    if c >= 3784 && c <= 3787 {
        return CanonicalClass::Class162;
    }
    if c >= 3864 && c <= 3895 {
        return CanonicalClass::Below;
    }
    if c == 3897 {
        return CanonicalClass::AttachedAboveRight;
    }
    if c == 3953 {
        return CanonicalClass::Class129;
    }
    if c == 3954 {
        return CanonicalClass::Class130;
    }
    if c == 3956 {
        return CanonicalClass::Class132;
    }
    if c >= 3962 && c <= 3968 {
        return CanonicalClass::Class130;
    }
    if c >= 3970 && c <= 3971 {
        return CanonicalClass::Above;
    }
    if c == 3972 {
        return CanonicalClass::Viramas;
    }
    if c >= 3974 && c <= 3975 {
        return CanonicalClass::Above;
    }
    if c == 4038 {
        return CanonicalClass::Below;
    }
    if c == 4151 {
        return CanonicalClass::Nuktas;
    }
    if c >= 4153 && c <= 4154 {
        return CanonicalClass::Viramas;
    }
    if c == 4237 {
        return CanonicalClass::Below;
    }
    if c >= 4957 && c <= 4959 {
        return CanonicalClass::Above;
    }
    if c >= 5908 && c <= 6098 {
        return CanonicalClass::Viramas;
    }
    if c == 6109 {
        return CanonicalClass::Above;
    }
    if c == 6313 {
        return CanonicalClass::AboveLeft;
    }
    if c == 6457 {
        return CanonicalClass::BelowRight;
    }
    if c == 6458 {
        return CanonicalClass::Above;
    }
    if c == 6459 {
        return CanonicalClass::Below;
    }
    if c == 6679 {
        return CanonicalClass::Above;
    }
    if c == 6680 {
        return CanonicalClass::Below;
    }
    if c == 6752 {
        return CanonicalClass::Viramas;
    }
    if c >= 6773 && c <= 6780 {
        return CanonicalClass::Above;
    }
    if c == 6783 {
        return CanonicalClass::Below;
    }
    if c >= 6832 && c <= 6836 {
        return CanonicalClass::Above;
    }
    if c >= 6837 && c <= 6842 {
        return CanonicalClass::Below;
    }
    if c >= 6843 && c <= 6844 {
        return CanonicalClass::Above;
    }
    if c == 6845 {
        return CanonicalClass::Below;
    }
    if c == 6964 {
        return CanonicalClass::Nuktas;
    }
    if c == 6980 {
        return CanonicalClass::Viramas;
    }
    if c == 7019 {
        return CanonicalClass::Above;
    }
    if c == 7020 {
        return CanonicalClass::Below;
    }
    if c >= 7021 && c <= 7027 {
        return CanonicalClass::Above;
    }
    if c >= 7082 && c <= 7083 {
        return CanonicalClass::Viramas;
    }
    if c == 7142 {
        return CanonicalClass::Nuktas;
    }
    if c >= 7154 && c <= 7155 {
        return CanonicalClass::Viramas;
    }
    if c == 7223 {
        return CanonicalClass::Nuktas;
    }
    if c >= 7376 && c <= 7378 {
        return CanonicalClass::Above;
    }
    if c == 7380 {
        return CanonicalClass::OverlaysInterior;
    }
    if c >= 7381 && c <= 7385 {
        return CanonicalClass::Below;
    }
    if c >= 7386 && c <= 7387 {
        return CanonicalClass::Above;
    }
    if c >= 7388 && c <= 7391 {
        return CanonicalClass::Below;
    }
    if c == 7392 {
        return CanonicalClass::Above;
    }
    if c >= 7394 && c <= 7400 {
        return CanonicalClass::OverlaysInterior;
    }
    if c == 7405 {
        return CanonicalClass::Below;
    }
    if c >= 7412 && c <= 7617 {
        return CanonicalClass::Above;
    }
    if c == 7618 {
        return CanonicalClass::Below;
    }
    if c >= 7619 && c <= 7625 {
        return CanonicalClass::Above;
    }
    if c == 7626 {
        return CanonicalClass::Below;
    }
    if c >= 7627 && c <= 7628 {
        return CanonicalClass::Above;
    }
    if c == 7629 {
        return CanonicalClass::DoubleAbove;
    }
    if c == 7630 {
        return CanonicalClass::AttachedAbove;
    }
    if c == 7631 {
        return CanonicalClass::Below;
    }
    if c == 7632 {
        return CanonicalClass::AttachedBelow;
    }
    if c >= 7633 && c <= 7669 {
        return CanonicalClass::Above;
    }
    if c == 7676 {
        return CanonicalClass::DoubleBelow;
    }
    if c == 7677 {
        return CanonicalClass::Below;
    }
    if c == 7678 {
        return CanonicalClass::Above;
    }
    if c == 7679 {
        return CanonicalClass::Below;
    }
    if c >= 8400 && c <= 8401 {
        return CanonicalClass::Above;
    }
    if c >= 8402 && c <= 8403 {
        return CanonicalClass::OverlaysInterior;
    }
    if c >= 8404 && c <= 8407 {
        return CanonicalClass::Above;
    }
    if c >= 8408 && c <= 8410 {
        return CanonicalClass::OverlaysInterior;
    }
    if c >= 8411 && c <= 8417 {
        return CanonicalClass::Above;
    }
    if c >= 8421 && c <= 8422 {
        return CanonicalClass::OverlaysInterior;
    }
    if c == 8423 {
        return CanonicalClass::Above;
    }
    if c == 8424 {
        return CanonicalClass::Below;
    }
    if c == 8425 {
        return CanonicalClass::Above;
    }
    if c >= 8426 && c <= 8427 {
        return CanonicalClass::OverlaysInterior;
    }
    if c >= 8428 && c <= 8431 {
        return CanonicalClass::Below;
    }
    if c >= 8432 && c <= 11505 {
        return CanonicalClass::Above;
    }
    if c == 11647 {
        return CanonicalClass::Viramas;
    }
    if c >= 11744 && c <= 11775 {
        return CanonicalClass::Above;
    }
    if c == 12330 {
        return CanonicalClass::BelowLeft;
    }
    if c == 12331 {
        return CanonicalClass::AboveLeft;
    }
    if c == 12332 {
        return CanonicalClass::AboveRight;
    }
    if c == 12333 {
        return CanonicalClass::BelowRight;
    }
    if c >= 12334 && c <= 12335 {
        return CanonicalClass::Left;
    }
    if c >= 12441 && c <= 12442 {
        return CanonicalClass::KatakanaVoicing;
    }
    if c >= 42607 && c <= 42737 {
        return CanonicalClass::Above;
    }
    if c >= 43014 && c <= 43204 {
        return CanonicalClass::Viramas;
    }
    if c >= 43232 && c <= 43249 {
        return CanonicalClass::Above;
    }
    if c >= 43307 && c <= 43309 {
        return CanonicalClass::Below;
    }
    if c == 43347 {
        return CanonicalClass::Viramas;
    }
    if c == 43443 {
        return CanonicalClass::Nuktas;
    }
    if c == 43456 {
        return CanonicalClass::Viramas;
    }
    if c >= 43696 && c <= 43699 {
        return CanonicalClass::Above;
    }
    if c == 43700 {
        return CanonicalClass::Below;
    }
    if c >= 43703 && c <= 43713 {
        return CanonicalClass::Above;
    }
    if c >= 43766 && c <= 44013 {
        return CanonicalClass::Viramas;
    }
    if c == 64286 {
        return CanonicalClass::Class26;
    }
    if c >= 65056 && c <= 65062 {
        return CanonicalClass::Above;
    }
    if c >= 0xFE27 && c <= 0xFE2D {
        return CanonicalClass::Above;
    }

    return CanonicalClass::NotReordered;
}
