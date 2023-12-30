// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn ex1() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("2", "2");
    assert_eq!(result.unwrap(), 4);

    println!("Success!");
}

// IMPLEMENT multiply with ?
// DON'T use unwrap here
fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    Ok(n1_str.parse::<i32>()? * n2_str.parse::<i32>()?)
}

fn ex2() {
    assert_eq!(multiply2("3", "4").unwrap(), 12);
    println!("Success!");
}

use std::fs::File;
use std::io::{ self, Read };

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// FILL in the blanks with one code line
// DON'T change any code lines
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn ex3() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
}

// FILL in the blank in two ways: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}

fn ex4() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}

// With the return type rewritten, we use pattern matching without `unwrap()`.
// But it's so Verbose...
fn multiply3(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => {
            match n2_str.parse::<i32>() {
                Ok(n2) => { Ok(n1 * n2) }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// Rewriting `multiply` to make it succinct
// You should use BOTH of  `and_then` and `map` here.
fn multiply4(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn ex5() {
    // This still presents a reasonable answer.
    let twenty = multiply4("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply3("t", "2");
    print(tt);

    println!("Success!");
}

// FILL in the blank
type Res<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply6(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str
        .parse::<i32>()
        .and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
}

// Here, the alias again allows us to save some space.
fn print6(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn ex6() {
    print6(multiply6("10", "2"));
    print6(multiply6("t", "2"));

    println!("Success!");
}

pub fn result_ex() {
    println!("Result:");
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
