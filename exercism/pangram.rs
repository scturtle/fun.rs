
pub fn is_pangram(s : &str) -> bool {
    let s2 = s.to_lowercase();
    (0..26).map(|c| s2.contains((c + 97) as u8 as char)).all(|c|c)
}
