use std::collections::BTreeSet;
use std::iter::FromIterator;

static TESTPASSPHRASES: &'static str = include_str!("dayfour.txt");
//static TESTPASSPHRASES: &'static str = include_str!("dayfourtest.txt");

pub fn day() -> String {
    TESTPASSPHRASES.lines().fold(0, |so_far, line| {
        let v = Vec::from_iter(line.split_whitespace());
        let s = BTreeSet::from_iter(line.split_whitespace());
        if v.len() == s.len() {
            so_far + 1
        } else {
            so_far
        }
    }).to_string()
}
