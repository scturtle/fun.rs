use std::mem::transmute;

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut ans: Vec<u8> = Vec::new();
    for &n in values.iter() {
        let mut cur: Vec<u8> = Vec::new();
        let mut add = 0;
        let mut bit = 0;
        for v in unsafe{ transmute::<u32, [u8; 4]>(n).iter() } {
            let used = v & (0x7f >> bit);
            cur.push((used << bit) + add);
            add = v >> (7 - bit);
            bit += 1;
            if bit == 7 {
                cur.push(add);
                bit = 0;
            }
        }
        cur.push(add);
        while cur.last() == Some(&0) {cur.pop();}
        if cur.is_empty() {cur.push(0);}
        for v in &mut cur {*v |= 0x80;}
        cur.reverse();
        *cur.last_mut().unwrap() &= 0x80 - 1;
        ans.extend(cur.into_iter());
    }
    ans
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut ans: Vec<u32> = Vec::new();
    let mut curr: u32 = 0;
    for b in bytes {
        curr |= (b & 0x7f) as u32;
        if (b & 0x80) == 0 {
            ans.push(curr);
            curr = 0;
        } else {
            if curr >= (1 << 25) {return Err("overflow");}
            curr <<= 7;
        }
    }
    if curr != 0 {return Err("incomplete");}
    Ok(ans)
}
