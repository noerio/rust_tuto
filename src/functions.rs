fn ex1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s: i32 = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn ex2() {
    print();
}

// Replace i32 with another type
fn print() -> () {
    println!("Success!");
}

// DON'T let `println!` works
fn ex3() {
    // _never_return(); // uncomment to see how it works

    println!("Failed!");
}

fn _never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("I return nothing!")
}

fn ex4() {
    // _get_option(38u8); // uncomment to see how it works
    println!("Success!");
}

fn _get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            //TODO
        }
        _ => {
            //TODO
        }
    }

    // Rather than returning a None, we use a diverging function instead
    _never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn _never_return_fn() -> ! {
    // panic!() // uncomment to see how it works
    unimplemented!()
    // todo!() // uncomment to see how it works
}

fn ex5() {
    // FILL in the blank
    let b = true; // set b to false to see how it works

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

pub fn functions_ex() {
    println!("Functions:");
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
