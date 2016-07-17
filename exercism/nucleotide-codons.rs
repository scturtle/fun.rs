use std::collections::HashMap;

pub struct NameFor(HashMap<String, String>);

pub fn parse(v: Vec<(&str, &str)>) -> NameFor {
    NameFor(v.into_iter().map(|(a, b)| (a.into(), b.into())).collect())
}

impl NameFor {
    pub fn name_for(&self, codon: &str) -> Result<&str, ()> {
        let c = codon.replace("M", "A").replace("R", "A").replace("Y", "C")
                     .replace("H", "A").replace("N", "A"); // TODO
        self.0.get(&c).map(|s| s.as_str()).ok_or(())
    }
}
