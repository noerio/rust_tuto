// Make it work
use std::mem::size_of_val;
fn ex1() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

// Make it work
fn ex2() {
    let c1: char = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

// Make println! work
fn ex3() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

// Make it work
fn ex4() {
    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

// Make it work, don't modify `implicitly_ret_unit` !
fn ex5() {
    let v: () = ();

    let _v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn _explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

// Modify `4` in assert to make it work
fn ex6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

pub fn char_bool_unit_ex() {
    println!("Char, Bool & Unit:");
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
