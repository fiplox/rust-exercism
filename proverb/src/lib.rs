pub fn build_proverb(list: &[&str]) -> String {
    let s = list.len();
    let mut res = String::new();

    for i in 0..s {
        if i < s - 1 {
            res = format!(
                "{}For want of a {} the {} was lost.\n",
                res,
                list[i],
                list[i + 1]
            );
        } else {
            res = format!("{}And all for the want of a {}.", res, list[0]);
        }
    }
    res
}
