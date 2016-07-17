use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal, Sublist, Superlist, Unequal
}
use Comparison::*;

fn horspool<T: Eq+Hash>(haystack: &[T], needle: &[T]) -> bool {
    let hlen = haystack.len();
    let nlen = needle.len();
    if nlen == 0 { return true /* 0 */ }
    // jump table
    let jmp: HashMap<&T, usize> = needle.iter().zip((1..nlen).rev()).collect();
    let last = nlen - 1;
    let mut j = 0;
    while j <= hlen - nlen {
        // search backward
        let mut i = last;
        while haystack[j + i] == needle[i] {
            if i == 0 {/* found */ return true /* j */}
            i -= 1
        }
        // jump according to the last one of haystack
        j += *jmp.get(&haystack[j + last]).unwrap_or(&nlen);
    }
    /* not found */ false /* -1 */
}

pub fn sublist<T: Eq+Hash>(la: &[T], lb: &[T]) -> Comparison {
    match std::cmp::Ord::cmp(&la.len(), &lb.len()) {
        Ordering::Equal =>
            if la == lb { Equal } else { Unequal },
        Ordering::Greater =>
            if horspool(la , lb) { Superlist } else { Unequal },
        Ordering::Less =>
            if horspool(lb , la) { Sublist } else { Unequal },
    }
}
