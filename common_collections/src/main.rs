use std::collections::HashMap;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.pop();

    println!("{:?}", v1);

    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];

    let _third: &i32 = &v2[2]; // if not -> panic!
    let _third: Option<&i32> = v2.get(2); // if not -> None

    let row: [SpreadSheetCell; 3] = [
        SpreadSheetCell::Int(5),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text(String::from("Blue")),
    ];

    println!("{:?}", row);


    let mut s1: String = String::from("foo");
    let s2: String = String::from("bar");
    s1.push_str(&s2);

    let mut s: String = String::from("lo");
    s.push('l');

    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");

    let s: String = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams: Vec<String> = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores: Vec<i32> = vec![10, 50];

    let scores:HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    

}
