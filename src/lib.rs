pub fn fizzbuzz(max_number: i32) -> String {
    println!("Printing fizzbuzz up to: {}", max_number);
    let mut results = Vec::new();
    for i in 1..max_number+1 {
        results.push(i.to_string());
    }
    return results.join(", ");
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn test_fizzbuzz_with_1() {
    //     assert_eq!(, 3);
    // }
}
