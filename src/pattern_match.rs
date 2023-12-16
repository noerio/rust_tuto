// Fill the blanks
enum Direction {
    _East,
    _West,
    _North,
    _South,
}

fn ex1() {
    let dire = Direction::_South;
    match dire {
        Direction::_East => println!("East"),
        Direction::_North | Direction::_South => {
            // Matching South or North here
            println!("South or North");
        }
        _ => println!("West"),
    };
}

fn ex2() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

// Fill in the blanks
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    _Write(String),
    ChangeColor(i32, i32, i32),
}

fn ex3() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants"),
    }
}

fn ex4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, '0'..='9' | 'A'..='Z' | 'a'..='z'))
    }

    println!("Success!");
}

enum MyEnum {
    Foo,
    Bar,
}

fn ex5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e as i32 == MyEnum::Foo as i32 {
            // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

fn ex6() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);

        println!("Success!");
    };
}

// Fill in the blank
enum Foo7 {
    Bar(u8),
}

#[allow(irrefutable_let_patterns)]
fn ex7() {
    let a = Foo7::Bar(1);

    if let Foo7::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}

enum Foo8 {
    _Bar,
    _Baz,
    _Qux(u32),
}

fn ex8() {
    let a = Foo8::_Qux(10);

    // Remove the codes below, using `match` instead
    match a {
        Foo8::_Bar => println!("match foo::bar"),
        Foo8::_Baz => println!("match foo::baz"),
        Foo8::_Qux(i) => println!("match foo::qux({})", i),
    }
}

// Fix the errors in-place
fn ex9() {
    let age = Some(30);
    if let Some(age) = age {
        // Create a new variable with the same name as previous `age`
        let age = Some(age);
        assert_eq!(age, Some(30));
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}

fn ex10() {
    match_number(1);
    match_number(4);
    match_number(8);
    match_number(11);
}

fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2..=5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn ex11() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: 2, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point {
            x: 0..=5,
            y: y @ (10 | 20 | 30),
        } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// Fix the errors
enum Message12 {
    Hello { id: i32 },
}

fn ex12() {
    let msg = Message12::Hello { id: 5 };

    match msg {
        Message12::Hello { id: id @ (3..=7) } => println!("Found an id in range [3, 7]: {}", id),
        Message12::Hello {
            id: newid @ (10 | 11 | 12),
        } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message12::Hello { id } => println!("Found some other id: {}", id),
    }
}

// Fill in the blank to make the code work, `split` MUST be used
fn ex13() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

// Fill the blank to make the code work
fn ex14() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

// FIX the error with least changing
// DON'T remove any code line
fn ex15() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
    }

    println!("{}", v);
}

pub fn pattern_match_ex() {
    println!("Pattern Match:");
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
    print!("\nExercice 14: ");
    ex14();
    print!("\nExercice 15: ");
    ex15();
}
