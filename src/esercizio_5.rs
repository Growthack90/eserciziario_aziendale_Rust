/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 5 */
// prendere una parola in input
// restituire stessa parola dove ogni lettera la trasforma nella posizione numerica alfabetica
// abc --> 123

// errore: parola vuota , o numeri
/**************************************************************************************************/
/**************************************************************************************************/


use std::{io, io::Error};

pub fn funzione_esercizio5() {

    let mut input = String::new();

        println!("\nDimmi una parola?\n");

    let read_line = io::stdin().read_line(&mut input);

        match read_line {
        Ok(_) => {
            let transformation_input = alph_index_string(input);
            match transformation_input {
                Ok(index_alph) => println!("alphabetical index corresponding input character: {}", index_alph),
                Err(e) => eprintln!("{}", e.to_string()),
            }

        }
        Err(error) => println!("error: {error}"),
        }

}


  fn alph_index_string(s: String) -> Result<String, Error> {
      
      // NO EMPTY STRING
      if s == "\n" {
          return Err(Error::new(io::ErrorKind::Other, "Voglio una stringa non vuota"));
        }
        
        // NO NUMERIC VALUE
        for el_num in s.chars() {
            if el_num.is_numeric() {
                return Err(Error::new(io::ErrorKind::Other, "Non voglio valori numerici"));
            }
            // println!("{}", el_num.is_numeric());
        }
        

    let alphabeth = "abcdefghijklmnopqrstuvwxz";

    let mut string: String = String::new();
     
    let mut new_string = Vec::<usize>::new();

        for char_string in s.chars().enumerate() {
            if char_string.1 != '\n' {
                // println!("single char INPUT: {}", char_string.1);
                    for char_alph in alphabeth.chars().enumerate() {
                        // println!("single char alphabet: {:?}", char_alph);
                        if char_string.1 == char_alph.1 {
                            string.push(char_string.1);
                            println!("char string: {} --> index alph: {}", char_string.1, char_alph.0);
                            
                            new_string.push(char_string.0);
                        }
                   }
            }
        }

        println!("STRING: {}", string);

        println!("NEW STRING: {:?}", new_string);

    Ok(string)

  }
