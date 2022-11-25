/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 6 */
// prendere una stringa in input
// restituire vettore dei caratteri invertiti di ordine
// ciao --> [o, a, i, c]

// errori: no vuota
/**************************************************************************************************/
/**************************************************************************************************/

use std::{io, io::Error};


pub fn funzione_esercizio6() {

    let mut input = String::new();

        println!("\nScrivi stringa che vuoi invertire?\n");

    let read_line = io::stdin().read_line(&mut input);

        match read_line {
        Ok(_) => {
            let vector_reverse = reverse_vector(input);
            match vector_reverse {
                Ok(reverse) => println!("Stampami stringa invertita dentro il vettore: {:?}", reverse),
                Err(e) => eprintln!("{}", e.to_string()),
            }
        }
        Err(error) => println!("error: {error}"),
        }

}


  fn reverse_vector(s: String) -> Result<Vec<char>, Error> {
    
    if s == "\n" {
        return Err(Error::new(io::ErrorKind::Other, "Voglio una stringa non vuota"));
    }

    let mut vector: Vec<char> = Vec::new();

    let vettore_stringa: Vec<char> = s.chars().rev().collect();

    for character in vettore_stringa {
        // println!("{}", character);
        if character != '\n' {
            vector.push(character);
        }
    }
    println!("Vector: {:?}", vector);


    Ok(vector)

  }