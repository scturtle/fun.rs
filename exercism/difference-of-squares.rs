#![feature(iter_arith)]

pub fn square_of_sum(n : u32) -> u32{
    (1..n+1).fold(0, |a, b| a + b).pow(2)
}
pub fn sum_of_squares(n : u32) -> u32{
    (1..n+1).map(|i|i.pow(2)).sum()
}
pub fn difference(n : u32) -> u32{
    square_of_sum(n) - sum_of_squares(n)
}
