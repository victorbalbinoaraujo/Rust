/*
Converta strings para Pig Latin, onde a primeira consoante de cada palavra é movida
para o final da palavra adicionado um "ay" , então “first” se torna “irst-fay”.
Palavras que começam com uma vogal recebem “hay” adicionado ao final (“apple” torna-se “apple-hay”).
Lembre-se sobre a codificação UTF-8!
 */

use std::io;

fn main() {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut s1: String = String::new();
    io::stdin().read_line(&mut s1).expect("Failed to read line");

    s1 = s1.trim().to_string();

    if vowels.iter().any(|&v| s1.starts_with(v)) {
        println!("{s1}-hay");
    } else {
        let first: char = s1.chars().next().unwrap();
        s1.remove(0);
        println!("{s1}-{first}ay");
    }
}
