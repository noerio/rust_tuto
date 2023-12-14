// Remove something to make it work
fn ex1() {
    let x: i32 = 5;
    let mut _y: i32 = 5;

    _y = x;

    let _z: i32 = 10; // Type of z ?

    println!("Success!");
}

// Fill the blank
fn ex2() {
    let v: u16 = 38_u8 as u16;

    println!("{}, Success!", v);
}

// Modify `assert_eq!` to make it work
fn ex3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// Fill the blanks to make it work
fn ex4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// Fix errors and panics to make it work
fn ex5() {
    let v1: u16 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();

    println!("{}, {}", v1, v2);
}

// Modify `assert!` to make it work
fn ex6() {
    let v: i32 = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

// Fill the blank to make it work
fn ex7() {
    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    assert_eq!(type_of(&y), "f32".to_string());
    assert_eq!(type_of(&z), "f64".to_string());
    println!("Success!");
}

fn ex8() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!((0.1 as f32) + (0.2 as f32) == (0.3 as f32));

    println!("Success!");
}

fn ex9() {
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        print!("{} ", c as u8);
    }
    println!();
}

// Fill the blanks
use std::ops::{ Range, RangeInclusive };
fn ex10() {
    assert_eq!(1..5, Range { start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));

    println!("Success!");
}

// Fill the blanks and fix the errors
fn ex11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    print!("0011 AND 0101 is {:04b}; ", 0b0011u32 & 0b0101);
    print!("0011 OR 0101 is {:04b}; ", 0b0011u32 | 0b0101);
    print!("0011 XOR 0101 is {:04b}; ", 0b0011u32 ^ 0b0101);
    print!("1 << 5 is {}; ", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

pub fn numbers_ex() {
    println!("Numbers:");
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
