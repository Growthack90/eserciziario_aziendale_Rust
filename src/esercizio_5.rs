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
                Ok(index_alph) => println!("Indice alfabetico- della parola: {}", index_alph),
                Err(e) => eprintln!("{}", e.to_string()),
            }

        }
        Err(error) => println!("error: {error}"),
        }

}


  fn alph_index_string(s: String) -> Result<String, Error> {


    let alphabeth = "abcdefghijklmnopqrstuvwxz";

    // NO STRINGA VUOTA
    if s == "\n" {
        return Err(Error::new(io::ErrorKind::Other, "Voglio una stringa non vuota"));
    }

    // NO VALORI NUMERICI
    for el_num in s.chars() {
        if el_num.is_numeric() {
            return Err(Error::new(io::ErrorKind::Other, "Non voglio valori numerici"));
        }
        // println!("{}", el_num.is_numeric());
    }
    

    let mut index_word: String = String::new();

    for el_word in s.chars().enumerate() {
        println!("singolo carattere parola: {}", el_word.1);

        for el_alph in alphabeth.chars().enumerate() {
            println!("carattere parola trovato nell'alfalfabeto: {:?}", el_alph);
            if el_alph.1 == el_word.1 {
                println!("Inserisci indice alfabetico parola {:?}", index_word);
                index_word.push(el_word.1);
            }
        }
    }
    

    Ok(index_word)

  }