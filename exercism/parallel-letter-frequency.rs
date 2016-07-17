use std::thread;
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::mpsc;

pub fn frequency(input: &[&'static str], workers: usize) -> HashMap<char, usize> {
    // have to convert to string?
    // let input: Arc<Vec<_>> = Arc::new(input.iter().map(|s|s.to_string()).collect());
    let input: Arc<Vec<_>> = Arc::new(Vec::from(input));
    let (tx, rx) = mpsc::channel::<(char, usize)>();
    for worker_idx in 0..workers {
        let input = input.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let mut cnt: HashMap<char, usize> = HashMap::new();
            for (idx, line) in input.iter().enumerate() {
                if idx % workers != worker_idx { continue }
                for ch in line.to_lowercase().chars() {
                    if ch.is_alphabetic() {
                        *cnt.entry(ch).or_insert(0) += 1;
                    }
                }
            }
            for t in cnt.into_iter() { tx.send(t).unwrap(); }
        });
    }
    let mut cnt = HashMap::new();
    drop(tx); // avoid hanging
    for (k, v) in rx.iter() {
        *cnt.entry(k).or_insert(0) += v;
    }
    cnt
}
