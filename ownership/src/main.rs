fn main() {
    let s1: String = String::from("texto1");
    let s2: String = s1;

    //println!("{s1}"); // Error: borrow of moved value
    println!("{s2}");

    let s3: String = String::from("texto2");
    let s4: String = s3.clone();

    println!("s3 = {s3} s4 = {s4}");

    let s5: String = String::from("texto3");
    take_ownership(s5);

    let x: i32 = 5;
    make_a_copy(x);

    let s6: String = gives_ownership();
    println!("{s6}");

    let s7: String = String::from("texto4");
    let s8: String = takes_and_gives_back(s7);
    println!("{s8}");

    let s9: String = String::from("texto5");
    let len_string: usize = calc_len(&s9);
    println!("Length of {s9} is {len_string}");

    let mut s10: String = String::from("texto");
    change(&mut s10);

    let mut s11: String = String::from("texto6");

    let r1: &mut String = &mut s11;
    // let r2: &mut String = &mut s11; // cannot borrow s11 as mutable more than once

    println!("{r1}");

    let s12: String = String::from("texto7");

    let r2: &String = &s12;
    let r3: &String = &s12;
    // let r4 = &mut s12; // cannot borrow s12 as mutable because it is also borrowed as immutable

    println!("{s12} {r2} {r3}");

    let s13: String = String::from("texto8");

    let slice1: &str = &s13[..3];
    let slice2: &str = &s13[3..];
    let slice3: &str = &s13[..];

    println!("{slice1} {slice2} {slice3}");

    let s14: String = String::from("texto longo");
    let word = first_word(&s14);

    println!("{word}");
}

fn take_ownership(s: String) {
    println!("{s}")
}

fn make_a_copy(i: i32) {
    println!("{i}")
}

fn gives_ownership() -> String {
    let s: String = String::from("Olá");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" longo")
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
