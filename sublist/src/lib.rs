use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Equal if _first_list == _second_list => Comparison::Equal,
        Ordering::Greater
            if _second_list.is_empty()
                || _first_list
                    .windows(_second_list.len())
                    .any(|x| x == _second_list) =>
        {
            Comparison::Superlist
        }
        Ordering::Less
            if _first_list.is_empty()
                || _second_list
                    .windows(_first_list.len())
                    .any(|x| x == _first_list) =>
        {
            Comparison::Sublist
        }
        _ => Comparison::Unequal,
    }
}
