
pub fn hex_to_int(s : &str) -> Option<i32> {
    let mut base = 1;
    let mut ans = 0;
    for c in s.chars().rev() {
        let mut t = c as i32 - '0' as i32;
        if t > 9 { t -= 39 }
        if t < 0 || t > 15 { return None }
        ans += t * base;
        base *= 16;
    }
    Some(ans)
}
