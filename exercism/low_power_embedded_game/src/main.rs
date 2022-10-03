fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

struct Position(i16, i16);
impl Position {
    fn manhattan(&self) -> i16 {
        self.1.abs() + self.0.abs()
    }
}

fn main() {
    println!("{:?}", divmod(10, 3));
    println!("{:?}", evens(1_i16..).next());

    let p: i16 = Position(3, 4).manhattan();
    println!("{p}");
}
