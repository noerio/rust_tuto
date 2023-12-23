// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
fn ex1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, String::from("hello, world!"));

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

// FILL in the blanks
fn ex2() {
    let s = String::from("hello, world");

    let slice1: &str = s.as_str(); // In two ways
    // let slice1: &str = "hello, world"; // In two ways
    assert_eq!(slice1, "hello, world");

    let slice2: &str = &s[..5];
    assert_eq!(slice2, "hello");

    let mut slice3: String = s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}

// Question: how many heap allocations are happening here?
// Your answer: 2
fn ex3() {
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!");

    // Create a slice point to String `s`
    let slice: &str = &s;

    // Create a String type based on the recently created slice
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

// FILL in the blank and FIX errors
fn ex4() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!");
}

// FILL in the blanks
fn ex5() {
    let mut s = String::new();
    s.push_str("hello");

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("Success!");
}

// Modify the code below to print out:
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
fn ex6() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!");
}

// FILL in the blanks
use std::mem;

fn ex7() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}

pub fn collection_string_ex() {
    println!("Collection String:");
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
}
