struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let mut usr1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        sign_in_count: 1,
        active: true,
    };

    usr1.email = String::from("anotheremail@example.com");

    let usr2: User = build_user(
        String::from("someone2@example.com"),
        String::from("someoneusername456"),
    );

    println!("{}", usr2.username);

    let usr3: User = User {
        email: String::from("someone3@example.com"),
        username: String::from("someoneusername789"),
        sign_in_count: usr1.sign_in_count,
        active: usr1.active,
    };

    println!("{}", usr3.active);

    let usr4: User = User {
        email: String::from("someone4@example.com"),
        username: String::from("someoneusername101112"),
        ..usr1
    };

    println!("{} {}", usr4.active, usr4.sign_in_count); // Same as usr1

    // ---- Struct Tuple ----
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    println!("{} {}", black.0, origin.1);

    // ---- Example Program ----

    let rect1: Rectangle = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect2: Rectangle = Rectangle {
        length: 40,
        width: 20,
    };
    let rect3: Rectangle = Rectangle {
        length: 45,
        width: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
