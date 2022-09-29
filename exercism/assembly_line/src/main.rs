fn production_rate_per_hour(speed: u8) -> f64 {
    let production: f64 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    production * speed as f64 * 221 as f64
}

fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60 as u32
}

fn main() {
    println!("{}", production_rate_per_hour(6));
    println!("{}", working_items_per_minute(6));
}
