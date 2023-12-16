// Fix the errors
enum Number {
    _Zero,
    One,
    _Two,
}

enum Number1 {
    _Zero = 0,
    One,
    _Two,
}

// C-like enum
enum Number2 {
    _Zero = 0,
    One = 1,
    _Two = 2,
}

fn ex1() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);

    println!("Success!");
}

// Fill in the blank
enum Message2 {
    _Quit,
    Move { _x: i32, _y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

fn ex2() {
    let _msg1 = Message2::Move { _x: 1, _y: 2 }; // Instantiating with x = 1, y = 2
    let _msg2 = Message2::Write(String::from("hello, world!")); // Instantiating with "hello, world!"

    println!("Success!");
}

// Fill in the blank and fix the error
enum Message3 {
    _Quit,
    Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

fn ex3() {
    let msg = Message3::Move { x: 1, y: 1 };

    if let Message3::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

// Fill in the blank and fix the errors
#[derive(Debug)]
enum Message4 {
    Quit,
    Move { _x: i32, _y: i32 },
    _Write(String),
    ChangeColor(i32, i32, i32),
}

fn ex4() {
    let msgs: [Message4; 3] = [
        Message4::Quit,
        Message4::Move { _x: 1, _y: 3 },
        Message4::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message4) {
    println!("msg: {:?}", msg);
}

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn ex5() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    if let Some(n) = six {
        println!("n: {:?}", n);

        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

use crate::enum_f::List::*;
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // After Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // Instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn ex6() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

pub fn enum_ex() {
    println!("Enum:");
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
