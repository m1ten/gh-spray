// Fonts sourced from https://github.com/Annihil/github-spray

#[derive(Clone)]
pub struct Font(&'static [(char, [&'static str; 7]); 49]);

impl Font {
    pub fn new(name: &str) -> Self {
        Font(match name.to_ascii_lowercase().as_str() {
            "mario" => &MARIO_PATTERNS,
            "lowercase" => &LOWERCASE_PATTERNS,
            _ => &DEFAULT_PATTERNS,
        })
    }

    pub fn get_patterns(&self) -> &'static [(char, [&'static str; 7]); 49] {
        self.0
    }
}

const DEFAULT_PATTERNS: [(char, [&str; 7]); 49] = [
    (
        'a',
        [
            "     ", "11111", "1   1", "11111", "1   1", "1   1", "     ",
        ],
    ),
    (
        'b',
        [
            "     ", "1111 ", "1   1", "11111", "1   1", "1111 ", "     ",
        ],
    ),
    (
        'c',
        [
            "     ", "11111", "1    ", "1    ", "1    ", "11111", "     ",
        ],
    ),
    (
        'd',
        [
            "     ", "1111 ", "1   1", "1   1", "1   1", "1111 ", "     ",
        ],
    ),
    (
        'e',
        [
            "     ", "11111", "1    ", "11111", "1    ", "11111", "     ",
        ],
    ),
    (
        'f',
        [
            "     ", "11111", "1    ", "1111 ", "1    ", "1    ", "     ",
        ],
    ),
    (
        'g',
        [
            "     ", "11111", "1    ", "1 111", "1   1", "11111", "     ",
        ],
    ),
    (
        'h',
        [
            "     ", "1   1", "1   1", "11111", "1   1", "1   1", "     ",
        ],
    ),
    (
        'i',
        [
            "     ", "11111", "  1  ", "  1  ", "  1  ", "11111", "     ",
        ],
    ),
    (
        'j',
        [
            "     ", "11111", "  1  ", "  1  ", "1 1  ", "111  ", "     ",
        ],
    ),
    (
        'k',
        [
            "     ", "1  1 ", "1 1  ", "11   ", "1 1  ", "1  1 ", "     ",
        ],
    ),
    (
        'l',
        [
            "     ", "1    ", "1    ", "1    ", "1    ", "11111", "     ",
        ],
    ),
    (
        'm',
        [
            "     ", "1   1", "11 11", "1 1 1", "1   1", "1   1", "     ",
        ],
    ),
    (
        'n',
        [
            "     ", "1   1", "11  1", "1 1 1", "1  11", "1   1", "     ",
        ],
    ),
    (
        'o',
        [
            "     ", "11111", "1   1", "1   1", "1   1", "11111", "     ",
        ],
    ),
    (
        'p',
        [
            "     ", "11111", "1   1", "11111", "1    ", "1    ", "     ",
        ],
    ),
    (
        'q',
        [
            "     ", "11111", "1   1", "1   1", "1  11", "11111", "     ",
        ],
    ),
    (
        'r',
        [
            "     ", "11111", "1   1", "11111", "1  1 ", "1   1", "     ",
        ],
    ),
    (
        's',
        [
            "     ", "11111", "1    ", "11111", "    1", "11111", "     ",
        ],
    ),
    (
        't',
        [
            "     ", "11111", "  1  ", "  1  ", "  1  ", "  1  ", "     ",
        ],
    ),
    (
        'u',
        [
            "     ", "1   1", "1   1", "1   1", "1   1", "11111", "     ",
        ],
    ),
    (
        'v',
        [
            "     ", "1   1", "1   1", " 1 1 ", " 1 1 ", "  1  ", "     ",
        ],
    ),
    (
        'w',
        [
            "     ", "1   1", "1   1", "1   1", "1 1 1", "11 11", "     ",
        ],
    ),
    (
        'x',
        [
            "     ", "1   1", " 1 1 ", "  1  ", " 1 1 ", "1   1", "     ",
        ],
    ),
    (
        'y',
        [
            "     ", "1   1", " 1 1 ", "  1  ", "  1  ", "  1  ", "     ",
        ],
    ),
    (
        'z',
        [
            "     ", "11111", "   1 ", "  1  ", " 1   ", "11111", "     ",
        ],
    ),
    (
        '0',
        [
            "     ", " 111 ", "1   1", "1   1", "1   1", " 111 ", "     ",
        ],
    ),
    ('1', ["  ", "11", " 1", " 1", " 1", " 1", "  "]),
    (
        '2',
        [
            "     ", " 111 ", "1   1", "   1 ", "  1  ", "11111", "     ",
        ],
    ),
    (
        '3',
        [
            "     ", "11111", "    1", "  111", "    1", "11111", "     ",
        ],
    ),
    (
        '4',
        [
            "     ", "1   1", "1   1", "11111", "    1", "    1", "     ",
        ],
    ),
    (
        '5',
        [
            "     ", " 1111", "1    ", "11111", "    1", "11111", "     ",
        ],
    ),
    (
        '6',
        [
            "     ", "11111", "1    ", "11111", "1   1", "11111", "     ",
        ],
    ),
    (
        '7',
        [
            "     ", "11111", "    1", "   1 ", "  1  ", "1    ", "     ",
        ],
    ),
    (
        '8',
        [
            "     ", "11111", "1   1", " 111 ", "1   1", "11111", "     ",
        ],
    ),
    (
        '9',
        [
            "     ", "11111", "1   1", "11111", "    1", "11111", "     ",
        ],
    ),
    (
        '-',
        [
            "     ", "     ", "     ", " 111 ", "     ", "     ", "     ",
        ],
    ),
    (
        '_',
        [
            "     ", "     ", "     ", "     ", "     ", "11111", "     ",
        ],
    ),
    (
        '.',
        [
            "     ", "     ", "     ", "     ", "     ", "  1  ", "     ",
        ],
    ),
    (
        '/',
        [
            "     ", "    1", "   1 ", "  1  ", " 1   ", "1    ", "     ",
        ],
    ),
    (
        '\\',
        [
            "     ", "1    ", " 1   ", "  1  ", "   1 ", "    1", "     ",
        ],
    ),
    (')', ["   ", "1  ", " 1 ", "  1", " 1 ", "1  ", "   "]),
    ('(', ["   ", "  1", " 1 ", "1  ", " 1 ", "  1", "   "]),
    (
        '[',
        ["    ", " 111", " 1  ", " 1  ", " 1  ", " 111", "    "],
    ),
    (
        ']',
        ["    ", "111 ", "  1 ", "  1 ", "  1 ", "111 ", "    "],
    ),
    (
        ':',
        [
            "     ", "     ", "  1  ", "     ", "  1  ", "     ", "     ",
        ],
    ),
    (
        '*',
        [
            "     ", "     ", "1 1 1", "  1  ", "1 1 1", "     ", "     ",
        ],
    ),
    (
        '!',
        [
            "     ", "  1  ", "  1  ", "  1  ", "     ", "  1  ", "     ",
        ],
    ),
    (
        ' ',
        [
            "     ", "     ", "     ", "     ", "     ", "     ", "     ",
        ],
    ),
];

