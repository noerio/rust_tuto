// Fix the error below with least amount of modification to the code
fn ex1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32 = 5; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, y);
    println!("Success!");
}

// Fill the blanks in the code to make it compile
fn ex2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

// Fix the error below with least amount of modification
fn ex3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

// Fix the error with the use of define_x
fn ex4() {
    define_x()
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn ex5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Remove a line in the code to make it compile
fn ex6() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    _x += 3;

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

// #[allow(unused_variables)] first solution
fn ex7() {
    let _x = 1; // second solution "x" -> "_x"

    println!("Success!");
}

// Fix the error below with least amount of modification
fn ex8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn ex9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}

pub fn variables_ex() {
    println!("Variables:");
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
