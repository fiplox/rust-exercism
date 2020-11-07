pub fn verse(n: u32) -> String {
    //unimplemented!("emit verse {}", n)
    let s0 = format!(
        "{} {} of beer on the wall, {} bottles of beer.\n",
        n, n==, n
    );
    let s1 = format!(
        "Take one down and pass it around, {} {} of beer on the wall.",
        n - 1,
        if n == 1 { "bottle" } else { "bottles" }
    );
    let fin = format!(
        "Go to the store and buy some more, {} bottles of beer on the wall.",
        n
    );
}

pub fn sing(start: u32, end: u32) -> String {
    unimplemented!("sing verses {} to {}, inclusive", start, end)
}
