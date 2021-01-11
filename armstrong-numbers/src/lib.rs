pub fn is_armstrong_number(num: u32) -> bool {
    let ns = num.to_string();

    ns.chars().fold(0, |a, x| {
        let x = match x.to_digit(10) {
            Some(n) => n,
            None => 0,
        };
        a + x.pow(ns.len() as u32)
    }) == num
}
