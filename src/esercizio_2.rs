/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 2 */
// reverse_string se riceve una stringa vuota "" restituisce result con errore
// reverse_string se riceve parola restituisce result con stringa
// usare result e match
/**************************************************************************************************/
/**************************************************************************************************/

use std::{io, io::Error};

pub fn funzione_esercizio2() {
    let mut input = String::new();

    println!("\nCosa vuoi invertire?\n");

    let read_line = io::stdin().read_line(&mut input);

    match read_line {
        Ok(_) => {
            let res_reverse = reverse_string(input);
            match res_reverse {
                Ok(reverse) => println!("{}", reverse),
                Err(e) => eprintln!("{}", e.to_string()),
            }
        }
        Err(error) => println!("error: {error}"),
    }
}

fn reverse_string(s: String) -> Result<String, Error> {
    if s == "\n" {
        return Err(Error::new(
            io::ErrorKind::Other,
            "Voglio una stringa non vuota",
        ));
    }
    Ok(s.chars().rev().collect())
}

/* TEST WITH ASSERT_EQ */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let input = String::from("hello");
        let result = reverse_string(input).unwrap();
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_reverse_string_2() {
        let input = String::from("hello");
        let result = reverse_string(input);

        match result {
            Ok(reversed) => {
                assert_eq!(reversed, "olleh");
            }
            Err(_) => {
                // Fail the test explicitly since this case is unexpected
                panic!("Expected Ok result, but got Err");
            }
        }
    }

    #[test]
    fn test_reverse_newline_string() {
        let input = String::from("\n");
        let result = reverse_string(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reverse_unicode_string() {
        let input = String::from("🌍🌞🌙");
        let result = reverse_string(input).unwrap();
        assert_eq!(result, "🌙🌞🌍");
    }

    #[test]
    fn test_reverse_unicode_string_2() {
        let input = String::from("🌍🌞🌙");
        let result = reverse_string(input);

        match result {
            Ok(reversed) => {
                assert_eq!(reversed, "🌙🌞🌍");
            }
            Err(_) => {
                // Fail the test explicitly since this case is unexpected
                panic!("Expected Ok result, but got Err");
            }
        }
    }

    #[test]
    fn test_reverse_mixed_case_string() {
        let input = String::from("RaCeCar");
        let result = reverse_string(input).unwrap();
        assert_eq!(result, "raCeCaR");
    }

    #[test]
    fn test_reverse_long_string() {
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
        let result = reverse_string(input).unwrap();
        assert_eq!(
            result,
            ".tile gnicsipida rutcesnoc ,tema tis rolod muspi meroL"
        );
    }
}
