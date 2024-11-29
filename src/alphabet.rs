use std::collections::HashMap;

#[derive(Clone)]
pub enum Font {
    Default,
    Mario,
}

pub fn get_alphabet(font: Option<Font>) -> HashMap<char, Vec<&'static str>> {
    let mut alphabet = HashMap::new();

    match font {
        Some(f) => match f {
            Font::Mario => default_font(&mut alphabet),
            Font::Default => default_font(&mut alphabet),
        },
        None => default_font(&mut alphabet),
    }

    alphabet
}

fn default_font(alphabet: &mut HashMap<char, Vec<&str>>) {
    alphabet.insert(
        'a',
        vec![
            "     ", "11111", "1   1", "11111", "1   1", "1   1", "     ",
        ],
    );
    alphabet.insert(
        'b',
        vec![
            "     ", "1111 ", "1   1", "11111", "1   1", "1111 ", "     ",
        ],
    );
    alphabet.insert(
        'c',
        vec![
            "     ", "11111", "1    ", "1    ", "1    ", "11111", "     ",
        ],
    );
    alphabet.insert(
        'd',
        vec![
            "     ", "1111 ", "1   1", "1   1", "1   1", "1111 ", "     ",
        ],
    );
    alphabet.insert(
        'e',
        vec![
            "     ", "11111", "1    ", "11111", "1    ", "11111", "     ",
        ],
    );
    alphabet.insert(
        'f',
        vec![
            "     ", "11111", "1    ", "1111 ", "1    ", "1    ", "     ",
        ],
    );
    alphabet.insert(
        'g',
        vec![
            "     ", "11111", "1    ", "1 111", "1   1", "11111", "     ",
        ],
    );
    alphabet.insert(
        'h',
        vec![
            "     ", "1   1", "1   1", "11111", "1   1", "1   1", "     ",
        ],
    );
    alphabet.insert(
        'i',
        vec![
            "     ", "11111", "  1  ", "  1  ", "  1  ", "11111", "     ",
        ],
    );
    alphabet.insert(
        'j',
        vec![
            "     ", "11111", "  1  ", "  1  ", "1 1  ", "111  ", "     ",
        ],
    );
    alphabet.insert(
        'k',
        vec![
            "     ", "1  1 ", "1 1  ", "11   ", "1 1  ", "1  1 ", "     ",
        ],
    );
    alphabet.insert(
        'l',
        vec![
            "     ", "1    ", "1    ", "1    ", "1    ", "11111", "     ",
        ],
    );
    alphabet.insert(
        'm',
        vec![
            "     ", "1   1", "11 11", "1 1 1", "1   1", "1   1", "     ",
        ],
    );
    alphabet.insert(
        'n',
        vec![
            "     ", "1   1", "11  1", "1 1 1", "1  11", "1   1", "     ",
        ],
    );
    alphabet.insert(
        'o',
        vec![
            "     ", "11111", "1   1", "1   1", "1   1", "11111", "     ",
        ],
    );
    alphabet.insert(
        'p',
        vec![
            "     ", "11111", "1   1", "11111", "1    ", "1    ", "     ",
        ],
    );
    alphabet.insert(
        'q',
        vec![
            "     ", "11111", "1   1", "1   1", "1  11", "11111", "     ",
        ],
    );
    alphabet.insert(
        'r',
        vec![
            "     ", "11111", "1   1", "11111", "1  1 ", "1   1", "     ",
        ],
    );
    alphabet.insert(
        's',
        vec![
            "     ", "11111", "1    ", "11111", "    1", "11111", "     ",
        ],
    );
    alphabet.insert(
        't',
        vec![
            "     ", "11111", "  1  ", "  1  ", "  1  ", "  1  ", "     ",
        ],
    );
    alphabet.insert(
        'u',
        vec![
            "     ", "1   1", "1   1", "1   1", "1   1", "11111", "     ",
        ],
    );
    alphabet.insert(
        'v',
        vec![
            "     ", "1   1", "1   1", " 1 1 ", " 1 1 ", "  1  ", "     ",
        ],
    );
    alphabet.insert(
        'w',
        vec![
            "     ", "1   1", "1   1", "1   1", "1 1 1", "11 11", "     ",
        ],
    );
    alphabet.insert(
        'x',
        vec![
            "     ", "1   1", " 1 1 ", "  1  ", " 1 1 ", "1   1", "     ",
        ],
    );
    alphabet.insert(
        'y',
        vec![
            "     ", "1   1", " 1 1 ", "  1  ", "  1  ", "  1  ", "     ",
        ],
    );
    alphabet.insert(
        'z',
        vec![
            "     ", "11111", "   1 ", "  1  ", " 1   ", "11111", "     ",
        ],
    );
    alphabet.insert(
        '0',
        vec![
            "     ", " 111 ", "1   1", "1   1", "1   1", " 111 ", "     ",
        ],
    );
    alphabet.insert('1', vec!["  ", "11", " 1", " 1", " 1", " 1", "  "]);
    alphabet.insert(
        '2',
        vec![
            "     ", " 111 ", "1   1", "   1 ", "  1  ", "11111", "     ",
        ],
    );
    alphabet.insert(
        '3',
        vec![
            "     ", "11111", "    1", "  111", "    1", "11111", "     ",
        ],
    );
    alphabet.insert(
        '4',
        vec![
            "     ", "1   1", "1   1", "11111", "    1", "    1", "     ",
        ],
    );
    alphabet.insert(
        '5',
        vec![
            "     ", " 1111", "1    ", "11111", "    1", "11111", "     ",
        ],
    );
    alphabet.insert(
        '6',
        vec![
            "     ", "11111", "1    ", "11111", "1   1", "11111", "     ",
        ],
    );
    alphabet.insert(
        '7',
        vec![
            "     ", "11111", "    1", "   1 ", "  1  ", "1    ", "     ",
        ],
    );
    alphabet.insert(
        '8',
        vec![
            "     ", "11111", "1   1", " 111 ", "1   1", "11111", "     ",
        ],
    );
    alphabet.insert(
        '9',
        vec![
            "     ", "11111", "1   1", "11111", "    1", "11111", "     ",
        ],
    );
    alphabet.insert(
        '-',
        vec![
            "     ", "     ", "     ", " 111 ", "     ", "     ", "     ",
        ],
    );
    alphabet.insert(
        '_',
        vec![
            "     ", "     ", "     ", "     ", "     ", "11111", "     ",
        ],
    );
    alphabet.insert(
        '.',
        vec![
            "     ", "     ", "     ", "     ", "     ", "  1  ", "     ",
        ],
    );
    alphabet.insert(
        '/',
        vec![
            "     ", "    1", "   1 ", "  1  ", " 1   ", "1    ", "     ",
        ],
    );
    alphabet.insert(
        '\\',
        vec![
            "     ", "1    ", " 1   ", "  1  ", "   1 ", "    1", "     ",
        ],
    );
    alphabet.insert(')', vec!["   ", "1  ", " 1 ", "  1", " 1 ", "1  ", "   "]);
    alphabet.insert('(', vec!["   ", "  1", " 1 ", "1  ", " 1 ", "  1", "   "]);
    alphabet.insert(
        '[',
        vec!["    ", " 111", " 1  ", " 1  ", " 1  ", " 111", "    "],
    );
    alphabet.insert(
        ']',
        vec!["    ", "111 ", "  1 ", "  1 ", "  1 ", "111 ", "    "],
    );
    alphabet.insert(
        ':',
        vec![
            "     ", "     ", "  1  ", "     ", "  1  ", "     ", "     ",
        ],
    );
    alphabet.insert(
        '*',
        vec![
            "     ", "     ", "1 1 1", "  1  ", "1 1 1", "     ", "     ",
        ],
    );
    alphabet.insert(
        '!',
        vec![
            "     ", "  1  ", "  1  ", "  1  ", "     ", "  1  ", "     ",
        ],
    );
}

pub const WEEK_DAYS: usize = 7;
pub const CHARS: [char; 4] = ['░', '▒', '▓', '█'];
