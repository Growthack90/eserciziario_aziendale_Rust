/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 4 */
// funzione che passando una stringa restituisce stringa con solo vocali (senza consonanti)
/**************************************************************************************************/
/**************************************************************************************************/

use std::io;
use std::io::Error;

pub fn funzione_esercizio4() {

    let mut word_input = String::new();

        println!("\nChe parola vuoi scrivere?\n");
        
        let read_word = io::stdin().read_line(&mut word_input);

        println!("**************************************************");

        match read_word {
        Ok(_) => {
            let vowels_word = function_vowels(word_input);
            match vowels_word {
                Ok(print_vowels) => println!("Stampa solo vocali della parola inserita: {}", print_vowels),
                Err(e) => eprintln!("{}", e.to_string()),
            }
        }
        Err(e) => println!("errore: {e}"),
        }

  }


fn function_vowels(word: String) -> Result<String, Error> {
    if word == "\n" {
        return Err(Error::new(io::ErrorKind::Other, "Voglio una stringa non vuota"));
    }

    // let parola = "ciao a tutti";

    // // DIMENSIONE STRINGA
    // let mut size = parola.char_indices().count();
    // println!("Stampami dimensione parola: {}", size);
    // println!("**************************************************");

    // // SPLITTARE STRINGA
    // let parole_divise: Vec<&str> = parola.split(' ').collect();
    // println!("Frase divisa in parole: {:?}", parole_divise);
    // println!("**************************************************");

    // // STRINGA IN UN VETTORE
    // let mut vettore_parola = Vec::new();
    // vettore_parola.push(parola);
    // println!("Stringa inserita in un vettore: {:?}", vettore_parola);
    // println!("**************************************************");

    // // STRINGA DIVISA IN SINGOLI CARATTERI
    // let parola_caratteri = parola.char_indices();
    // println!("Parola divisa in caratteri: {:?}", parola_caratteri);
    // println!("**************************************************");
    

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut new_word = String::new();

    for character in word.to_lowercase().chars().enumerate() {   // nota: character è un tipo tupla

            if vowels.contains(&character.1) {
                // println!("il carattere è una vocale!");
                new_word.push(character.1);
            }
            
        }

        // println!("Nuova parola senza vocali: {:?}", new_word);

    Ok(new_word)

  }