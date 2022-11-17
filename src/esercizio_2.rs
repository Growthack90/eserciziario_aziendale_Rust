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
        return Err(Error::new(io::ErrorKind::Other, "Voglio una stringa non vuota"));
    }
    Ok(s.chars().rev().collect())
  }