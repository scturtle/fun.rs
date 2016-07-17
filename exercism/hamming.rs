
pub fn hamming_distance(a : &str, b : &str) -> Result<u32, &'static str> {
    if a.len() != b.len() {
        Err("inputs of different length")
    } else {
        Ok(a.chars().zip(b.chars())
            .map(|(c1,c2)| if c1 == c2 {0} else {1})
            .fold(0, |a,b| a+b))
    }
}
