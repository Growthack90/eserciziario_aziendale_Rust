/**************************************************************************************************/
/**************************************************************************************************/
/* ESERCIZIO 3 */
// chiamo altro metodo da main che riceve in input array di interi
// deve restituirli ordinati in ordine crescente
// restituire un result che contiene o un array o un errore (in caso di vuoto o un solo numero)
/**************************************************************************************************/
/**************************************************************************************************/

use std::fmt::Error;
use std::io;
use std::ops::Index;
use std::result::Result;

pub fn funzione_esercizio3() {
    let mut array = Vec::<i32>::new();

    array.push(13);
    array.push(5);
    array.push(10);
    array.push(1);
    array.push(2);

    // println!("Stampami array di interi");
    println!("Array non ordinato: {:?}", array);
    println!("*********************************************************************");

    let array_ordinato = order_array(array);
    match array_ordinato {
        Ok(new_array) => println!("Array ordinato: {:?}", new_array),
        Err(e) => println!("Errore {:?}", e),
    }
}

fn order_array(mut arr: Vec<i32>) -> Result<Vec<i32>, Error> {
    // println!("restituire un result che contiene un array di interi (in ordine crescente) o un errore");

    if arr.is_empty() || arr.len() == 1 {
        return Err(Error);
    }

    let mut ordering_array: Vec<i32> = Vec::<i32>::new();

    let mut minimo_temporaneo = arr[0];

    let mut posizione_min_temp = 0;

    for _ in arr.clone() {
        for (index, element) in arr.iter().enumerate() {
            if *element < minimo_temporaneo {
                minimo_temporaneo = *element;
                posizione_min_temp = index;
            }
        }

        arr.remove(posizione_min_temp);

        ordering_array.push(minimo_temporaneo);

        if arr.len() >= 1 {
            minimo_temporaneo = arr[0];
        }

        posizione_min_temp = 0;
    }

    Ok(ordering_array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_array_empty() {
        let input: Vec<i32> = Vec::new();
        let result = order_array(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_order_array_single_element() {
        let input = vec![42];
        let result = order_array(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_order_array_multiple_elements() {
        let input = vec![13, 5, 10, 1, 2];
        let expected_output = vec![1, 2, 5, 10, 13];
        let result = order_array(input.clone());
        assert!(result.is_ok());
        let ordered_array = result.unwrap();
        assert_eq!(ordered_array, expected_output);
    }

    #[test]
    fn test_order_array_large_array() {
        let input = vec![5, 2, 9, 1, 5, 7, 3, 8, 6, 4];
        let expected_output = vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9];
        let result = order_array(input.clone());
        assert!(result.is_ok());
        let ordered_array = result.unwrap();
        assert_eq!(ordered_array, expected_output);
    }
}
