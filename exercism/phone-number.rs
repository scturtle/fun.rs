
pub fn number(s: &str) -> Option<String> {
    let nums : String = s.chars().filter(|c| c.is_digit(10)).collect();
    match nums.len() {
        10 => Some(nums),
        11 if &nums[0..1] == "1" => Some(nums[1..].into()),
        _ => None
    }
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|v| v[..3].into())
}


pub fn pretty_print(s: &str) -> String {
    match number(s) {
        None => "invalid".into(),
        Some(s) => format!("({}) {}-{}", &s[0..3], &s[3..6], &s[6..])
    }
}
