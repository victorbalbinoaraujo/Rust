fn expected_minutes_in_oven() -> i32 {
    40
}

fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    (expected_minutes_in_oven() - remaining_minutes_in_oven(actual_minutes_in_oven))
        + preparation_time_in_minutes(number_of_layers)
}

fn main() {
    println!("{}", expected_minutes_in_oven());
    println!("{}", preparation_time_in_minutes(2));
    println!("{}", remaining_minutes_in_oven(30));
    println!("{}", elapsed_time_in_minutes(3, 20));
}
