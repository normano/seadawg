// The pointer compare macro with offset support
macro_rules! cmp (
    ($left:expr, $right: expr, $var:ident, $offset:expr) => {
        unsafe {*($left.offset($offset) as *const $var) == *($right.offset($offset) as *const $var)}
    }
);

macro_rules! slice_compare (
    ($a:expr, $b:expr, $len:expr) => {{
        match $len {
            1 => cmp!($a, $b, u8, 0),
            2 => cmp!($a, $b, u16, 0),
            3 => cmp!($a, $b, u16, 0) && cmp!($a, $b, u8, 2),
            4 => cmp!($a, $b, u32, 0),
            5 => cmp!($a, $b, u32, 0) && cmp!($a, $b, u8, 4),
            6 => cmp!($a, $b, u32, 0) && cmp!($a, $b, u16, 4),
            7 => cmp!($a, $b, u32, 0) && cmp!($a, $b, u16, 4) && cmp!($a, $b, u8, 6),
            8 => cmp!($a, $b, u64, 0),
            9 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u8, 8),
            10 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u16, 8),
            11 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u16, 8) && cmp!($a, $b, u8, 10),
            12 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u32, 8),
            13 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u32, 8) && cmp!($a, $b, u8, 12),
            14 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u32, 8) && cmp!($a, $b, u16, 12),
            15 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u32, 8) && cmp!($a, $b, u16, 12) && cmp!($a, $b, u8, 14),
            16 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8),
            17 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u8, 16),
            18 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u16, 16),
            19 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u16, 16) && cmp!($a, $b, u8, 18),
            20 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u32, 16),
            21 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u32, 16) && cmp!($a, $b, u8, 20),
            22 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u32, 16) && cmp!($a, $b, u16, 20),
            23 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u32, 16) && cmp!($a, $b, u16, 20) && cmp!($a, $b, u8, 22),
            24 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16),
            25 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u8, 24),
            26 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u16, 24),
            27 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u16, 24) && cmp!($a, $b, u8, 26),
            28 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u32, 24),
            29 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u32, 24) && cmp!($a, $b, u8, 28),
            30 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u32, 24) && cmp!($a, $b, u16, 28),
            31 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u32, 24) && cmp!($a, $b, u16, 28) && cmp!($a, $b, u8, 30),
            32 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24),
            33 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u8, 32),
            34 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u16, 32),
            35 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u16, 32) && cmp!($a, $b, u8, 34),
            36 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u32, 32),
            37 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u32, 32) && cmp!($a, $b, u8, 36),
            38 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u32, 32) && cmp!($a, $b, u16, 36),
            39 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u32, 32) && cmp!($a, $b, u16, 36) && cmp!($a, $b, u8, 38),
            40 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32),
            41 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u8, 40),
            42 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u16, 40),
            43 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u16, 40) && cmp!($a, $b, u8, 42),
            44 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u32, 40),
            45 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u32, 40) && cmp!($a, $b, u8, 44),
            46 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u32, 40) && cmp!($a, $b, u16, 44),
            47 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u32, 40) && cmp!($a, $b, u16, 44) && cmp!($a, $b, u8, 46),
            48 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40),
            49 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u8, 48),
            50 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u16, 48),
            51 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u16, 48) && cmp!($a, $b, u8, 50),
            52 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u32, 48),
            53 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u32, 48) && cmp!($a, $b, u8, 52),
            54 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u32, 48) && cmp!($a, $b, u16, 52),
            55 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u32, 48) && cmp!($a, $b, u16, 52) && cmp!($a, $b, u8, 54),
            56 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48),
            57 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u8, 56),
            58 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u16, 56),
            59 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u16, 56) && cmp!($a, $b, u8, 58),
            60 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u32, 56),
            61 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u32, 56) && cmp!($a, $b, u8, 60),
            62 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u32, 56) && cmp!($a, $b, u16, 60),
            63 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u32, 56) && cmp!($a, $b, u16, 60) && cmp!($a, $b, u8, 62),
            64 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56),
            65 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u8, 64),
            66 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u16, 64),
            67 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u16, 64) && cmp!($a, $b, u8, 66),
            68 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u32, 64),
            69 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u32, 64) && cmp!($a, $b, u8, 68),
            70 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u32, 64) && cmp!($a, $b, u16, 68),
            71 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u32, 64) && cmp!($a, $b, u16, 68) && cmp!($a, $b, u8, 70),
            72 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64),
            73 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u8, 72),
            74 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u16, 72),
            75 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u16, 72) && cmp!($a, $b, u8, 74),
            76 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u32, 72),
            77 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u32, 72) && cmp!($a, $b, u8, 76),
            78 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u32, 72) && cmp!($a, $b, u16, 76),
            79 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u32, 72) && cmp!($a, $b, u16, 76) && cmp!($a, $b, u8, 78),
            80 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72),
            81 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u8, 80),
            82 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u16, 80),
            83 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u16, 80) && cmp!($a, $b, u8, 82),
            84 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u32, 80),
            85 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u32, 80) && cmp!($a, $b, u8, 84),
            86 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u32, 80) && cmp!($a, $b, u16, 84),
            87 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u32, 80) && cmp!($a, $b, u16, 84) && cmp!($a, $b, u8, 86),
            88 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80),
            89 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u8, 88),
            90 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u16, 88),
            91 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u16, 88) && cmp!($a, $b, u8, 90),
            92 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u32, 88),
            93 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u32, 88) && cmp!($a, $b, u8, 92),
            94 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u32, 88) && cmp!($a, $b, u16, 92),
            95 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u32, 88) && cmp!($a, $b, u16, 92) && cmp!($a, $b, u8, 94),
            96 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88),
            97 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u8, 96),
            98 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u16, 96),
            99 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u16, 96) && cmp!($a, $b, u8, 98),
            100 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u32, 96),
            101 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u32, 96) && cmp!($a, $b, u8, 100),
            102 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u32, 96) && cmp!($a, $b, u16, 100),
            103 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u32, 96) && cmp!($a, $b, u16, 100) && cmp!($a, $b, u8, 102),
            104 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96),
            105 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u8, 104),
            106 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u16, 104),
            107 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u16, 104) && cmp!($a, $b, u8, 106),
            108 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u32, 104),
            109 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u32, 104) && cmp!($a, $b, u8, 108),
            110 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u32, 104) && cmp!($a, $b, u16, 108),
            111 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u32, 104) && cmp!($a, $b, u16, 108) && cmp!($a, $b, u8, 110),
            112 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104),
            113 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u8, 112),
            114 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u16, 112),
            115 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u16, 112) && cmp!($a, $b, u8, 114),
            116 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u32, 112),
            117 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u32, 112) && cmp!($a, $b, u8, 116),
            118 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u32, 112) && cmp!($a, $b, u16, 116),
            119 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u32, 112) && cmp!($a, $b, u16, 116) && cmp!($a, $b, u8, 118),
            120 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112),
            121 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u8, 120),
            122 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u16, 120),
            123 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u16, 120) && cmp!($a, $b, u8, 122),
            124 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u32, 120),
            125 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u32, 120) && cmp!($a, $b, u8, 124),
            126 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u32, 120) && cmp!($a, $b, u16, 124),
            127 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u32, 120) && cmp!($a, $b, u16, 124) && cmp!($a, $b, u8, 126),
            128 => cmp!($a, $b, u64, 0) && cmp!($a, $b, u64, 8) && cmp!($a, $b, u64, 16) && cmp!($a, $b, u64, 24) && cmp!($a, $b, u64, 32) && cmp!($a, $b, u64, 40) && cmp!($a, $b, u64, 48) && cmp!($a, $b, u64, 56) && cmp!($a, $b, u64, 64) && cmp!($a, $b, u64, 72) && cmp!($a, $b, u64, 80) && cmp!($a, $b, u64, 88) && cmp!($a, $b, u64, 96) && cmp!($a, $b, u64, 104) && cmp!($a, $b, u64, 112) && cmp!($a, $b, u64, 120),
            _ => unsafe { memcmp($a, $b, $len) == 0 },
        }
    }}
);

/// Memory compare trait
pub trait Compare {
  /// Compares an `&[u8]` to another one
  fn feq(self: &Self, to: &Self) -> bool;
}

impl Compare for [u8] {
  #[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
  #[inline(always)]
  fn feq(&self, to: &[u8]) -> bool {

    // Fallback if the slices are too large
    extern "C" {
      fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
    }

    // Get the comparison pointers
    let a = to.as_ptr() as *const i8;
    let b = self.as_ptr() as *const i8;
    let len = to.len();

    // Do the comparison
    self.len() == len && slice_compare!(a, b, len)
  }
}