// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        let even = 10;
        let even_resutl = is_even(even);
        let even2 = 12;
        let even2_result = is_even(even2);
        assert!(even_resutl);
        assert!(even2_result);
    }
}
