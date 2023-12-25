// FIX the errors and FILL in the blank
// DON'T remove any code
fn ex1() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let _c1: char = integer as char;
    let _c2 = integer as char;

    assert_eq!(integer, 'a' as u8);

    println!("Success!");
}

#[allow(overflowing_literals)]
fn ex2() {
    assert_eq!(u8::MAX, 255);
    // The max of `u8` is 255 as shown above.
    // so the below code will cause an overflow error: literal out of range for `u8`.
    // PLEASE looking for clues within compile errors to FIX it.
    // DON'T modify any code in main.
    let _v = 1000 as u8;

    println!("Success!");
}

#[allow(overflowing_literals)]
fn ex3() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", (300.0_f32).to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

// FILL in the blanks
fn ex4() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32;
    unsafe {
        // Add one to the second element
        *p2 += 1;
    }

    assert_eq!(values[1], 3);

    println!("Success!");
}

fn ex5() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13);
    }

    println!("Success!");
}

fn ex6() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you 
   should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let _i3: i32 = 'a' as i32;

    // FIX the error in two ways
    let _s: String = String::from("a");

    println!("Success!");
}

// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // IMPLEMENT `from` method
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// FILL in the blanks
fn ex7() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = (5).into();
    assert_eq!(num.value, 5);

    println!("Success!");
}

use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // IMPLEMENT from method
    fn from(err: io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<num::ParseIntError> for CliError {
    // IMPLEMENT from method
    fn from(err: num::ParseIntError) -> Self {
        CliError::ParseError(err)
    }
}

fn _open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn ex8() {
    println!("Success!");
}

// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
use std::convert::TryInto;

fn ex9() {
    let n: i16 = 256;

    // Into trait has a method `into`,
    // hence TryInto has a method ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!");
}

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 { Ok(EvenNum(value)) } else { Err(()) }
    }
}

fn ex10() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = (8i32).try_into();
    assert_eq!(result, (8i32).try_into());
    let result: Result<EvenNum, ()> = (5i32).try_into();
    assert_eq!(result, (5i32).try_into());

    println!("Success!");
}

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)?;
        Ok(())
    }
}

fn ex11() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin.to_string()), "The point is (0, 0)");

    println!("Success!");
}

// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;
fn ex12() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point13 {
    x: i32,
    y: i32,
}

impl FromStr for Point13 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| (p == '(' || p == ')'))
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point13 { x: x_fromstr, y: y_fromstr })
    }
}

fn ex13() {
    // FILL in the blanks in two ways
    // DON'T change code anywhere else
    let p = "(3,4)".parse::<Point13>();
    assert_eq!(p.unwrap(), Point13 { x: 3, y: 4 });

    println!("Success!");
}

pub fn type_conversion_ex() {
    println!("Type Conversion:");
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
    print!("\nExercice 12: ");
    ex12();
    print!("\nExercice 13: ");
    ex13();
}
