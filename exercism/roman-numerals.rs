pub fn from(n : i32) -> String {
    let mut t = n;
    let mut s = String::new();
    const L : [(i32, &'static str); 13] =
        [(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
         (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];
    for &(v, r) in &L {
        while t - v >= 0 {
            t -= v;
            s.push_str(r);
        }
        if t == 0 { break }
    }
    s
}
