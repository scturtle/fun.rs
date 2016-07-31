use std::mem;
use std::default::Default;

pub struct CircularBuffer<T> {
    buf: Vec<T>,
    cnt: usize,
    fr: usize,
    tl: usize,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(cap: usize) -> Self {
        CircularBuffer {
            // or vec![T::default(); cap] wth Clone trait
            buf: (0..cap).map(|_| T::default()).collect(),
            cnt: 0,
            fr: 0,
            tl: 0,
        }
    }
    pub fn read(&mut self) -> Result<T, Error> {
        println!("read  fr:{} tl:{} cnt:{}", self.fr, self.tl, self.cnt);
        if self.cnt == 0 {
            Err(Error::EmptyBuffer)
        } else {
            // or self.buf[self.fr].to_owned() with clone trait
            let t = mem::replace(&mut self.buf[self.fr], T::default());
            self.fr = (self.fr + 1) % self.buf.len();
            self.cnt -= 1;
            Ok(t)
        }
    }
    pub fn write(&mut self, a: T) -> Result<(), Error> {
        println!("write fr:{} tl:{} cnt:{}", self.fr, self.tl, self.cnt);
        if self.cnt == self.buf.len() {
            Err(Error::FullBuffer)
        } else {
            self.buf[self.tl] = a;
            self.tl = (self.tl + 1) % self.buf.len();
            self.cnt += 1;
            Ok(())
        }
    }
    pub fn clear(&mut self) {
        self.tl = self.fr;
        self.cnt = 0;
    }
    pub fn overwrite(&mut self, a: T) {
        if self.cnt == self.buf.len() {
            self.buf[self.fr] = a;
            self.fr = (self.fr + 1) % self.buf.len();
            self.tl = (self.tl + 1) % self.buf.len();
        } else {
            let _ = self.write(a);
        }
    }
}
