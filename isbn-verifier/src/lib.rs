/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let v: Vec<char> = isbn
        .chars()
        .filter(|&c| c.is_digit(10) || c == 'X')
        .collect();

    if v.len() != 10 || v[0..9].iter().any(|c| !c.is_digit(10)) {
        return false;
    }

    let mut res = 0;
    let mut j = 10;
    for c in v {
        match c {
            'X' => {
                if j != 1 {
                    return false;
                }
                res += 10;
                break;
            }
            _ => {
                res += c.to_digit(10).unwrap() * j; //safe unwrap as we know it is digit.
                j -= 1;
            }
        };
    }
    res % 11 == 0
}
