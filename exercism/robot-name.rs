extern crate rand;

pub struct Robot { name: String }

impl Robot {
    fn gen_name() -> String {
        let mut rng = rand::thread_rng();
        let mut sample = |r: std::ops::Range<u8>, n|
          rand::sample(&mut rng, r.map(|i| i as char), n);
        sample(65..91, 2).into_iter().chain(
        sample(48..58, 3).into_iter()).collect()
    }
    pub fn new() -> Self {
        Robot{name: Self::gen_name()}
    }
    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
    pub fn reset_name(&mut self) {
        self.name = Self::gen_name()
    }
}
