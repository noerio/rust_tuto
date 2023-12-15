// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn ex1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!(
        "{} is {} years old, and his hobby is {}.",
        p.name, p.age, p.hobby
    );
}

struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {}
fn ex2() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(_u: Unit) {}

// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn ex3() {
    let v = Point(0, 127, 255);
    check_color(Color(v.0, v.1, v.2));

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
}

// Fill the blank and fix the error without adding/removing new line
struct Person4 {
    name: String,
    age: u8,
}

fn ex4() {
    let age = 18;
    let mut p = Person4 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

// Fill the blank
struct Person5 {
    name: String,
    age: u8,
}

fn ex5() {
    let p = build_person(String::from("Bernard"), 18);
    println!("{} is {} years old.", p.name, p.age);
}

fn build_person(name: String, age: u8) -> Person5 {
    Person5 { name, age }
}

// Fill the blank to make the code work
struct User {
    active: bool,
    username: String,
    _email: String,
    sign_in_count: u64,
}

fn ex6() {
    let u1 = User {
        _email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let _u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        _email: String::from("contact@im.dev"),
        username: u.username,
        active: u.active,
        sign_in_count: u.sign_in_count,
    }
}

// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

fn ex7() {
    let scale = 2;
    let rect1 = Rectangle {
        _width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        _height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("rect1: {:?}", rect1); // Print debug info to stdout
}

// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn ex8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}, {}", _name, f.data);
}

pub fn struct_ex() {
    println!("Struct:");
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
    print!("\nExercice 07: ");
    ex7();
    print!("\nExercice 08: ");
    ex8();
}
