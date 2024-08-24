use std::io;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn calculate_area_of_rectangle(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        match self.width > other.width && self.height > other.height {
            true => true,
            false => false,
        }
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Enter side a: ");
    let a: i32 = get_user_input_as_number();
    println!("Now enter side b: ");
    let b: i32 = get_user_input_as_number();

    let sides = Rectangle {
        width: a,
        height: b,
    };

    println!(
        "The area of the rectangle is: {}",
        sides.calculate_area_of_rectangle()
    );

    let new_one = Rectangle {
        width: 23,
        height: 24,
    };

    println!(
        "A new wild Rectangle appeared! width: {}, height: {}",
        new_one.width, new_one.height
    );

    match sides.can_hold(&new_one) {
        true => println!("The Rectangle can be inside the first one you entered!"),
        false => println!("The Rectangle can't be inside the first one you entered!"),
    }

    let square = Rectangle::square(new_one.width);

    println!(
        "A square made out of the wild Rectangle's width!: {:?}",
        square
    );

    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    match localhost.is_ipv4() {
        true => println!("127.0.0.1 is an ipv4 address..."),
        false => println!("127.0.0.1 is not an ipv4 address..."),
    }
}

fn get_user_input_as_number() -> i32 {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unexpected outcome happened...");
    s.trim().parse().expect("Not a number!")
}
