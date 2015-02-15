use std::cmp::Ordering;

pub fn compare(input: usize, test: usize) -> Ordering {
    if input < test {
        Ordering::Less
    }
    else if input > test {
        Ordering::Greater
    }
    else {
        Ordering::Equal
    }
}
