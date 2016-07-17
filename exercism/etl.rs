use std::collections::BTreeMap;

pub fn transform(input : &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter().flat_map(|(&k, v)| v.iter().map(move |s| (s.to_lowercase(), k))).collect()
}
