pub fn fizzbuzz(max_number: i32) -> String {
    println!("Printing fizzbuzz up to: {}", max_number);
    let mut results = Vec::new();
    for i in 1..=max_number {
        let result = number_to_fizzbuzz(i);
        results.push(result);
    }
    return results.join(", ");
}

fn number_to_fizzbuzz(number: i32) -> String {
    if number % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if number % 3 == 0 {
        return "fizz".to_string();
    } else if number % 5 == 0 {
        return "buzz".to_string();
    } else {
        return number.to_string();
    }
}

// fn number_to_fizzbuzz(number: i32) -> String {
//     let mut result = String::new();
//     if number % 3 == 0 {
//         result += "fizz";
//     }
//     if number % 5 == 0 {
//         result += "buzz";
//     }
//     if result == "" {
//         result = number.to_string();
//     }
//     return result;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_fizzbuzz() {
        let test_data = vec![
            (1, "1"),
            (2, "2"),
            (3, "fizz"),
            (4, "4"),
            (5, "buzz"),
            (6, "fizz"),
            (9, "fizz"),
            (10, "buzz"),
            (12, "fizz"),
            (15, "fizzbuzz"),
            (30, "fizzbuzz"),
            (99, "fizz"),
            (100, "buzz"),
        ];
        for (number, expected_output) in test_data.iter() {
            assert_eq!(expected_output.to_string(), number_to_fizzbuzz(*number));
        }
    }
}
