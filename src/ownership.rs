fn ex1() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("hello, world");
    let y: String = x.clone();
    println!("x: {}, y: {}", x, y);
}

// Don't modify code in ex2!
fn ex2() {
    let s1: String = String::from("hello, world");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    s
}

fn ex3() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    // Convert String to Vec
    let _s: &[u8] = s.as_bytes();
    s
}

// Fix the error without removing code line
fn ex4() {
    let mut s = String::from("hello, world");

    s = print_str(s);

    println!("{}", s);
}

fn print_str(s: String) -> String {
    print!("{}, ", s);
    s
}

// fn print_str(s: &String) {
//     print!("{}, ", s);
// }

// Don't use clone ,use copy instead
fn ex5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

fn ex6() {
    let s = String::from("hello, ");

    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}

fn ex7() {
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(0); // Implement this line, dont change other lines!

    *y = 4;

    assert_eq!(*y, 4);
    assert_eq!(*x, 5);

    println!("Success!");
}

fn ex8() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s: &String = &t.0;

    print!("_s: {}; ", _s);

    // Modify this line only, don't use `_s`
    println!("{:?}", t);
}

fn ex9() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2): (String, String) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

pub fn ownership_ex() {
    println!("Ownership:");
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
    print!("\nExercice 09: ");
    ex9();
}
