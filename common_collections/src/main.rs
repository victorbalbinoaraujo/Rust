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
}
