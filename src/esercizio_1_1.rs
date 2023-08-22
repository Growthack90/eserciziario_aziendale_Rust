/*********************************/
/*********************************/
/* ALTERNATIVA ESERCIZIO 1 + TEST CON MATCH */
/*********************************/
/*********************************/
/********************************************************************************/
use std::io;

pub fn funzione_esercizio1_1() {
    println!("Please enter a string to reverse:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    match reverse_string(input) {
        Ok(output) => println!("Reversed string: {}", output),
        Err(err) => eprintln!("Failed to reverse string: {}", err),
    }
}

fn reverse_string(input: String) -> Result<String, &'static str> {
    if input.is_empty() {
        return Err("Input string is empty");
    }

    let output = input.chars().rev().collect();
    Ok(output)
}

// /* TEST WITH MATCH */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_lower_string_ok() {
        let input = String::from("hello");
        let expected_output = String::from("olleh");
        match reverse_string(input) {
            Ok(output) => assert_eq!(output, expected_output),
            Err(err) => panic!("Failed to reverse string: {}", err),
        }
    }
}

#[test]
fn test_reverse_upper_string_ok() {
    let input = String::from("HELLO");
    let expected_output = String::from("OLLEH");
    match reverse_string(input) {
        Ok(output) => assert_eq!(output, expected_output),
        Err(err) => panic!("Failed to reverse string: {}", err),
    }
}
#[test]
fn test_reverse_string_ko() {
    let input = String::from("HELLO");
    let expected_output = String::from("olleh");
    match reverse_string(input) {
        Ok(output) => assert_ne!(output, expected_output),
        Err(err) => panic!("Failed to reverse string: {}", err),
    }
}
/********************************************************************************/
