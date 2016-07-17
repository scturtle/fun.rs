
pub struct WordProblem {
    problem: String
}

impl WordProblem {
    pub fn new(p: &str) -> Self {
        WordProblem { problem : p.to_string() }
    }
    pub fn answer(&self) -> Result<i32, String> {
        let v: Vec<&str> = self.problem[..self.problem.len()-1].split(' ').collect();
        if v.len() < 3 || v[0] != "What" || v[1] != "is" {return Err("wrong starting".into())}
        let mut ans = {
            let t = v[2].parse();
            if t.is_err() {return Err(v[2].to_string() + " is not number")};
            t.unwrap()
        };
        let mut i = 3;
        while i < v.len() {
            match v[i] {
                "plus" | "minus" => {
                    let t = v[i+1].parse();
                    if t.is_err() {return Err(v[i+1].to_string() + " is not number")};
                    if v[i] == "plus" {ans += t.unwrap()} else {ans -= t.unwrap()}
                    i += 2;
                },
                "multiplied" | "divided" if v[i+1] == "by" => {
                    let t = v[i+2].parse();
                    if t.is_err() {return Err(v[i+2].to_string() + " is not number")};
                    if v[i] == "multiplied" {ans *= t.unwrap()} else {ans /= t.unwrap()}
                    i += 3;
                }
                _ => return Err(v[i].to_string() + " is not good operator")
            }
        }
        Ok(ans)
    }
}
