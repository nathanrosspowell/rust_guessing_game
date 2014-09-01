pub mod my_funcs {
    pub fn compaire(input: uint, test: uint) -> Ordering {
        if input < test {
            Less
        }
        else if input > test {
            Greater
        }
        else {
            Equal
        }
    }
}
