use std::io;

fn main() {
    println!("Temperature");

    let mut temperature: String = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: i32 = match temperature.trim().parse() {
        Ok(temperature) => temperature,
        Err(_e) => {
            panic!("Invalid input!")
        }
    };

    println!("Unit of Temperature - C or F");

    let mut unit_temp: String = String::new();

    io::stdin()
        .read_line(&mut unit_temp)
        .expect("Failed to read unit of temperature");

    match unit_temp.as_str() {
        "C\r\n" => println!("{}", f_to_c(temperature)),
        "F\r\n" => println!("{}", c_to_f(temperature)),
        _ => println!("Invalid unit of temperature!"),
    }
}

fn c_to_f(temperature: i32) -> f64 {
    f64::from(temperature) * (9.0 / 5.0) + 32.0
}

fn f_to_c(temperature: i32) -> f64 {
    (f64::from(temperature) - 32.0) * 5.0 / 9.0
}
