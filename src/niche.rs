macro_rules! new_byte {
    (
        $(
            $Name:ident($Primitive:ident) { $($Variant:ident = $value:literal,)* }
        )*
    ) => {$(
        #[allow(dead_code)]
        #[derive(Copy, Clone)]
        #[repr($Primitive)]
        enum $Name {
            $($Variant = $value,)*
        }
    )*}
}

macro_rules! new_int {
    (
        $(
            ( $Name:ident $Primitive:ident )
            ( $min:literal ..= $max:literal )
            ( $($LittleField:ident: $LittleType:ident),* )
            ( $($BigField:ident: $BigType:ident),* )
        )*
    ) => {$(
        #[cfg(target_endian = "little")]
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone)]
        #[repr(C)]
        pub(crate) struct $Name {
            align: [core::primitive::$Primitive; 0],
            $($LittleField: $LittleType,)*
        }

        #[cfg(target_endian = "big")]
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone)]
        #[repr(C)]
        pub(crate) struct $Name {
            align: [core::primitive::$Primitive; 0],
            $($BigField: $BigType,)*
        }

        impl $Name {
            pub(crate) const MIN: core::primitive::$Primitive = $min;
            pub(crate) const MAX: core::primitive::$Primitive = $max;
        }
    )*};
}

new_byte! {
    MaxI1(i8) {
        M1 = -1,
        V0 = 0,
    }

    MaxI2(i8) {
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
    }

    MaxI3(i8) {
        M4 = -4,
        M3 = -3,
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
    }

    MaxI4(i8) {
        M8 = -8,
        M7 = -7,
        M6 = -6,
        M5 = -5,
        M4 = -4,
        M3 = -3,
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
    }

    MaxI5(i8) {
        M16 = -16,
        M15 = -15,
        M14 = -14,
        M13 = -13,
        M12 = -12,
        M11 = -11,
        M10 = -10,
        M9 = -9,
        M8 = -8,
        M7 = -7,
        M6 = -6,
        M5 = -5,
        M4 = -4,
        M3 = -3,
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
    }

    MaxI6(i8) {
        M32 = -32,
        M31 = -31,
        M30 = -30,
        M29 = -29,
        M28 = -28,
        M27 = -27,
        M26 = -26,
        M25 = -25,
        M24 = -24,
        M23 = -23,
        M22 = -22,
        M21 = -21,
        M20 = -20,
        M19 = -19,
        M18 = -18,
        M17 = -17,
        M16 = -16,
        M15 = -15,
        M14 = -14,
        M13 = -13,
        M12 = -12,
        M11 = -11,
        M10 = -10,
        M9 = -9,
        M8 = -8,
        M7 = -7,
        M6 = -6,
        M5 = -5,
        M4 = -4,
        M3 = -3,
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
    }

    MaxI7(i8) {
        M64 = -64,
        M63 = -63,
        M62 = -62,
        M61 = -61,
        M60 = -60,
        M59 = -59,
        M58 = -58,
        M57 = -57,
        M56 = -56,
        M55 = -55,
        M54 = -54,
        M53 = -53,
        M52 = -52,
        M51 = -51,
        M50 = -50,
        M49 = -49,
        M48 = -48,
        M47 = -47,
        M46 = -46,
        M45 = -45,
        M44 = -44,
        M43 = -43,
        M42 = -42,
        M41 = -41,
        M40 = -40,
        M39 = -39,
        M38 = -38,
        M37 = -37,
        M36 = -36,
        M35 = -35,
        M34 = -34,
        M33 = -33,
        M32 = -32,
        M31 = -31,
        M30 = -30,
        M29 = -29,
        M28 = -28,
        M27 = -27,
        M26 = -26,
        M25 = -25,
        M24 = -24,
        M23 = -23,
        M22 = -22,
        M21 = -21,
        M20 = -20,
        M19 = -19,
        M18 = -18,
        M17 = -17,
        M16 = -16,
        M15 = -15,
        M14 = -14,
        M13 = -13,
        M12 = -12,
        M11 = -11,
        M10 = -10,
        M9 = -9,
        M8 = -8,
        M7 = -7,
        M6 = -6,
        M5 = -5,
        M4 = -4,
        M3 = -3,
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
        V32 = 32,
        V33 = 33,
        V34 = 34,
        V35 = 35,
        V36 = 36,
        V37 = 37,
        V38 = 38,
        V39 = 39,
        V40 = 40,
        V41 = 41,
        V42 = 42,
        V43 = 43,
        V44 = 44,
        V45 = 45,
        V46 = 46,
        V47 = 47,
        V48 = 48,
        V49 = 49,
        V50 = 50,
        V51 = 51,
        V52 = 52,
        V53 = 53,
        V54 = 54,
        V55 = 55,
        V56 = 56,
        V57 = 57,
        V58 = 58,
        V59 = 59,
        V60 = 60,
        V61 = 61,
        V62 = 62,
        V63 = 63,
    }

    MaxI8(i8) {
        M128 = -128,
        M127 = -127,
        M126 = -126,
        M125 = -125,
        M124 = -124,
        M123 = -123,
        M122 = -122,
        M121 = -121,
        M120 = -120,
        M119 = -119,
        M118 = -118,
        M117 = -117,
        M116 = -116,
        M115 = -115,
        M114 = -114,
        M113 = -113,
        M112 = -112,
        M111 = -111,
        M110 = -110,
        M109 = -109,
        M108 = -108,
        M107 = -107,
        M106 = -106,
        M105 = -105,
        M104 = -104,
        M103 = -103,
        M102 = -102,
        M101 = -101,
        M100 = -100,
        M99 = -99,
        M98 = -98,
        M97 = -97,
        M96 = -96,
        M95 = -95,
        M94 = -94,
        M93 = -93,
        M92 = -92,
        M91 = -91,
        M90 = -90,
        M89 = -89,
        M88 = -88,
        M87 = -87,
        M86 = -86,
        M85 = -85,
        M84 = -84,
        M83 = -83,
        M82 = -82,
        M81 = -81,
        M80 = -80,
        M79 = -79,
        M78 = -78,
        M77 = -77,
        M76 = -76,
        M75 = -75,
        M74 = -74,
        M73 = -73,
        M72 = -72,
        M71 = -71,
        M70 = -70,
        M69 = -69,
        M68 = -68,
        M67 = -67,
        M66 = -66,
        M65 = -65,
        M64 = -64,
        M63 = -63,
        M62 = -62,
        M61 = -61,
        M60 = -60,
        M59 = -59,
        M58 = -58,
        M57 = -57,
        M56 = -56,
        M55 = -55,
        M54 = -54,
        M53 = -53,
        M52 = -52,
        M51 = -51,
        M50 = -50,
        M49 = -49,
        M48 = -48,
        M47 = -47,
        M46 = -46,
        M45 = -45,
        M44 = -44,
        M43 = -43,
        M42 = -42,
        M41 = -41,
        M40 = -40,
        M39 = -39,
        M38 = -38,
        M37 = -37,
        M36 = -36,
        M35 = -35,
        M34 = -34,
        M33 = -33,
        M32 = -32,
        M31 = -31,
        M30 = -30,
        M29 = -29,
        M28 = -28,
        M27 = -27,
        M26 = -26,
        M25 = -25,
        M24 = -24,
        M23 = -23,
        M22 = -22,
        M21 = -21,
        M20 = -20,
        M19 = -19,
        M18 = -18,
        M17 = -17,
        M16 = -16,
        M15 = -15,
        M14 = -14,
        M13 = -13,
        M12 = -12,
        M11 = -11,
        M10 = -10,
        M9 = -9,
        M8 = -8,
        M7 = -7,
        M6 = -6,
        M5 = -5,
        M4 = -4,
        M3 = -3,
        M2 = -2,
        M1 = -1,
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
        V32 = 32,
        V33 = 33,
        V34 = 34,
        V35 = 35,
        V36 = 36,
        V37 = 37,
        V38 = 38,
        V39 = 39,
        V40 = 40,
        V41 = 41,
        V42 = 42,
        V43 = 43,
        V44 = 44,
        V45 = 45,
        V46 = 46,
        V47 = 47,
        V48 = 48,
        V49 = 49,
        V50 = 50,
        V51 = 51,
        V52 = 52,
        V53 = 53,
        V54 = 54,
        V55 = 55,
        V56 = 56,
        V57 = 57,
        V58 = 58,
        V59 = 59,
        V60 = 60,
        V61 = 61,
        V62 = 62,
        V63 = 63,
        V64 = 64,
        V65 = 65,
        V66 = 66,
        V67 = 67,
        V68 = 68,
        V69 = 69,
        V70 = 70,
        V71 = 71,
        V72 = 72,
        V73 = 73,
        V74 = 74,
        V75 = 75,
        V76 = 76,
        V77 = 77,
        V78 = 78,
        V79 = 79,
        V80 = 80,
        V81 = 81,
        V82 = 82,
        V83 = 83,
        V84 = 84,
        V85 = 85,
        V86 = 86,
        V87 = 87,
        V88 = 88,
        V89 = 89,
        V90 = 90,
        V91 = 91,
        V92 = 92,
        V93 = 93,
        V94 = 94,
        V95 = 95,
        V96 = 96,
        V97 = 97,
        V98 = 98,
        V99 = 99,
        V100 = 100,
        V101 = 101,
        V102 = 102,
        V103 = 103,
        V104 = 104,
        V105 = 105,
        V106 = 106,
        V107 = 107,
        V108 = 108,
        V109 = 109,
        V110 = 110,
        V111 = 111,
        V112 = 112,
        V113 = 113,
        V114 = 114,
        V115 = 115,
        V116 = 116,
        V117 = 117,
        V118 = 118,
        V119 = 119,
        V120 = 120,
        V121 = 121,
        V122 = 122,
        V123 = 123,
        V124 = 124,
        V125 = 125,
        V126 = 126,
        V127 = 127,
    }

    MaxU0(u8) {
        V0 = 0,
    }

    MaxU1(u8) {
        V0 = 0,
        V1 = 1,
    }

    MaxU2(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
    }

    MaxU3(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
    }

    MaxU4(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
    }

    MaxU5(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
    }

    MaxU6(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
        V32 = 32,
        V33 = 33,
        V34 = 34,
        V35 = 35,
        V36 = 36,
        V37 = 37,
        V38 = 38,
        V39 = 39,
        V40 = 40,
        V41 = 41,
        V42 = 42,
        V43 = 43,
        V44 = 44,
        V45 = 45,
        V46 = 46,
        V47 = 47,
        V48 = 48,
        V49 = 49,
        V50 = 50,
        V51 = 51,
        V52 = 52,
        V53 = 53,
        V54 = 54,
        V55 = 55,
        V56 = 56,
        V57 = 57,
        V58 = 58,
        V59 = 59,
        V60 = 60,
        V61 = 61,
        V62 = 62,
        V63 = 63,
    }

    MaxU7(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
        V32 = 32,
        V33 = 33,
        V34 = 34,
        V35 = 35,
        V36 = 36,
        V37 = 37,
        V38 = 38,
        V39 = 39,
        V40 = 40,
        V41 = 41,
        V42 = 42,
        V43 = 43,
        V44 = 44,
        V45 = 45,
        V46 = 46,
        V47 = 47,
        V48 = 48,
        V49 = 49,
        V50 = 50,
        V51 = 51,
        V52 = 52,
        V53 = 53,
        V54 = 54,
        V55 = 55,
        V56 = 56,
        V57 = 57,
        V58 = 58,
        V59 = 59,
        V60 = 60,
        V61 = 61,
        V62 = 62,
        V63 = 63,
        V64 = 64,
        V65 = 65,
        V66 = 66,
        V67 = 67,
        V68 = 68,
        V69 = 69,
        V70 = 70,
        V71 = 71,
        V72 = 72,
        V73 = 73,
        V74 = 74,
        V75 = 75,
        V76 = 76,
        V77 = 77,
        V78 = 78,
        V79 = 79,
        V80 = 80,
        V81 = 81,
        V82 = 82,
        V83 = 83,
        V84 = 84,
        V85 = 85,
        V86 = 86,
        V87 = 87,
        V88 = 88,
        V89 = 89,
        V90 = 90,
        V91 = 91,
        V92 = 92,
        V93 = 93,
        V94 = 94,
        V95 = 95,
        V96 = 96,
        V97 = 97,
        V98 = 98,
        V99 = 99,
        V100 = 100,
        V101 = 101,
        V102 = 102,
        V103 = 103,
        V104 = 104,
        V105 = 105,
        V106 = 106,
        V107 = 107,
        V108 = 108,
        V109 = 109,
        V110 = 110,
        V111 = 111,
        V112 = 112,
        V113 = 113,
        V114 = 114,
        V115 = 115,
        V116 = 116,
        V117 = 117,
        V118 = 118,
        V119 = 119,
        V120 = 120,
        V121 = 121,
        V122 = 122,
        V123 = 123,
        V124 = 124,
        V125 = 125,
        V126 = 126,
        V127 = 127,
    }

    MaxU8(u8) {
        V0 = 0,
        V1 = 1,
        V2 = 2,
        V3 = 3,
        V4 = 4,
        V5 = 5,
        V6 = 6,
        V7 = 7,
        V8 = 8,
        V9 = 9,
        V10 = 10,
        V11 = 11,
        V12 = 12,
        V13 = 13,
        V14 = 14,
        V15 = 15,
        V16 = 16,
        V17 = 17,
        V18 = 18,
        V19 = 19,
        V20 = 20,
        V21 = 21,
        V22 = 22,
        V23 = 23,
        V24 = 24,
        V25 = 25,
        V26 = 26,
        V27 = 27,
        V28 = 28,
        V29 = 29,
        V30 = 30,
        V31 = 31,
        V32 = 32,
        V33 = 33,
        V34 = 34,
        V35 = 35,
        V36 = 36,
        V37 = 37,
        V38 = 38,
        V39 = 39,
        V40 = 40,
        V41 = 41,
        V42 = 42,
        V43 = 43,
        V44 = 44,
        V45 = 45,
        V46 = 46,
        V47 = 47,
        V48 = 48,
        V49 = 49,
        V50 = 50,
        V51 = 51,
        V52 = 52,
        V53 = 53,
        V54 = 54,
        V55 = 55,
        V56 = 56,
        V57 = 57,
        V58 = 58,
        V59 = 59,
        V60 = 60,
        V61 = 61,
        V62 = 62,
        V63 = 63,
        V64 = 64,
        V65 = 65,
        V66 = 66,
        V67 = 67,
        V68 = 68,
        V69 = 69,
        V70 = 70,
        V71 = 71,
        V72 = 72,
        V73 = 73,
        V74 = 74,
        V75 = 75,
        V76 = 76,
        V77 = 77,
        V78 = 78,
        V79 = 79,
        V80 = 80,
        V81 = 81,
        V82 = 82,
        V83 = 83,
        V84 = 84,
        V85 = 85,
        V86 = 86,
        V87 = 87,
        V88 = 88,
        V89 = 89,
        V90 = 90,
        V91 = 91,
        V92 = 92,
        V93 = 93,
        V94 = 94,
        V95 = 95,
        V96 = 96,
        V97 = 97,
        V98 = 98,
        V99 = 99,
        V100 = 100,
        V101 = 101,
        V102 = 102,
        V103 = 103,
        V104 = 104,
        V105 = 105,
        V106 = 106,
        V107 = 107,
        V108 = 108,
        V109 = 109,
        V110 = 110,
        V111 = 111,
        V112 = 112,
        V113 = 113,
        V114 = 114,
        V115 = 115,
        V116 = 116,
        V117 = 117,
        V118 = 118,
        V119 = 119,
        V120 = 120,
        V121 = 121,
        V122 = 122,
        V123 = 123,
        V124 = 124,
        V125 = 125,
        V126 = 126,
        V127 = 127,
        V128 = 128,
        V129 = 129,
        V130 = 130,
        V131 = 131,
        V132 = 132,
        V133 = 133,
        V134 = 134,
        V135 = 135,
        V136 = 136,
        V137 = 137,
        V138 = 138,
        V139 = 139,
        V140 = 140,
        V141 = 141,
        V142 = 142,
        V143 = 143,
        V144 = 144,
        V145 = 145,
        V146 = 146,
        V147 = 147,
        V148 = 148,
        V149 = 149,
        V150 = 150,
        V151 = 151,
        V152 = 152,
        V153 = 153,
        V154 = 154,
        V155 = 155,
        V156 = 156,
        V157 = 157,
        V158 = 158,
        V159 = 159,
        V160 = 160,
        V161 = 161,
        V162 = 162,
        V163 = 163,
        V164 = 164,
        V165 = 165,
        V166 = 166,
        V167 = 167,
        V168 = 168,
        V169 = 169,
        V170 = 170,
        V171 = 171,
        V172 = 172,
        V173 = 173,
        V174 = 174,
        V175 = 175,
        V176 = 176,
        V177 = 177,
        V178 = 178,
        V179 = 179,
        V180 = 180,
        V181 = 181,
        V182 = 182,
        V183 = 183,
        V184 = 184,
        V185 = 185,
        V186 = 186,
        V187 = 187,
        V188 = 188,
        V189 = 189,
        V190 = 190,
        V191 = 191,
        V192 = 192,
        V193 = 193,
        V194 = 194,
        V195 = 195,
        V196 = 196,
        V197 = 197,
        V198 = 198,
        V199 = 199,
        V200 = 200,
        V201 = 201,
        V202 = 202,
        V203 = 203,
        V204 = 204,
        V205 = 205,
        V206 = 206,
        V207 = 207,
        V208 = 208,
        V209 = 209,
        V210 = 210,
        V211 = 211,
        V212 = 212,
        V213 = 213,
        V214 = 214,
        V215 = 215,
        V216 = 216,
        V217 = 217,
        V218 = 218,
        V219 = 219,
        V220 = 220,
        V221 = 221,
        V222 = 222,
        V223 = 223,
        V224 = 224,
        V225 = 225,
        V226 = 226,
        V227 = 227,
        V228 = 228,
        V229 = 229,
        V230 = 230,
        V231 = 231,
        V232 = 232,
        V233 = 233,
        V234 = 234,
        V235 = 235,
        V236 = 236,
        V237 = 237,
        V238 = 238,
        V239 = 239,
        V240 = 240,
        V241 = 241,
        V242 = 242,
        V243 = 243,
        V244 = 244,
        V245 = 245,
        V246 = 246,
        V247 = 247,
        V248 = 248,
        V249 = 249,
        V250 = 250,
        V251 = 251,
        V252 = 252,
        V253 = 253,
        V254 = 254,
        V255 = 255,
    }
}