// mario font
const MARIO_PATTERNS: [(char, [&str; 7]); 49] = [
    (
        'a',
        [
            "       ", "       ", " 1111  ", "    11 ", " 11111 ", "11  11 ", " 111 11",
        ],
    ),
    (
        'b',
        [
            "      ", "11    ", "11    ", "11111 ", "11  11", "11  11", "11111 ",
        ],
    ),
    (
        'c',
        [
            "      ", "      ", " 11111", "11    ", "11    ", "11    ", " 11111",
        ],
    ),
    (
        'd',
        [
            "      ", "    11", "    11", " 11111", "11  11", "11  11", " 11111",
        ],
    ),
    (
        'e',
        [
            "      ", "      ", " 1111 ", "11  11", "111111", "11    ", " 1111 ",
        ],
    ),
    (
        'f',
        [
            "      ", "   111", "  11  ", "111111", "  11  ", "  11  ", "  11  ",
        ],
    ),
    (
        'g',
        [
            "      ", " 1111 ", "11  11", "11  11", " 11111", "    11", " 1111 ",
        ],
    ),
    (
        'h',
        [
            "      ", "11    ", "11    ", "11111 ", "11  11", "11  11", "11  11",
        ],
    ),
    ('i', ["  ", "11", "  ", "11", "11", "11", "11"]),
    ('j', ["   ", " 11", "   ", " 11", " 11", " 11", "11 "]),
    (
        'k',
        [
            "      ", "11    ", "11  11", "11 11 ", "1111  ", "11 11 ", "11  11",
        ],
    ),
    ('l', ["  ", "11", "11", "11", "11", "11", "11"]),
    (
        'm',
        [
            "      ", "      ", "11111 ", "1 1 11", "1 1 11", "1 1 11", "1 1 11",
        ],
    ),
    (
        'n',
        [
            "     ", "     ", "1111 ", "1  11", "1  11", "1  11", "1  11",
        ],
    ),
    (
        'o',
        [
            "      ", "      ", " 1111 ", "11  11", "11  11", "11  11", " 1111 ",
        ],
    ),
    (
        'p',
        [
            "      ", "      ", "11111 ", "11  11", "11  11", "11111 ", "11    ",
        ],
    ),
    (
        'q',
        [
            "      ", "      ", " 11111", "11  11", "11  11", " 11111", "    11",
        ],
    ),
    (
        'r',
        [
            "     ", "     ", "1 111", "111  ", "11   ", "11   ", "11   ",
        ],
    ),
    (
        's',
        [
            "     ", "     ", " 111 ", "11   ", " 111 ", "   11", " 111 ",
        ],
    ),
    (
        't',
        [
            "      ", "      ", "  11  ", "111111", "  11  ", "  11  ", "  11  ",
        ],
    ),
    (
        'u',
        [
            "      ", "      ", "11  11", "11  11", "11  11", "11  11", " 11111",
        ],
    ),
    (
        'v',
        [
            "      ", "      ", "11  11", "11  11", "11  11", " 1  1 ", "  11  ",
        ],
    ),
    (
        'w',
        [
            "      ", "      ", "1 1 11", "1 1 11", "1 1 11", "1 1 11", "11111 ",
        ],
    ),
    (
        'x',
        [
            "      ", "      ", "11  11", " 1111 ", "  11  ", " 1111 ", "11  11",
        ],
    ),
    (
        'y',
        [
            "      ", "      ", "11  11", "11  11", " 11111", "    11", " 1111 ",
        ],
    ),
    (
        'z',
        [
            "      ", "      ", "111111", "   11 ", "  11  ", " 11   ", "111111",
        ],
    ),
    (
        '0',
        [
            "      ", "      ", " 1111 ", "11  11", "11  11", "11  11", " 1111 ",
        ],
    ),
    (
        '1',
        [
            "     ", "     ", "111  ", " 11  ", " 11  ", " 11  ", "1111 ",
        ],
    ),
    (
        '2',
        [
            "      ", "      ", " 1111 ", "11  11", "   11 ", "  11  ", "111111",
        ],
    ),
    (
        '3',
        [
            "      ", "      ", "11111 ", "    11", "  111 ", "    11", "11111 ",
        ],
    ),
    (
        '4',
        [
            "      ", "      ", "  111 ", " 1 11 ", "1  11 ", "111111", "   11 ",
        ],
    ),
    (
        '5',
        [
            "      ", "      ", "11111 ", "11    ", "11111 ", "    11", "11111 ",
        ],
    ),
    (
        '6',
        [
            "      ", "      ", " 1111 ", "11    ", "11111 ", "11  11", " 1111 ",
        ],
    ),
    (
        '7',
        [
            "      ", "      ", "111111", "11  11", "   11 ", "  11  ", "  11  ",
        ],
    ),
    (
        '8',
        [
            "      ", "      ", " 1111 ", "11  11", " 1111 ", "11  11", " 1111 ",
        ],
    ),
    (
        '9',
        [
            "      ", "      ", " 1111 ", "11  11", " 11111", "    11", " 1111 ",
        ],
    ),
    (
        '-',
        [
            "      ", "      ", "      ", "111111", "      ", "      ", "      ",
        ],
    ),
    (
        '_',
        [
            "      ", "      ", "      ", "      ", "      ", "111111", "      ",
        ],
    ),
    ('.', ["  ", "  ", "  ", "  ", "  ", "11", "11"]),
    (
        '/',
        [
            "     ", "   1 ", "  1  ", " 1   ", "1    ", "     ", "     ",
        ],
    ),
    (
        '\\',
        [
            "     ", "1    ", " 1   ", "  1  ", "   1 ", "     ", "     ",
        ],
    ),
    (
        ')',
        [
            "     ", "1    ", " 1   ", "  1  ", " 1   ", "1    ", "     ",
        ],
    ),
    (
        '(',
        [
            "     ", "  1  ", " 1   ", "1    ", " 1   ", "  1  ", "     ",
        ],
    ),
    (
        '[',
        [
            "     ", " 11  ", " 1   ", " 1   ", " 1   ", " 11  ", "     ",
        ],
    ),
    (
        ']',
        [
            "     ", "11   ", " 1   ", " 1   ", " 1   ", "11   ", "     ",
        ],
    ),
    (
        ':',
        [
            "     ", "     ", " 1   ", "     ", " 1   ", "     ", "     ",
        ],
    ),
    (
        '*',
        [
            "     ", "     ", "1 1  ", " 1   ", "1 1  ", "     ", "     ",
        ],
    ),
    (
        '!',
        [
            "     ", "    1", "   11", "   1 ", "  1  ", "     ", "1    ",
        ],
    ),
    (
        ' ',
        [
            "      ", "      ", "      ", "      ", "      ", "      ", "      ",
        ],
    ),
];

