#![feature(slice_patterns)]
use std::path::Path;
use std::fs::File;
use std::io::{BufRead,BufReader,Write,Error};
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Status { win: isize, loss: isize, draw: isize }

pub fn tally(input: &Path, output: &Path) -> Result<usize, Error> {
    let mut m: HashMap<String, Status> = HashMap::new();
    let fin = BufReader::new(try!(File::open(input)));
    let init = Status{win: 0, loss: 0, draw: 0};
    let mut good = 0;
    for _line in fin.lines() {
        let line = try!(_line);
        let ws: Vec<&str> = line.split(';').collect();
        if let [team1, team2, result] = *ws.as_slice() {
            good += 1;
            let (team1, team2) = (team1.to_string(), team2.to_string());
            match result.as_ref() {
                "win" => {
                    m.entry(team1).or_insert(init).win += 1;
                    m.entry(team2).or_insert(init).loss += 1;
                }
                "loss" => {
                    m.entry(team1).or_insert(init).loss += 1;
                    m.entry(team2).or_insert(init).win += 1;
                }
                "draw" => {
                    m.entry(team1).or_insert(init).draw += 1;
                    m.entry(team2).or_insert(init).draw += 1;
                }
                _ => { good -= 1; }
            }
        }
    }
    let mut ordered: Vec<_> = m.into_iter().collect();
    ordered.sort_by_key(
        |&(ref nm, st)| (-(3*st.win + st.draw), -st.win, nm.to_string()));

    let mut fout = try!(File::create(output));
    try!(fout.write_fmt(format_args!("{:<30} | MP |  W |  D |  L |  P\n", "Team")));
    for &(ref nm, st) in &ordered {
        try!(fout.write_fmt(
            format_args!("{:<30} |  {} |  {} |  {} |  {} |  {}\n",
                         nm, st.win + st.loss + st.draw,
                         st.win, st.draw, st.loss, 3*st.win + st.draw)));
    }
    Ok(good)
}
