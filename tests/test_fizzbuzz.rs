extern crate rust_fizzbuzz;

#[cfg(test)]
mod tests {
    #[test]
    fn test_fizzbuzz_0() {
        println!("Testing!");
        rust_fizzbuzz::fizzbuzz();
        assert_eq!(1, 2);
    }
}
