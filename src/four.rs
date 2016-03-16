use md5;

pub static INPUT: &'static str = "ckczppom";

macro_rules! md5(
    ($string:expr) => ({
        let mut context = md5::Context::new();
        context.consume($string.as_bytes());
        let mut digest = String::with_capacity(2 * 16);
        for x in &context.compute()[..] {
            digest.push_str(&format!("{:02x}", x));
        }
        digest
    });
);

pub fn solve(input: &str) -> (usize, usize) {
    let test = |n, pat| md5!(format!("{}{}", input, n)).starts_with(pat);
    ((0..).find(|&n| test(n, "00000")).unwrap(), (0..).find(|&n| test(n, "000000")).unwrap())
}
