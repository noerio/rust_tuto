fn ex1() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

fn ex2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

// Fix error
fn ex3() {
    let s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(_s: &String) {}

// Fix error
fn ex4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn ex5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

fn ex6() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let r2;
    r2 = &c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// Remove something to make it work
// Don't remove a whole line !
fn ex7() {
    let mut s = String::from("hello");

    let r1 = s.clone();
    let r2 = &mut s;

    print!("{}, {}, ", r1, r2);

    println!("Success!");
}

fn ex8() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object_mut(&mut s);

    println!("Success!");
}

fn borrow_object_mut(_s: &mut String) {}

// This code has no errors!
fn ex9() {
    let mut s = String::from("hello, ");

    borrow_object_9(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object_9(_s: &String) {}

// Comment one line to make it work
fn ex10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // println!("{}", r1);
    println!("Success!");
}

fn ex11() {
    let mut s = String::from("hello, ");

    let _r1 = &mut s;
    let _r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time

    // println!("{}", _r1); // uncomment to make it crash
    println!("Success!");
}

pub fn borrowing_ex() {
    println!("Borrowing:");
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
    print!("\nExercice 10: ");
    ex10();
    print!("\nExercice 11: ");
    ex11();
}
