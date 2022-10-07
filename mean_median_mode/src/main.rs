/*
Dada uma lista de inteiros, use um vetor e retorne a média, a mediana
(quando classificado, o valor na posição do meio) e modo (o valor que
ocorre com mais frequência; um hash map será útil aqui) da lista.
 */

use rand::Rng;
use std::collections::HashMap;

fn mean(vec: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(vec.iter());
    f64::from(sum) / (vec.len() as f64)
}

fn median(vec: &[i32]) -> f64 {
    let midpoint: usize = vec.len() / 2;

    if vec.len() % 2 == 1 {
        vec[midpoint] as f64
    } else {
        mean(&vec[(midpoint - 1)..(midpoint + 1)])
    }
}

fn main() {
    let mut v: Vec<i32> = (1..51)
        .map(|_| rand::thread_rng().gen_range(1..101))
        .collect();

    v.sort();

    let mut freq: HashMap<i32, i32> = HashMap::new();

    for n in &v {
        let count: &mut i32 = freq.entry(*n).or_insert(0);
        *count += 1;
    }

    let max: (&i32, &i32) = freq.iter().max_by_key(|entry| entry.1).unwrap();

    println!("List: {:?}", v);

    println!(
        "Mean: {:?} Median: {:?} Mode: {:?} with {:?} entries",
        mean(&v),
        median(&v),
        max.0,
        max.1
    );
}
