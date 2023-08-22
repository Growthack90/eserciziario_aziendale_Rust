pub fn prova_metodi() {
    let word = "goodbye";

    let count_chars = word.chars().count();
    println!("{:?}", count_chars);

    let chars_word = word.chars();
    println!("{:?}", chars_word);
}