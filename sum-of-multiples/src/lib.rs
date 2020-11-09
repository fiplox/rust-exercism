pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut res: Vec<u32> = Vec::new();

    for i in 0..limit {
        let is_multiple = factors.iter().any(|&x| (x != 0) && (i % x == 0));
        match is_multiple {
            true => res.push(i),
            false => continue,
        }
    }
    res.into_iter().sum()
}
