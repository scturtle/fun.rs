use std::collections::BTreeMap;

pub struct School {
    mapping : BTreeMap<usize, Vec<String>>
}

impl School {
    pub fn new() -> School {
        return School{ mapping : BTreeMap::new() };
    }
    pub fn add(&mut self, grade : usize, name : &str) {
        let s = name.to_string();
        let v = self.mapping.entry(grade).or_insert(vec![]);
        match v.binary_search(&s) {
            Ok(_) => panic!("Name exists!"),
            Err(idx) => v.insert(idx, s),
        }
    }
    pub fn grades(&self) -> Vec<usize> {
        self.mapping.keys().cloned().collect()
    }
    pub fn grade(&self, grade : usize) -> Option<&Vec<String>> {
        self.mapping.get(&grade)
    }
}
