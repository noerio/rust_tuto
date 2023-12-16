struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn ex1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}

// Only fill in the blanks, DON'T remove any line!
#[derive(Debug)]
struct TrafficLight2 {
    color: String,
}

impl TrafficLight2 {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}
fn ex2() {
    let light = TrafficLight2 {
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}

struct TrafficLight3 {
    color: String,
}

impl TrafficLight3 {
    // Using `Self` to fill in the blank.
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}

fn ex3() {
    let mut t = TrafficLight3 {
        color: "red".to_owned(),
    };
    t.change_state();
    t.show_state();
    println!("Success!");
}

#[derive(Debug)]
struct TrafficLight4 {
    color: String,
}

impl TrafficLight4 {
    // 1. Implement an assotiated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn ex4() {
    let light = TrafficLight4::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}

struct Rectangle5 {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle5 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle5 {
    fn can_hold(&self, other: &Rectangle5) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn ex5() {
    let r1 = Rectangle5 {
        width: 10,
        height: 10,
    };
    let r2 = Rectangle5 {
        width: 30,
        height: 40,
    };
    println!("{}", r1.area());
    println!("{}", r2.can_hold(&r1));
    println!("Success!");
}

#[derive(Debug)]
enum TrafficLightColor {
    _Red,
    _Yellow,
    _Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            TrafficLightColor::_Green => "green",
            TrafficLightColor::_Red => "red",
            TrafficLightColor::_Yellow => "yellow"
        }
    }
}

fn ex6() {
    let c = TrafficLightColor::_Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}

pub fn method_ex() {
    println!("Method:");
    print!("Exercice 01: ");
    ex1();
    print!("\nExercice 02: ");
    ex2();
    print!("\nExercice 03: ");
    ex3();
    print!("\nExercice 04: ");
    ex4();
    print!("\nExercice 05: ");
    ex5();
    print!("\nExercice 06: ");
    ex6();
}
