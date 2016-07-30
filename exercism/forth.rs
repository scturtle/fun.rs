extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default, Debug)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {stack: Vec::new(), words: HashMap::new()}
    }

    pub fn format_stack(&self) -> String {
        self.stack.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let re = Regex::new("[ \u{0000}\u{0001} \\r\\t\\n]+").unwrap();
        let mut ws: Vec<String> = re.split(input.to_lowercase().as_str())
                                    .map(|s| s.into()).collect();
        ws.reverse();
        while let Some(w) = ws.pop() {
            if let Ok(n) = w.parse() {
                self.stack.push(n);
                continue;
            }
            if let Some(l) = self.words.get(&w) {
                ws.extend(l.iter().rev().cloned());
                continue;
            }
            // eval
            match w.as_str() {
                "+" | "-" | "*" | "/" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow)
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if w.as_str() == "/" && a == 0 {
                        return Err(Error::DivisionByZero)
                    }
                    let op = match w.as_str() {
                        "+" => Add::add, "-" => Sub::sub,
                        "*" => Mul::mul, "/" => Div::div,
                        _ => unimplemented!(),
                    };
                    self.stack.push(op(b, a));
                }
                "dup" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow)
                    }
                    let a = self.stack[self.stack.len()-1];
                    self.stack.push(a);
                }
                "drop" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow)
                    }
                    self.stack.pop();
                }
                "swap" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow)
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a);
                    self.stack.push(b);
                }
                "over" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow)
                    }
                    let a = self.stack[self.stack.len()-2];
                    self.stack.push(a);
                }
                ":" => {
                    if ws.is_empty() {
                        return Err(Error::InvalidWord)
                    }
                    let name = ws.pop().unwrap();
                    if let Ok(_) = name.parse::<Value>() {
                        return Err(Error::InvalidWord)
                    }
                    let mut l = Vec::new();
                    let mut ended = false;
                    while let Some(t) = ws.pop() {
                        if t == ";" { ended = true; break }
                        l.push(t);
                    }
                    if !ended {
                        return Err(Error::InvalidWord)
                    }
                    self.words.insert(name, l);
                }
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }
}
