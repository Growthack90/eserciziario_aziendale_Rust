/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 7 */
// uguale a ESERCIZIO 6 ma senza metodo .rev()
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

    let mut n = vettore_stringa.len() - 1;
    println!("Prova debug: {:?}", n);
    
    // let i = 1;

    
    for character in &vettore_stringa {
        println!("character: {}", character);
        
        if character != &'\n' {
            vector.push(*character);
        }

        for i in 0..n {
            println!("Prova debug: {}", i);
        }

        // while i <= n {
        //     println!("{}", n);
        //     n -= 1;
        //     nuovo_vettore.push(*character);
        // }
        
        println!("Vector: {:?}", vector);
        println!("Vector nuevo: {:?}", nuovo_vettore);
    }
    
    Ok(vector)

}


// println!("Stampa di Debug nella funzione: {:?}", vector);



// for c in 0..n {
//     println!("Prova debug: {}", c);
// }

// let i = 1;

// while i <= n {
//     println!("{}", n);
//     n -= 1;
// }

// for el_word in s.chars().enumerate() {
//     println!("singolo carattere parola: {}", el_word.1);
// }


// FOR DI JAVA
// for (int i = input.length() - 1; i >= 0; i--) {
//     System.out.println("prova ciclo - " + i);
//     output.append(input.charAt(i));
// }