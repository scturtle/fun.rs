use std::collections::HashMap;

pub fn count(c : char, s : &str) -> usize {
    s.matches(c).count()
}

pub fn nucleotide_counts(s : &str) -> HashMap<char, usize> {
    "ACGT".chars().map(|c| (c, count(c, s))).collect()
}