new_int! {
    (u1 u8)
    (0 ..= 0x01)
    (a: MaxU1)
    (a: MaxU1)

    (u2 u8)
    (0 ..= 0x03)
    (a: MaxU2)
    (a: MaxU2)

    (u3 u8)
    (0 ..= 0x07)
    (a: MaxU3)
    (a: MaxU3)

    (u4 u8)
    (0 ..= 0x0f)
    (a: MaxU4)
    (a: MaxU4)

    (u5 u8)
    (0 ..= 0x1f)
    (a: MaxU5)
    (a: MaxU5)

    (u6 u8)
    (0 ..= 0x3f)
    (a: MaxU6)
    (a: MaxU6)

    (u7 u8)
    (0 ..= 0x7f)
    (a: MaxU7)
    (a: MaxU7)

    // (u8 u8)
    // (0 ..= 0xff)
    // (a: MaxU8)
    // (a: MaxU8)

    (u9 u16)
    (0 ..= 0x01ff)
    (a: MaxU8, b: MaxU1)
    (b: MaxU1, a: MaxU8)

    (u10 u16)
    (0 ..= 0x03ff)
    (a: MaxU8, b: MaxU2)
    (b: MaxU2, a: MaxU8)

    (u11 u16)
    (0 ..= 0x07ff)
    (a: MaxU8, b: MaxU3)
    (b: MaxU3, a: MaxU8)

    (u12 u16)
    (0 ..= 0x0fff)
    (a: MaxU8, b: MaxU4)
    (b: MaxU4, a: MaxU8)

    (u13 u16)
    (0 ..= 0x1fff)
    (a: MaxU8, b: MaxU5)
    (b: MaxU5, a: MaxU8)

    (u14 u16)
    (0 ..= 0x3fff)
    (a: MaxU8, b: MaxU6)
    (b: MaxU6, a: MaxU8)

    (u15 u16)
    (0 ..= 0x7fff)
    (a: MaxU8, b: MaxU7)
    (b: MaxU7, a: MaxU8)

    // (u16 u16)
    // (0 ..= 0xffff)
    // (a: MaxU8, b: MaxU8)
    // (b: MaxU8, a: MaxU8)

    (u17 u32)
    (0 ..= 0x0001_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU1, d: MaxU0)
    (d: MaxU0, c: MaxU1, b: MaxU8, a: MaxU8)

    (u18 u32)
    (0 ..= 0x0003_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU2, d: MaxU0)
    (d: MaxU0, c: MaxU2, b: MaxU8, a: MaxU8)

    (u19 u32)
    (0 ..= 0x0007_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU3, d: MaxU0)
    (d: MaxU0, c: MaxU3, b: MaxU8, a: MaxU8)

    (u20 u32)
    (0 ..= 0x000f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU4, d: MaxU0)
    (d: MaxU0, c: MaxU4, b: MaxU8, a: MaxU8)

    (u21 u32)
    (0 ..= 0x001f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU5, d: MaxU0)
    (d: MaxU0, c: MaxU5, b: MaxU8, a: MaxU8)

    (u22 u32)
    (0 ..= 0x003f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU6, d: MaxU0)
    (d: MaxU0, c: MaxU6, b: MaxU8, a: MaxU8)

    (u23 u32)
    (0 ..= 0x007f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU7, d: MaxU0)
    (d: MaxU1, c: MaxU7, b: MaxU8, a: MaxU8)

    (u24 u32)
    (0 ..= 0x00ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU0)
    (d: MaxU0, c: MaxU8, b: MaxU8, a: MaxU8)

    (u25 u32)
    (0 ..= 0x01ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU1)
    (d: MaxU1, c: MaxU8, b: MaxU8, a: MaxU8)

    (u26 u32)
    (0 ..= 0x03ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU2)
    (d: MaxU2, c: MaxU8, b: MaxU8, a: MaxU8)

    (u27 u32)
    (0 ..= 0x07ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU3)
    (d: MaxU3, c: MaxU8, b: MaxU8, a: MaxU8)

    (u28 u32)
    (0 ..= 0x0fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU4)
    (d: MaxU4, c: MaxU8, b: MaxU8, a: MaxU8)

    (u29 u32)
    (0 ..= 0x1fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU5)
    (d: MaxU5, c: MaxU8, b: MaxU8, a: MaxU8)

    (u30 u32)
    (0 ..= 0x3fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU6)
    (d: MaxU6, c: MaxU8, b: MaxU8, a: MaxU8)

    (u31 u32)
    (0 ..= 0x7fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU7)
    (d: MaxU7, c: MaxU8, b: MaxU8, a: MaxU8)

    // (u32 u32)
    // (0 ..= 0xffff_ffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8)
    // (d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u33 u64)
    (0 ..= 0x00000001ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU1, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU1, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u34 u64)
    (0 ..= 0x00000003ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU2, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU2, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u35 u64)
    (0 ..= 0x00000007ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU3, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU3, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u36 u64)
    (0 ..= 0x0000000fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU4, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU4, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u37 u64)
    (0 ..= 0x0000001fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU5, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU5, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u38 u64)
    (0 ..= 0x0000003fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU6, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU6, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u39 u64)
    (0 ..= 0x0000007fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU7, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU7, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u40 u64)
    (0 ..= 0x000000ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u41 u64)
    (0 ..= 0x000001ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU1, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU1, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u42 u64)
    (0 ..= 0x000003ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU2, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU2, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u43 u64)
    (0 ..= 0x000007ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU3, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU3, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u44 u64)
    (0 ..= 0x00000fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU4, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU4, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u45 u64)
    (0 ..= 0x00001fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU5, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU5, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u46 u64)
    (0 ..= 0x00003fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU6, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU6, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u47 u64)
    (0 ..= 0x00007fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU7, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU7, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u48 u64)
    (0 ..= 0x0000ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u49 u64)
    (0 ..= 0x0001ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU1, h: MaxU0)
    (h: MaxU0, g: MaxU1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u50 u64)
    (0 ..= 0x0003ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU2, h: MaxU0)
    (h: MaxU0, g: MaxU2, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u51 u64)
    (0 ..= 0x0007ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU3, h: MaxU0)
    (h: MaxU0, g: MaxU3, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u52 u64)
    (0 ..= 0x000fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU4, h: MaxU0)
    (h: MaxU0, g: MaxU4, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u53 u64)
    (0 ..= 0x001fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU5, h: MaxU0)
    (h: MaxU0, g: MaxU5, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u54 u64)
    (0 ..= 0x003fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU6, h: MaxU0)
    (h: MaxU0, g: MaxU6, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u55 u64)
    (0 ..= 0x007fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU7, h: MaxU0)
    (h: MaxU0, g: MaxU7, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u56 u64)
    (0 ..= 0x00ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU0)
    (h: MaxU0, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u57 u64)
    (0 ..= 0x01ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU1)
    (h: MaxU1, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u58 u64)
    (0 ..= 0x03ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU2)
    (h: MaxU2, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u59 u64)
    (0 ..= 0x07ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU3)
    (h: MaxU3, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u60 u64)
    (0 ..= 0x0fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU4)
    (h: MaxU4, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u61 u64)
    (0 ..= 0x1fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU5)
    (h: MaxU5, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u62 u64)
    (0 ..= 0x3fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU6)
    (h: MaxU6, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u63 u64)
    (0 ..= 0x7fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU7)
    (h: MaxU7, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (u64 u64)
    // (0 ..= 0xffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8)
    // (h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u65 u128)
    (0 ..= 0x0000000000000001ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU1, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU1, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u66 u128)
    (0 ..= 0x0000000000000003ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU2, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU2, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u67 u128)
    (0 ..= 0x0000000000000007ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU3, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU3, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u68 u128)
    (0 ..= 0x000000000000000fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU4, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU4, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u69 u128)
    (0 ..= 0x000000000000001fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU5, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU5, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u70 u128)
    (0 ..= 0x000000000000003fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU6, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU6, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u71 u128)
    (0 ..= 0x000000000000007fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU7, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU7, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u72 u128)
    (0 ..= 0x00000000000000ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u73 u128)
    (0 ..= 0x00000000000001ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU1, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU1, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u74 u128)
    (0 ..= 0x00000000000003ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU2, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU2, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u75 u128)
    (0 ..= 0x00000000000007ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU3, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU3, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u76 u128)
    (0 ..= 0x0000000000000fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU4, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU4, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u77 u128)
    (0 ..= 0x0000000000001fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU5, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU5, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u78 u128)
    (0 ..= 0x0000000000003fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU6, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU6, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u79 u128)
    (0 ..= 0x0000000000007fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU7, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU7, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u80 u128)
    (0 ..= 0x000000000000ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u81 u128)
    (0 ..= 0x000000000001ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU1, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU1, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u82 u128)
    (0 ..= 0x000000000003ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU2, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU2, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u83 u128)
    (0 ..= 0x000000000007ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU3, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU3, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u84 u128)
    (0 ..= 0x00000000000fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU4, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU4, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u85 u128)
    (0 ..= 0x00000000001fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU5, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU5, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u86 u128)
    (0 ..= 0x00000000003fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU6, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU6, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u87 u128)
    (0 ..= 0x00000000007fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU7, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU7, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u88 u128)
    (0 ..= 0x0000000000ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u89 u128)
    (0 ..= 0x0000000001ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU1, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU1, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u90 u128)
    (0 ..= 0x0000000003ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU2, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU2, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u91 u128)
    (0 ..= 0x0000000007ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU3, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU3, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u92 u128)
    (0 ..= 0x000000000fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU4, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU4, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u93 u128)
    (0 ..= 0x000000001fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU5, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU5, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u94 u128)
    (0 ..= 0x000000003fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU6, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU6, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u95 u128)
    (0 ..= 0x000000007fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU7, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU7, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u96 u128)
    (0 ..= 0x00000000ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u97 u128)
    (0 ..= 0x00000001ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU1, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU1, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u98 u128)
    (0 ..= 0x00000003ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU2, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU2, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u99 u128)
    (0 ..= 0x00000007ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU3, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU3, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u100 u128)
    (0 ..= 0x0000000fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU4, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU4, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u101 u128)
    (0 ..= 0x0000001fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU5, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU5, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u102 u128)
    (0 ..= 0x0000003fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU6, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU6, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u103 u128)
    (0 ..= 0x0000007fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU7, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU7, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u104 u128)
    (0 ..= 0x000000ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u105 u128)
    (0 ..= 0x000001ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU1, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU1, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u106 u128)
    (0 ..= 0x000003ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU2, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU2, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u107 u128)
    (0 ..= 0x000007ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU3, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU3, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u108 u128)
    (0 ..= 0x00000fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU4, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU4, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u109 u128)
    (0 ..= 0x00001fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU5, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU5, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u110 u128)
    (0 ..= 0x00003fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU6, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU6, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u111 u128)
    (0 ..= 0x00007fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU7, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU7, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u112 u128)
    (0 ..= 0x0000ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u113 u128)
    (0 ..= 0x0001ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU1, p: MaxU1)
    (p: MaxU1, o: MaxU1, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u114 u128)
    (0 ..= 0x0003ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU2, p: MaxU1)
    (p: MaxU1, o: MaxU2, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u115 u128)
    (0 ..= 0x0007ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU3, p: MaxU1)
    (p: MaxU1, o: MaxU3, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u116 u128)
    (0 ..= 0x000fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU4, p: MaxU1)
    (p: MaxU1, o: MaxU4, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u117 u128)
    (0 ..= 0x001fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU5, p: MaxU1)
    (p: MaxU1, o: MaxU5, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u118 u128)
    (0 ..= 0x003fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU6, p: MaxU1)
    (p: MaxU1, o: MaxU6, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u119 u128)
    (0 ..= 0x007fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU7, p: MaxU1)
    (p: MaxU1, o: MaxU7, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u120 u128)
    (0 ..= 0x00ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU1)
    (p: MaxU1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u121 u128)
    (0 ..= 0x01ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU1)
    (p: MaxU1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u122 u128)
    (0 ..= 0x03ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU2)
    (p: MaxU2, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u123 u128)
    (0 ..= 0x07ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU3)
    (p: MaxU3, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u124 u128)
    (0 ..= 0x0fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU4)
    (p: MaxU4, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u125 u128)
    (0 ..= 0x1fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU5)
    (p: MaxU5, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u126 u128)
    (0 ..= 0x3fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU6)
    (p: MaxU6, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u127 u128)
    (0 ..= 0x7fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU7)
    (p: MaxU7, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (u128 u128)
    // (0 ..= 0xffffffffffffffffffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU8)
    // (p: MaxU8, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i1 i8)
    (-0x01 ..= 0x00)
    (a: MaxI1)
    (a: MaxI1)

    (i2 i8)
    (-0x02 ..= 0x01)
    (a: MaxI2)
    (a: MaxI2)

    (i3 i8)
    (-0x04 ..= 0x03)
    (a: MaxI3)
    (a: MaxI3)

    (i4 i8)
    (-0x08 ..= 0x07)
    (a: MaxI4)
    (a: MaxI4)

    (i5 i8)
    (-0x10 ..= 0x0f)
    (a: MaxI5)
    (a: MaxI5)

    (i6 i8)
    (-0x20 ..= 0x1f)
    (a: MaxI6)
    (a: MaxI6)

    (i7 i8)
    (-0x40 ..= 0x3f)
    (a: MaxI7)
    (a: MaxI7)

    // (i8 i8)
    // (-0x80 ..= 0x7f)
    // (a: MaxI8)
    // (a: MaxI8)

    (i9 i16)
    (-0x0100 ..= 0x00ff)
    (a: MaxU8, b: MaxI1)
    (b: MaxI1, a: MaxU8)

    (i10 i16)
    (-0x0200 ..= 0x01ff)
    (a: MaxU8, b: MaxI2)
    (b: MaxI2, a: MaxU8)

    (i11 i16)
    (-0x0400 ..= 0x03ff)
    (a: MaxU8, b: MaxI3)
    (b: MaxI3, a: MaxU8)

    (i12 i16)
    (-0x0800 ..= 0x07ff)
    (a: MaxU8, b: MaxI4)
    (b: MaxI4, a: MaxU8)

    (i13 i16)
    (-0x1000 ..= 0x0fff)
    (a: MaxU8, b: MaxI5)
    (b: MaxI5, a: MaxU8)

    (i14 i16)
    (-0x2000 ..= 0x1fff)
    (a: MaxU8, b: MaxI6)
    (b: MaxI6, a: MaxU8)

    (i15 i16)
    (-0x4000 ..= 0x3fff)
    (a: MaxU8, b: MaxI7)
    (b: MaxI7, a: MaxU8)

    // (i16 i16)
    // (-0x8000 ..= 0x7fff)
    // (a: MaxU8, b: MaxI8)
    // (b: MaxI8, a: MaxU8)

    (i17 i32)
    (-0x0001_0000 ..= 0x0000_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI1, d: MaxI1)
    (d: MaxI1, c: MaxI1, b: MaxU8, a: MaxU8)

    (i18 i32)
    (-0x0002_0000 ..= 0x0001_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI2, d: MaxI1)
    (d: MaxI1, c: MaxI2, b: MaxU8, a: MaxU8)

    (i19 i32)
    (-0x0004_0000 ..= 0x0003_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI3, d: MaxI1)
    (d: MaxI1, c: MaxI3, b: MaxU8, a: MaxU8)

    (i20 i32)
    (-0x0008_0000 ..= 0x0007_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI4, d: MaxI1)
    (d: MaxI1, c: MaxI4, b: MaxU8, a: MaxU8)

    (i21 i32)
    (-0x0010_0000 ..= 0x000f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI5, d: MaxI1)
    (d: MaxI1, c: MaxI5, b: MaxU8, a: MaxU8)

    (i22 i32)
    (-0x0020_0000 ..= 0x001f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI6, d: MaxI1)
    (d: MaxI1, c: MaxI6, b: MaxU8, a: MaxU8)

    (i23 i32)
    (-0x0040_0000 ..= 0x003f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI7, d: MaxI1)
    (d: MaxI1, c: MaxI7, b: MaxU8, a: MaxU8)

    (i24 i32)
    (-0x0080_0000 ..= 0x007f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI8, d: MaxI1)
    (d: MaxI1, c: MaxU8, b: MaxU8, a: MaxU8)

    (i25 i32)
    (-0x0100_0000 ..= 0x00ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI1)
    (d: MaxI1, c: MaxU8, b: MaxU8, a: MaxU8)

    (i26 i32)
    (-0x0200_0000 ..= 0x01ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI2)
    (d: MaxI2, c: MaxU8, b: MaxU8, a: MaxU8)

    (i27 i32)
    (-0x0400_0000 ..= 0x03ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI3)
    (d: MaxI3, c: MaxU8, b: MaxU8, a: MaxU8)

    (i28 i32)
    (-0x0800_0000 ..= 0x07ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI4)
    (d: MaxI4, c: MaxU8, b: MaxU8, a: MaxU8)

    (i29 i32)
    (-0x1000_0000 ..= 0x0fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI5)
    (d: MaxI5, c: MaxU8, b: MaxU8, a: MaxU8)

    (i30 i32)
    (-0x2000_0000 ..= 0x1fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI6)
    (d: MaxI6, c: MaxU8, b: MaxU8, a: MaxU8)

    (i31 i32)
    (-0x4000_0000 ..= 0x3fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI7)
    (d: MaxI7, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i32 i32)
    // (-0x8000_0000 ..= 0x7fff_ffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI8)
    // (d: MaxI8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i33 i64)
    (-0x0000000100000000 ..= 0x00000000ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI1, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI1, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i34 i64)
    (-0x0000000200000000 ..= 0x00000001ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI2, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI2, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i35 i64)
    (-0x0000000400000000 ..= 0x00000003ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI3, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI3, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i36 i64)
    (-0x0000000800000000 ..= 0x00000007ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI4, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI4, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i37 i64)
    (-0x0000001000000000 ..= 0x0000000fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI5, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI5, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i38 i64)
    (-0x0000002000000000 ..= 0x0000001fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI6, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI6, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i39 i64)
    (-0x0000004000000000 ..= 0x0000003fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI7, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI7, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i40 i64)
    (-0x0000008000000000 ..= 0x0000007fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI8, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i41 i64)
    (-0x0000010000000000 ..= 0x000000ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i42 i64)
    (-0x0000020000000000 ..= 0x000001ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI2, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI2, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i43 i64)
    (-0x0000040000000000 ..= 0x000003ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI3, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI3, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i44 i64)
    (-0x0000080000000000 ..= 0x000007ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI4, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI4, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i45 i64)
    (-0x0000100000000000 ..= 0x00000fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI5, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI5, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i46 i64)
    (-0x0000200000000000 ..= 0x00001fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI6, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI6, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i47 i64)
    (-0x0000400000000000 ..= 0x00003fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI7, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI7, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i48 i64)
    (-0x0000800000000000 ..= 0x00007fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI8, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i49 i64)
    (-0x0001000000000000 ..= 0x0000ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i50 i64)
    (-0x0002000000000000 ..= 0x0001ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI2, h: MaxI1)
    (h: MaxI1, g: MaxI2, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i51 i64)
    (-0x0004000000000000 ..= 0x0003ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI3, h: MaxI1)
    (h: MaxI1, g: MaxI3, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i52 i64)
    (-0x0008000000000000 ..= 0x0007ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI4, h: MaxI1)
    (h: MaxI1, g: MaxI4, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i53 i64)
    (-0x0010000000000000 ..= 0x000fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI5, h: MaxI1)
    (h: MaxI1, g: MaxI5, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i54 i64)
    (-0x0020000000000000 ..= 0x001fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI6, h: MaxI1)
    (h: MaxI1, g: MaxI6, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i55 i64)
    (-0x0040000000000000 ..= 0x003fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI7, h: MaxI1)
    (h: MaxI1, g: MaxI7, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i56 i64)
    (-0x0080000000000000 ..= 0x007fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI8, h: MaxI1)
    (h: MaxI1, g: MaxI8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i57 i64)
    (-0x0100000000000000 ..= 0x00ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI1)
    (h: MaxI1, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i58 i64)
    (-0x0200000000000000 ..= 0x01ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI2)
    (h: MaxI2, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i59 i64)
    (-0x0400000000000000 ..= 0x03ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI3)
    (h: MaxI3, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i60 i64)
    (-0x0800000000000000 ..= 0x07ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI4)
    (h: MaxI4, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i61 i64)
    (-0x1000000000000000 ..= 0x0fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI5)
    (h: MaxI5, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i62 i64)
    (-0x2000000000000000 ..= 0x1fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI6)
    (h: MaxI6, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i63 i64)
    (-0x4000000000000000 ..= 0x3fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI7)
    (h: MaxI7, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i64 i64)
    // (-0x8000000000000000 ..= 0x7fffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI8)
    // (h: MaxI8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i65 i128)
    (-0x00000000000000010000000000000000 ..= 0x0000000000000000ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI1, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI1, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i66 i128)
    (-0x00000000000000020000000000000000 ..= 0x0000000000000001ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI2, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI2, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i67 i128)
    (-0x00000000000000040000000000000000 ..= 0x0000000000000003ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI3, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI3, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i68 i128)
    (-0x00000000000000080000000000000000 ..= 0x0000000000000007ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI4, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI4, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i69 i128)
    (-0x00000000000000100000000000000000 ..= 0x000000000000000fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI5, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI5, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i70 i128)
    (-0x00000000000000200000000000000000 ..= 0x000000000000001fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI6, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI6, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i71 i128)
    (-0x00000000000000400000000000000000 ..= 0x000000000000003fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI7, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI7, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i72 i128)
    (-0x00000000000000800000000000000000 ..= 0x000000000000007fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI8, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i73 i128)
    (-0x00000000000001000000000000000000 ..= 0x00000000000000ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i74 i128)
    (-0x00000000000002000000000000000000 ..= 0x00000000000001ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI2, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI2, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i75 i128)
    (-0x00000000000004000000000000000000 ..= 0x00000000000003ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI3, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI3, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i76 i128)
    (-0x00000000000008000000000000000000 ..= 0x00000000000007ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI4, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI4, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i77 i128)
    (-0x00000000000010000000000000000000 ..= 0x0000000000000fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI5, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI5, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i78 i128)
    (-0x00000000000020000000000000000000 ..= 0x0000000000001fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI6, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI6, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i79 i128)
    (-0x00000000000040000000000000000000 ..= 0x0000000000003fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI7, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI7, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i80 i128)
    (-0x00000000000080000000000000000000 ..= 0x0000000000007fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI8, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i81 i128)
    (-0x00000000000100000000000000000000 ..= 0x000000000000ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i82 i128)
    (-0x00000000000200000000000000000000 ..= 0x000000000001ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI2, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI2, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i83 i128)
    (-0x00000000000400000000000000000000 ..= 0x000000000003ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI3, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI3, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i84 i128)
    (-0x00000000000800000000000000000000 ..= 0x000000000007ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI4, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI4, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i85 i128)
    (-0x00000000001000000000000000000000 ..= 0x00000000000fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI5, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI5, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i86 i128)
    (-0x00000000002000000000000000000000 ..= 0x00000000001fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI6, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI6, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i87 i128)
    (-0x00000000004000000000000000000000 ..= 0x00000000003fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI7, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI7, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i88 i128)
    (-0x00000000008000000000000000000000 ..= 0x00000000007fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI8, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i89 i128)
    (-0x00000000010000000000000000000000 ..= 0x0000000000ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i90 i128)
    (-0x00000000020000000000000000000000 ..= 0x0000000001ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI2, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI2, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i91 i128)
    (-0x00000000040000000000000000000000 ..= 0x0000000003ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI3, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI3, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i92 i128)
    (-0x00000000080000000000000000000000 ..= 0x0000000007ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI4, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI4, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i93 i128)
    (-0x00000000100000000000000000000000 ..= 0x000000000fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI5, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI5, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i94 i128)
    (-0x00000000200000000000000000000000 ..= 0x000000001fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI6, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI6, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i95 i128)
    (-0x00000000400000000000000000000000 ..= 0x000000003fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI7, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI7, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i96 i128)
    (-0x00000000800000000000000000000000 ..= 0x000000007fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI8, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i97 i128)
    (-0x00000001000000000000000000000000 ..= 0x00000000ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i98 i128)
    (-0x00000002000000000000000000000000 ..= 0x00000001ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI2, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI2, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i99 i128)
    (-0x00000004000000000000000000000000 ..= 0x00000003ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI3, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI3, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i100 i128)
    (-0x00000008000000000000000000000000 ..= 0x00000007ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI4, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI4, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i101 i128)
    (-0x00000010000000000000000000000000 ..= 0x0000000fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI5, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI5, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i102 i128)
    (-0x00000020000000000000000000000000 ..= 0x0000001fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI6, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI6, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i103 i128)
    (-0x00000040000000000000000000000000 ..= 0x0000003fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI7, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI7, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i104 i128)
    (-0x00000080000000000000000000000000 ..= 0x0000007fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI8, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i105 i128)
    (-0x00000100000000000000000000000000 ..= 0x000000ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i106 i128)
    (-0x00000200000000000000000000000000 ..= 0x000001ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI2, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI2, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i107 i128)
    (-0x00000400000000000000000000000000 ..= 0x000003ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI3, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI3, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i108 i128)
    (-0x00000800000000000000000000000000 ..= 0x000007ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI4, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI4, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i109 i128)
    (-0x00001000000000000000000000000000 ..= 0x00000fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI5, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI5, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i110 i128)
    (-0x00002000000000000000000000000000 ..= 0x00001fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI6, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI6, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i111 i128)
    (-0x00004000000000000000000000000000 ..= 0x00003fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI7, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI7, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i112 i128)
    (-0x00008000000000000000000000000000 ..= 0x00007fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI8, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i113 i128)
    (-0x00010000000000000000000000000000 ..= 0x0000ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i114 i128)
    (-0x00020000000000000000000000000000 ..= 0x0001ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI2, p: MaxI1)
    (p: MaxI1, o: MaxI2, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i115 i128)
    (-0x00040000000000000000000000000000 ..= 0x0003ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI3, p: MaxI1)
    (p: MaxI1, o: MaxI3, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i116 i128)
    (-0x00080000000000000000000000000000 ..= 0x0007ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI4, p: MaxI1)
    (p: MaxI1, o: MaxI4, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i117 i128)
    (-0x00100000000000000000000000000000 ..= 0x000fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI5, p: MaxI1)
    (p: MaxI1, o: MaxI5, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i118 i128)
    (-0x00200000000000000000000000000000 ..= 0x001fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI6, p: MaxI1)
    (p: MaxI1, o: MaxI6, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i119 i128)
    (-0x00400000000000000000000000000000 ..= 0x003fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI7, p: MaxI1)
    (p: MaxI1, o: MaxI7, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i120 i128)
    (-0x00800000000000000000000000000000 ..= 0x007fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI8, p: MaxI1)
    (p: MaxI1, o: MaxI8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i121 i128)
    (-0x01000000000000000000000000000000 ..= 0x00ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI1)
    (p: MaxI1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i122 i128)
    (-0x02000000000000000000000000000000 ..= 0x01ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI2)
    (p: MaxI2, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i123 i128)
    (-0x04000000000000000000000000000000 ..= 0x03ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI3)
    (p: MaxI3, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i124 i128)
    (-0x08000000000000000000000000000000 ..= 0x07ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI4)
    (p: MaxI4, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i125 i128)
    (-0x10000000000000000000000000000000 ..= 0x0fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI5)
    (p: MaxI5, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i126 i128)
    (-0x20000000000000000000000000000000 ..= 0x1fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI6)
    (p: MaxI6, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i127 i128)
    (-0x40000000000000000000000000000000 ..= 0x3fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI7)
    (p: MaxI7, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i128 i128)
    // (-0x80000000000000000000000000000000 ..= 0x7fffffffffffffffffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI8)
    // (p: MaxI8, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)
}
