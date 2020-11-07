/* 1 + 2 + 3 + ... + n = n(n + 1) / 2
   (n(n + 1) / 2)^2 = n^2(n+1)^2 / 4
*/
pub fn square_of_sum(n: u32) -> u32 {
    n * n * (n * n + 2 * n + 1) / 4
}

// 1^2 + 2^2 + 3^2 + ... + n^2 = n(n + 1)(2n + 1) / 6
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}