/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 3_1 */
// alternativa ESERCIZIO 3, dove ordina un array di elementi dati in input dall'utente
/**************************************************************************************************/
/**************************************************************************************************/

use std::io;

pub fn funzione_esercizio3_1() {
    let mut array = Vec::<i32>::new();

    let mut input = String::new();
    println!("Type the numbers separating them with a space:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            for num_str in input.trim().split_whitespace() {     
                match num_str.parse::<i32>() {
                    Ok(num) => array.push(num),
                    Err(_) => {
                        println!("Errore nella conversione del numero: {}", num_str);
                        return;
                    }
                }
                println!("DEBUG_1 {}", num_str);
            }
        }
        Err(_) => {
            println!("Errore nell'input");
            return;
        }
    }

    println!("*********************************************************************");

    println!("Array non ordinato: {:?}", array);

    println!("*********************************************************************");

    let array_ordinato = order_array(&array);
    match array_ordinato {
        Ok(new_array) => println!("Array ordinato: {:?}", new_array),
        Err(e) => println!("Errore {:?}", e),
    }
}

fn order_array(array: &[i32]) -> Result<Vec<i32>, &str> {
    let mut sorted_array = array.to_vec();
    sorted_array.sort();
    Ok(sorted_array)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_array_empty() {
        let input = vec![];
        assert_eq!(order_array(&input), Ok(vec![]));
    }

    #[test]
    fn test_order_array_unsorted() {
        let input = vec![4, 2, 7, 1, 9];
        assert_eq!(order_array(&input), Ok(vec![1, 2, 4, 7, 9]));
    }

    #[test]
    fn test_order_array_sorted() {
        let input = vec![1, 2, 4, 7, 9];
        assert_eq!(order_array(&input), Ok(vec![1, 2, 4, 7, 9]));
    }

    #[test]
    fn test_order_array_negative() {
        let input = vec![-9, -4, -2, -7, -1];
        assert_eq!(order_array(&input), Ok(vec![-9, -7, -4, -2, -1]));
    }

    #[test]
    fn test_order_array() {
        let input = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let expected = vec![1, 1, 2, 3, 4, 5, 6, 9];
        assert_eq!(order_array(&input), Ok(expected));
    }

    #[test]
    fn test_empty_value_skipped() {
        let input = "3 1 4  9 2  ";
        let expected = vec![3, 1, 4, 9, 2];
        let mut array = Vec::<i32>::new();

        for num_str in input.trim().split_whitespace() {
            if num_str.is_empty() {
                continue;
            }

            match num_str.parse::<i32>() {
                Ok(num) => array.push(num),
                Err(_) => {
                    panic!("Errore nella conversione del numero: {}", num_str);
                }
            }
        }

        assert_eq!(array, expected);
    }

}
