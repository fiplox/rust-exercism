use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new: BTreeMap<char, i32> = BTreeMap::new();
    for (i, v) in h {
        for c in &*v {
            new.insert(c.to_ascii_lowercase(), *i);
        }
    }

    new
}
