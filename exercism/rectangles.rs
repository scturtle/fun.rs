
pub fn count(map: &[&str]) -> usize {
    let mut v: Vec<(usize, usize)> = Vec::new();
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '+' { v.push((i, j)) }
        }
    }
    if v.len() < 4 { return 0 }
    let mut cnt = 0;
    for (i, lt) in v.iter().enumerate().take(v.len()-3) {
        for (j, rt) in v.iter().enumerate().take(v.len()-2).skip(i+1) {
            if lt.0 != rt.0 { continue }
            for (k, lb) in v.iter().enumerate().take(v.len()-1).skip(j+1) {
                if lt.1 != lb.1 { continue }
                for rb in v.iter().skip(k+1) {
                    if lb.0 != rb.0 || rt.1 != rb.1 { continue }
                    if test(map, lt.0, lb.0, lt.1, rt.1) { cnt += 1 }
                }
            }
        }
    }
    cnt
}

fn test(map: &[&str], ia: usize, ib: usize, ja: usize, jb: usize) -> bool {
    map[ia+1..ib].iter().all(|&l| {
        "+|".contains(l.chars().nth(ja).unwrap()) &&
            "+|".contains(l.chars().nth(jb).unwrap())
    })
        &&
        map[ia][ja+1..jb].chars().all(|c| c=='+'||c=='-') &&
        map[ib][ja+1..jb].chars().all(|c| c=='+'||c=='-')
}
