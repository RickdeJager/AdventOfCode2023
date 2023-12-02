use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1(input: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in input {
        let first = line
            .chars()
            .find(char::is_ascii_digit)
            .expect("No first digit found")
            .to_digit(10)
            .unwrap() as usize;
        let last = line
            .chars()
            .rfind(char::is_ascii_digit)
            .expect("No last digit found")
            .to_digit(10)
            .unwrap() as usize;
        sum += first * 10 + last;
    }

    sum
}

const TOKENS: [(&str, usize); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn part2(input: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in input {
        let first = TOKENS
            .iter()
            .filter_map(|(token, value)| {
                let idx = line.find(token)?;
                Some((idx, *value))
            })
            .min_by_key(|(idx, _value)| *idx)
            .expect("No first token found")
            .1;

        let last = TOKENS
            .iter()
            .filter_map(|(token, value)| {
                let idx = line.rfind(token)?;
                Some((idx, *value))
            })
            .max_by_key(|(idx, _value)| *idx)
            .expect("No last token found")
            .1;

        sum += first * 10 + last;
    }

    sum
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Nothing,
    Done,
    O,
    ON,
    T,
    TW,
    TH,
    THR,
    THRE,
    F,
    FO,
    FOU,
    FI,
    FIV,
    S,
    SI,
    SE,
    SEV,
    SEVE,
    E,
    EI,
    EIG,
    EIGH,
    N,
    NI,
    NIN,
}

type Transition = (State, u32);

const fn create_table() -> [[Transition; 128]; 26]{
    let mut table = [[(State::Nothing, 0u32); 128]; 26];

    // A digit in any position should always result in a transition to the Done state.
    let mut state = 0;
    while state < 26 {
        let mut chr = 0x30usize;
        while chr < 0x3Ausize {
            table[state][chr] = (State::Done, (chr-0x30) as u32);
            chr += 1;
        }

        table[state]['o' as usize] = (State::O, 0);
        table[state]['t' as usize] = (State::T, 0);
        table[state]['f' as usize] = (State::F, 0);
        table[state]['s' as usize] = (State::S, 0);
        table[state]['e' as usize] = (State::E, 0);
        table[state]['n' as usize] = (State::N, 0);

        state += 1;
    }

    // ONE
    table[State::O as usize]['n' as usize] = (State::ON, 0);
    table[State::FO as usize]['n' as usize] = (State::ON, 0);
    table[State::ON as usize]['e' as usize] = (State::Done, 1);

    // TWO
    table[State::T as usize]['w' as usize] = (State::TW, 0);
    table[State::TW as usize]['o' as usize] = (State::Done, 2);

    // THREE
    table[State::T as usize]['h' as usize] = (State::TH, 0);
    table[State::TH as usize]['r' as usize] = (State::THR, 0);
    table[State::THR as usize]['e' as usize] = (State::THRE, 0);
    table[State::THRE as usize]['e' as usize] = (State::Done, 3);

    // FOUR
    table[State::F as usize]['o' as usize] = (State::FO, 0);
    table[State::FO as usize]['u' as usize] = (State::FOU, 0);
    table[State::FOU as usize]['r' as usize] = (State::Done, 4);

    // FIVE
    table[State::F as usize]['i' as usize] = (State::FI, 0);
    table[State::FI as usize]['v' as usize] = (State::FIV, 0);
    table[State::FIV as usize]['e' as usize] = (State::Done, 5);

    // SIX
    table[State::S as usize]['i' as usize] = (State::SI, 0);
    table[State::SI as usize]['x' as usize] = (State::Done, 6);
    
    // SEVEN
    table[State::S as usize]['e' as usize] = (State::SE, 0);
    table[State::SE as usize]['v' as usize] = (State::SEV, 0);
    table[State::SEV as usize]['e' as usize] = (State::SEVE, 0);
    table[State::SEVE as usize]['n' as usize] = (State::Done, 7);

    // EIGHT
    table[State::E as usize]['i' as usize] = (State::EI, 0);
    table[State::THRE as usize]['i' as usize] = (State::EI, 0);
    table[State::SE as usize]['i' as usize] = (State::EI, 0);
    table[State::SEVE as usize]['i' as usize] = (State::EI, 0);
    table[State::EI as usize]['g' as usize] = (State::EIG, 0);
    table[State::EIG as usize]['h' as usize] = (State::EIGH, 0);
    table[State::EIGH as usize]['t' as usize] = (State::Done, 8);

    // NINE
    table[State::N as usize]['i' as usize] = (State::NI, 0);
    table[State::ON as usize]['i' as usize] = (State::NI, 0);
    table[State::NI as usize]['n' as usize] = (State::NIN, 0);
    table[State::NIN as usize]['e' as usize] = (State::Done, 9);

    table
}

pub fn part2a(input: &Vec<String>) -> usize {
    let table = create_table();
    
    let mut sum = 0;
    for line in input {
        let mut state = State::Nothing;
        let mut first = 0usize;
        let mut last = 0usize;
        for chr in line.chars() {
            let (new_state, value) = table[state as usize][chr as usize];
            state = new_state;
            if state == State::Done {
                if first == 0 {
                    first = value as usize;
                }
                last = value as usize;
                
                // reset state to deal with overlap (Numbers will never overlap)
                state = table[State::Nothing as usize][chr as usize].0;
            }
        }
        sum += first * 10 + last;
    }
    sum

}

pub fn read_input(filename: &str) -> Vec<String> {
    let mut input = Vec::new();
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}
