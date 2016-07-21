extern crate rand;
extern crate rayon;
use rand::{Rng, StdRng, SeedableRng};

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut i = 1;
    for j in 1..v.len() {
        if v[j] <= v[0] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(0, i-1);
    i-1
}

fn seq_quicksort<T: PartialOrd>(mut v: &mut [T]) {
    if v.len() <= 1 { return }
    let mid = partition(&mut v);
    let (lo, hi) = v.split_at_mut(mid);
    seq_quicksort(lo); seq_quicksort(&mut hi[1..]);
}

fn par_quicksort<T: PartialOrd+Send>(mut v: &mut [T]) {
    if v.len() <= 1 { return }
    if v.len() <= 1024 { seq_quicksort(&mut v); return }
    let mid = partition(&mut v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| par_quicksort(lo), || par_quicksort(&mut hi[1..]));
}

fn main(){
    let mut rng = StdRng::from_seed(&[0]);
    let mut v: Vec<_> = (0..1000000).flat_map(|x| vec![x; 10]).collect();
    let sort = par_quicksort; // 1.20 real, 2.23 user
    // let sort = seq_quicksort; // 1.82 real, 1.79 user
    rng.shuffle(&mut v); sort(&mut v);
    assert!(v.iter().zip(v.iter().skip(1)).all(|(a, b)| a <= b));
}
