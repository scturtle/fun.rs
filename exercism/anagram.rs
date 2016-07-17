
pub fn anagrams_for<'a>(word: &'a str, inputs: &[&'a str]) -> Vec<&'a str> {
    let w_lc = word.to_lowercase();
    let mut w: Vec<char> = w_lc.chars().collect();
    w.sort();
    inputs.iter().filter(|v| {
        let v_lc = v.to_lowercase();
        let mut v: Vec<char> = v_lc.chars().collect();
        v.sort(); w == v && w_lc != v_lc
    }).cloned().collect()
}
