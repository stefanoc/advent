pub fn solve(input: &str) -> (usize, usize) {
    let mut result = (0, 0, 0);
    for line in input.lines() {
        let r = str_sizes(line);
        result.0 += r.0;
        result.1 += r.1;
        result.2 += re_encode(line);
    }
    (result.0 - result.1, result.2 - result.0)
}

#[derive(Debug)]
enum State {
    Initial,
    Quote,
    Escape,
    Escape1(char),
    Escape2(char, char),
    Any,
    Final
}

impl State {
    fn transition(self, input: char, length: &mut usize) -> State {
        let s = match (self, input) {
            (State::Initial, ch) if ch == '"' => State::Quote,
            (State::Initial, _)               => panic!("Illegal sequence (1)"),
            (State::Quote, '\\')              => State::Escape,
            (State::Quote,   _)               => { *length += 1; State::Any },
            (State::Any, ch) if ch == '\\'    => State::Escape,
            (State::Any, ch) if ch == '"'     => State::Final,
            (State::Any, _)                   => { *length += 1; State::Any },
            (State::Escape, ch) if ch == '"'  => { *length += 1; State::Any },
            (State::Escape, ch) if ch == '\\' => { *length += 1; State::Any },
            (State::Escape, ch)               => State::Escape1(ch),
            (State::Escape1(a), ch)           => State::Escape2(a, ch),
            (State::Escape2(_, _), _)         => { *length += 1; State::Any },
            (State::Final, _)                 => panic!("Illegal sequence (2)")
        };
        s
    }
}

fn str_sizes(input: &str) -> (usize, usize) {
    let mut state   = State::Initial;
    let mut length  = 0;

    for ch in input.chars() {
        state = state.transition(ch, &mut length);
    }
    (input.len(), length)
}

fn re_encode(input: &str) -> usize {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        encode(&ch, &mut out);
    }
    out.len() + 2
}

fn encode(input: &char, output: &mut String) {
    match *input {
        '"'     => { output.push('\\'); output.push('"'); },
        '\\'    => { output.push('\\'); output.push('\\'); },
        _       => output.push(*input)
    }
}
