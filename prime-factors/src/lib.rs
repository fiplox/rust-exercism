pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut n = n;
    for candidate in 2..=n {
        if n == 1 {
            break;
        }
        while n % candidate == 0 {
            res.push(candidate);
            n /= candidate;
        }
    }
    res
}
