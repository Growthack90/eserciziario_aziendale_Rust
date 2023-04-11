/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 7 */
// uguale a ESERCIZIO 6 ma senza metodo .rev()

// controllo su stringa lunga:
//          9  -> 1 char
//          10 -> 2 char
// esempio x trasformazione tipi:
//          usize -> i32 -> char
/**************************************************************************************************/
/**************************************************************************************************/

use std::{io, io::Error};

pub fn funzione_esercizio7() {
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
        return Err(Error::new(
            io::ErrorKind::Other,
            "Voglio una stringa non vuota",
        ));
    }

    let mut vector: Vec<char> = Vec::new();

    let vettore_stringa: Vec<char> = s.chars().collect();

    let mut nuovo_vettore: Vec<char> = Vec::new();

    for character in &vettore_stringa {
        // println!("character: {}", character);
        if character != &'\n' {
            vector.push(*character);
        }
    }

    // println!("Vector: {:?}", vector);   // Vector: ['c', 'i', 'a', 'o']

    let mut n = vector.len();

    while n!= 0 {   // n è usize ciòè senza segno perciò nn può essere negativo
        println!("{}", n);
        nuovo_vettore.push(vector[n - 1]);  
        println!("nuovo vettore: {:?}", nuovo_vettore);     // nuovo vettore: ['o', 'a', 'i', 'c'] 
        n -= 1;
    }

    Ok(nuovo_vettore)
}
