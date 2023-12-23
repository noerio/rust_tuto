trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn _fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn ex1() {
    // FILL in the blank.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

// IMPLEMENT this function.
fn hatch_a_bird(id: i32) -> Box<dyn Bird> {
    match id {
        1 => Box::new(Swan {}),
        2 => Box::new(Duck {}),
        i32::MIN..=0_i32 | 3_i32..=i32::MAX => todo!(),
    }
}

trait Bird2 {
    fn quack(&self);
}

struct Duck2;
impl Duck2 {
    fn _fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan2;
impl Swan2 {
    fn _fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird2 for Duck2 {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird2 for Swan2 {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn ex2() {
    // FILL in the blank to make the code work.
    let birds: [Box<dyn Bird2>; 4] = [
        Box::new(Duck2),
        Box::new(Swan2),
        Box::new(Swan2),
        Box::new(Duck2),
    ];

    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly();
    }
}

// FILL in the blanks.
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn ex3() {
    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// IMPLEMENT below with generics.
fn static_dispatch<T: Foo>(x: T) {
    println!("{}", x.method());
}

// Implement below with trait objects.
fn dynamic_dispatch(x: &dyn Foo) {
    println!("{}", x.method());
}

fn ex4() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}

// Use at least two approaches to make it work.
// DON'T add/remove any code line.
use std::any::Any;

trait MyTrait {
    fn f(&self) -> Box<dyn Any>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn Any> {
        Box::new(42)
    }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn Any> {
    x.f()
}

fn ex5() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}

pub fn objtraits_ex() {
    println!("Traits Objects:");
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
}
