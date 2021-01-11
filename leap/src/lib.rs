pub fn is_leap_year(year: u64) -> bool {
    let mut res = false;
    if (year % 4) == 0 && (year % 100) != 0 || (year % 400) == 0 {
        res = true;
    }
    res
}
