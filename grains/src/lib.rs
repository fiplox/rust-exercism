pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    } else {
        1 << (s - 1)
    }
}

pub fn total() -> u64 {
    let a: i128 = (1 << 64) - 1;
    a as u64
}
