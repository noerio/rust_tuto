
// Fix the error below with least amount of modification to the code
fn ex1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32 = 5; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, y);
    print!("Success!");
}


// Fill the blanks in the code to make it compile
fn ex2() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    print!("Success!");
}


// Fix the error below with least amount of modification
fn ex3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        print!("The value of x is {} and value of y is {}", x, y);
    }
    print!("The value of x is {} and value of y is {}", x, y); 
}


// Fix the error with the use of define_x
fn ex4() {
    define_x()
}

fn define_x() {
    let x = "hello";
    print!("{}, world", x); 
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
    print!("{}", x); // Prints "42".
}


// Remove a line in the code to make it compile
fn ex6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    print!("Success!");
}


// #[allow(unused_variables)] first solution
fn ex7() {
    let _x = 1; // second solution "x" -> "_x"
}


// Fix the error below with least amount of modification
fn ex8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    print!("Success!");
}


fn ex9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    print!("Success!");
}


fn main() {
    println!("Exercice 01: {}", ex1());
    println!("Exercice 02: {}", ex2());
    println!("Exercice 03: {}", ex3());
    println!("Exercice 04: {}", ex4());
    println!("Exercice 05: {}", ex5());
    println!("Exercice 06: {}", ex6());
    println!("Exercice 07: {}", ex7());
    println!("Exercice 08: {}", ex8());
    println!("Exercice 09: {}", ex9());
}