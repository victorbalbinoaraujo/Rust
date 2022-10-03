struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    fn new(name: String, age: u32, weight: f32) -> Self {
        Self {
            name: (name),
            age: (age),
            weight: (weight),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

fn main() {
    let mut bob: User = User::new(String::from("Bob"), 33, 155.2);
    println!("{} {} {}", bob.name(), bob.age(), bob.weight());

    bob.set_age(34);
    bob.set_weight(134.90);

    println!("{} {} {}", bob.name(), bob.age(), bob.weight());
}
