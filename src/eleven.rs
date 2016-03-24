use iterslide::SlideIterator;
use std::collections::HashMap;

pub fn solve(input: &str) -> (String, String) {
    let a = next_password(input);
    let b = next_password(&a);
    (a, b)
}

fn next_password(pwd: &str) -> String {
    let mut s = pwd.into();
    loop {
        let s_ = succ(&s);
        if is_good(&s_) { return s_; }
        s = s_;
    }
}

fn is_good(s: &String) -> bool {
    let has_seq = s.chars().slide(3).any(|seq| is_sequence(&seq));
    let has_iol = s.chars().any(|ch| ch == 'i' || ch == 'o' || ch == 'l');
    let mut pairs = HashMap::new();
    for pair in s.chars().slide(2).filter(|seq| seq[0] == seq[1]) {
        pairs.entry(pair).or_insert(true);
    }
    has_seq && !has_iol && pairs.keys().len() >= 2
}

fn is_sequence(chars: &[char]) -> bool {
    let (a, b, c) = (chars[0] as u8, chars[1] as u8, chars[2] as u8);
    a + 1 == b && b + 1 == c
}

fn succ(s: &String) -> String {
    _succ(&s, s.len() - 1)
}

fn _succ(s: &String, p: usize) -> String {
    let mut v = s.clone().into_bytes();
    if v[p] == 122 {
        if p == 0 {
            for i in 0 .. s.len() {
                v[i] = 97;
            }
            return String::from_utf8(v).unwrap();
        } else {
            for i in p .. s.len() {
                v[i] = 97;
            }
            return _succ(&String::from_utf8(v).unwrap(), p - 1);
        }
    } else {
        v[p] += 1;
        return String::from_utf8(v).unwrap();
    }
}
