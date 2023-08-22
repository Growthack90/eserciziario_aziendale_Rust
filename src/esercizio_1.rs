/**************************************************************************/
/**************************************************************************/
/* ESERCIZIO 1 */
// il main si prende in input una stringa
// chiama reverse_string() che ritorna una stringa
// stampa il risultato
/**************************************************************************/
/**************************************************************************/

/*********************************/
/*********************************/
/* SOLUZIONE_1 + TEST CON ASSERT_EQ */
/*********************************/
/*********************************/
// /********************************************************************************/
use std::io;

pub fn funzione_esercizio1() {
  
    let mut input = String::new();

        println!("\nCosa vuoi invertire?\n");

        let read_line = io::stdin().read_line(&mut input);

        match read_line {
        Ok(_) => {
            let reverse = reverse_string(input);
            println!("{}", reverse);
        }
        Err(error) => println!("error: {error}"),
        }

  }

  fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
  }


/* TEST WITH ASSERT_EQ */
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn test_reverse_string_ok() {
          let input = String::from("ciao");
          let expected_output = String::from("oaic");
          assert_eq!(reverse_string(input), expected_output);
      }

      #[test]
      fn test_reverse_string_uppercase_ok() {
          let input = String::from("CIAO");
          let expected_output = String::from("OAIC");
          assert_eq!(reverse_string(input), expected_output);
      }
      
      #[test]
      fn test_reverse_string_uppercase_ko() {
          let input = String::from("CIAO");
          let expected_output = String::from("oaic");
          assert_ne!(reverse_string(input), expected_output);
      }
  }
// /********************************************************************************/

/*********************************/
/* SOLUZIONI ALTERNATIVE */
/*********************************/
// fn main() {
//     let str1 = String::from("Hello, world!");
//     let str2 = reverse_string(&str1);
//     println!("The reverse of string \"{}\" is \"{}\".", str1, str2);
//   }

//   fn reverse_string(s: &str) -> String {
//     s.chars().rev().collect()
//   }

/*********************************/

// fn main() {
//   let str1 = String::from("Hello, world!");
//   let str2 = reverse(&str1);
//   println!("The reverse of \"{}\" is \"{}\".", str1, str2);
// }

// pub fn reverse(input: &str) -> String {
//     let mut result = String::new();
//     for c in input.chars().rev() {
//         result.push(c)
//     }
//     result
// }

/*********************************/

// use std::io;

// fn main() {

//     let mut input = String::new();

//     println!("Cosa vuoi invertire?");

//     println!("{}", input);

//     inverti_stringa();

// }

// pub fn inverti_stringa() {

//     let mut input = String::new();

//     match io::stdin().read_line(&mut input) {
//         Ok(_) => {
//             println!("{}", input.chars().rev().collect::<String>());
//         }
//         Err(error) => println!("error: {error}"),
//     }

// }

/*********************************/