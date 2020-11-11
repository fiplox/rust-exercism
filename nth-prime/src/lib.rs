pub fn nth(n: u32) -> u32 {
    let mut prime = 2;
    let mut i : u32 = 0;
    while i < n {
        prime += 1;
        if is_prime(prime) {
            i += 1;
        }
    }
    prime
}

fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if (n%i) == 0 {
            return false;
        }
    }
    return true;
}
