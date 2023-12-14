// Fix the errors, DON'T add new lines!
fn ex1() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2: &str = "hello, world" as &str;

    println!("Success!");
}

fn ex2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

fn ex3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

fn ex4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = slice1;

    assert_eq!(slice1, slice2);

    println!("Success!");
}

fn ex5() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}

// Fix errors
fn ex6() {
    let s = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want know more, this is called `Deref coercion`.
    let word = first_word(&s);

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    &s[..1]
}

pub fn slice_ex() {
    println!("Slice:");
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
