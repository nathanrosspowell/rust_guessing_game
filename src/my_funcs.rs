use std::cmp::Ordering;

pub fn compare(input: u32, test: u32) -> Ordering {
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
