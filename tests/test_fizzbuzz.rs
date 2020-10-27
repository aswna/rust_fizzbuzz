extern crate rust_fizzbuzz;

#[cfg(test)]
mod tests {
    #[test]
    fn test_fizzbuzz_outputs_for_input_1() {
        let output = rust_fizzbuzz::fizzbuzz(1);
        assert_eq!("1", output);
    }

    #[test]
    fn test_fizzbuzz_outputs_for_input_2() {
        let output = rust_fizzbuzz::fizzbuzz(2);
        assert_eq!("1, 2", output);
    }

    #[test]
    fn test_fizzbuzz_outputs_for_input_3() {
        let output = rust_fizzbuzz::fizzbuzz(3);
        assert_eq!("1, 2, fizz", output);
    }
}