const LOWERCASE_PATTERNS: [(char, [&str; 7]); 49] = [
    (
        'a',
        [
            "     ", "     ", " 111 ", "   1 ", " 1111", "1  1 ", " 111 ",
        ],
    ),
    (
        'b',
        [
            "     ", "1    ", "111  ", "1  1 ", "1  1 ", "1  1 ", "111  ",
        ],
    ),
    (
        'c',
        [
            "     ", "     ", " 111 ", "1    ", "1    ", "1    ", " 111 ",
        ],
    ),
    (
        'd',
        [
            "     ", "   1 ", " 111 ", "1  1 ", "1  1 ", "1  1 ", " 111 ",
        ],
    ),
    (
        'e',
        [
            "     ", "     ", " 11  ", "1  1 ", "111  ", "1    ", " 111 ",
        ],
    ),
    (
        'f',
        [
            "     ", "  11 ", " 1   ", "111  ", " 1   ", " 1   ", " 1   ",
        ],
    ),
    (
        'g',
        [
            "     ", "     ", " 111 ", "1  1 ", " 111 ", "   1 ", "111  ",
        ],
    ),
    (
        'h',
        [
            "     ", "1    ", "111  ", "1  1 ", "1  1 ", "1  1 ", "1  1 ",
        ],
    ),
    (
        'i',
        [
            "     ", " 1   ", "     ", "11   ", " 1   ", " 1   ", "111  ",
        ],
    ),
    (
        'j',
        [
            "     ", "  1  ", "     ", "  1  ", "  1  ", "1 1  ", " 1   ",
        ],
    ),
    (
        'k',
        [
            "     ", "1    ", "1  1 ", "1 1  ", "11   ", "1 1  ", "1  1 ",
        ],
    ),
    (
        'l',
        [
            "     ", "11   ", " 1   ", " 1   ", " 1   ", " 1   ", "111  ",
        ],
    ),
    (
        'm',
        [
            "     ", "     ", "11 1 ", "1 1 1", "1 1 1", "1   1", "1   1",
        ],
    ),
    (
        'n',
        [
            "     ", "     ", "111  ", "1  1 ", "1  1 ", "1  1 ", "1  1 ",
        ],
    ),
    (
        'o',
        [
            "     ", "     ", " 11  ", "1  1 ", "1  1 ", "1  1 ", " 11  ",
        ],
    ),
    (
        'p',
        [
            "     ", "     ", "111  ", "1  1 ", "111  ", "1    ", "1    ",
        ],
    ),
    (
        'q',
        [
            "     ", "     ", " 111 ", "1  1 ", " 111 ", "   1 ", "   1 ",
        ],
    ),
    (
        'r',
        [
            "     ", "     ", "1 11 ", "11   ", "1    ", "1    ", "1    ",
        ],
    ),
    (
        's',
        [
            "     ", "     ", " 111 ", "1    ", " 11  ", "   1 ", "111  ",
        ],
    ),
    (
        't',
        [
            "     ", " 1   ", "111  ", " 1   ", " 1   ", " 1   ", "  11 ",
        ],
    ),
    (
        'u',
        [
            "     ", "     ", "1  1 ", "1  1 ", "1  1 ", "1  1 ", " 111 ",
        ],
    ),
    (
        'v',
        [
            "     ", "     ", "1  1 ", "1  1 ", "1  1 ", " 11  ", "  1  ",
        ],
    ),
    (
        'w',
        [
            "     ", "     ", "1   1", "1   1", "1 1 1", "1 1 1", " 1 1 ",
        ],
    ),
    (
        'x',
        [
            "     ", "     ", "1  1 ", " 11  ", "  1  ", " 11  ", "1  1 ",
        ],
    ),
    (
        'y',
        [
            "     ", "     ", "1  1 ", "1  1 ", " 111 ", "   1 ", "111  ",
        ],
    ),
    (
        'z',
        [
            "     ", "     ", "1111 ", "  1  ", " 1   ", "1    ", "1111 ",
        ],
    ),
    ('0', DEFAULT_PATTERNS[26].1),
    ('1', DEFAULT_PATTERNS[27].1),
    ('2', DEFAULT_PATTERNS[28].1),
    ('3', DEFAULT_PATTERNS[29].1),
    ('4', DEFAULT_PATTERNS[30].1),
    ('5', DEFAULT_PATTERNS[31].1),
    ('6', DEFAULT_PATTERNS[32].1),
    ('7', DEFAULT_PATTERNS[33].1),
    ('8', DEFAULT_PATTERNS[34].1),
    ('9', DEFAULT_PATTERNS[35].1),
    ('-', DEFAULT_PATTERNS[36].1),
    ('_', DEFAULT_PATTERNS[37].1),
    ('.', DEFAULT_PATTERNS[38].1),
    ('/', DEFAULT_PATTERNS[39].1),
    ('\\', DEFAULT_PATTERNS[40].1),
    (')', DEFAULT_PATTERNS[41].1),
    ('(', DEFAULT_PATTERNS[42].1),
    ('[', DEFAULT_PATTERNS[43].1),
    (']', DEFAULT_PATTERNS[44].1),
    (':', DEFAULT_PATTERNS[45].1),
    ('*', DEFAULT_PATTERNS[46].1),
    ('!', DEFAULT_PATTERNS[47].1),
    (' ', DEFAULT_PATTERNS[48].1),
];

pub const WEEK_DAYS: usize = 7;
pub const CHARS: [char; 4] = ['░', '▒', '▓', '█'];
