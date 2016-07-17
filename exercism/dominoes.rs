use std::collections::HashSet;

pub type Domino = (usize, usize);

fn dfs (input: &[Domino], n: &usize, i: usize,
        ans: &mut Vec<Domino>, used: &mut HashSet<usize>) -> bool {
    if i == *n { return ans[0].0 == ans[n-1].1 }
    for (j, &d) in input.iter().enumerate() {
        if used.contains(&j) {continue}
        let next = {
            if ans[i-1].1 == d.0 {d}
            else if ans[i-1].1 == d.1 {(d.1, d.0)}
            else {continue}
        };
        used.insert(j); ans.push(next);
        if dfs(input, n, i+1, ans, used) {return true}
        used.remove(&j); ans.pop();
    }
    false
}

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    let n = input.len();
    if n == 0 { return Some(Vec::new()) }
    let mut ans: Vec<Domino> = vec![input[0]];
    let mut used: HashSet<usize> = vec![0].into_iter().collect();
    if dfs(input, &n, 1, &mut ans, &mut used)
      {Some(ans)} else {None}
}
