// 3113322113
pub fn solve(input: &str) -> (usize, usize) {
    (say_n(input, 40), say_n(input, 50))
}

struct Sequence(char, u8);

fn say_n(s: &str, n: usize) -> usize {
    let mut out = String::from(s);
    for _ in 0..n {
        out = say(&out);
    }
    out.len()
}

fn say(s: &str) -> String {
    let mut out = String::from("");
    let mut chr = s.chars();
    let mut seq = Sequence(chr.next().unwrap(), 1);
    while let Some(c) = chr.next() {
        if c == seq.0 {
            seq.1 += 1;
        } else {
            out.push_str(&format!("{}{}", seq.1, seq.0));
            seq.0 = c;
            seq.1 = 1;
        }
    }
    out.push_str(&format!("{}{}", seq.1, seq.0));
    out
}
