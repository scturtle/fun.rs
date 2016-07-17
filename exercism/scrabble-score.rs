
fn char2score(c : char) -> u32 {
    match c {
            'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
            'd'|'g' => 2,
            'b'|'c'|'m'|'p' => 3,
            'f'|'h'|'v'|'w'|'y' => 4,
            'k' => 5,
            'j'|'x' => 8,
            'q'|'z' => 10,
            _ => 0
    }
}

pub fn score(s : &str) -> u32 {
    s.to_lowercase().chars().map(char2score).fold(0, std::ops::Add::add)
}
