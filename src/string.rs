// Fix error without adding new line
fn ex1() {
    let s: &str = "hello, world";

    println!("{}", s);
}

// Fix the error with at least two solutions
fn ex2() {
    let s: Box<str> = "hello, world".into();
    greetings_2(&s)
}

fn greetings_2(s: &str) {
    println!("{}", s)
}

// Fill the blank
fn ex3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

// Fix all errors without adding newline
fn ex4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

// Fill the blank
fn ex5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

// Fix errors without removing any line
fn ex6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + s2.as_str();
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

// Fix error with at least two solutions
fn ex7() {
    let s = "hello, world";
    greetings_7(s);
    greetings_7_bis(s.to_string());
}

fn greetings_7(s: &str) {
    print!("{}; ", s);
}

fn greetings_7_bis(s: String) {
    println!("{}", s);
}

// Use two approaches to fix the error and without adding a new line
fn ex8() {
    let s = "hello, world".to_string();
    let _s1: &str = s.as_str();

    println!("Success!");
}

fn ex8bis() {
    let s = "hello, world";
    let _s1: &str = s;

    println!("Success!");
}

fn ex9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string =
        "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

/* Fill in the blank and fix the errors */
fn ex10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

fn ex11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success!");
}

fn ex12() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        print!("{}", c);
    }
    println!();
}

pub fn string_ex() {
    println!("String:");
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
    print!("\nExercice 08B: ");
    ex8bis();
    print!("\nExercice 09: ");
    ex9();
    print!("\nExercice 10: ");
    ex10();
    print!("\nExercice 11: ");
    ex11();
    print!("\nExercice 12: ");
    ex12();
}
