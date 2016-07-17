
pub fn reply(s : &str) -> &str {
    if s.is_empty() {
        return "Fine. Be that way!"
    }
    if s.ends_with('?') {
        return "Sure."
    }
    if s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        return "Whoa, chill out!"
    }
    "Whatever."
}
