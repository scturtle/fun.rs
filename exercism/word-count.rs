use std::collections::HashMap;

pub fn word_count(s : &str) -> HashMap<String, u32> {
    let mut m: HashMap<String, u32> = HashMap::new();
    for w in s.split(|c : char| !c.is_alphanumeric()) {
        if w.is_empty() { continue }
        let c = m.entry(w.to_lowercase()).or_insert(0);
        *c += 1;
    }
    m
}
