// Fill in the blanks
fn ex1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

// Fix the errors
fn ex2() {
    let n = 5.0;

    let big_n = {
        if n < 10.0 && n > -10.0 {
            println!(", and is a small number, increase ten-fold");

            10.0 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0
        }
    };

    println!("{} -> {}", n, big_n);
}

fn ex3() {
    for n in 1..100 {
        // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}

// Fix the errors without adding or removing lines
fn ex4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
        print!("{} ", name);
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
        print!("{} ", n);
    }

    println!("{:?}", numbers);
}

fn ex5() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

// Fill in the blanks to make the last println! work !
fn ex6() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n < 20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

// Fill in the blank
fn ex7() {
    let mut n = 0;
    for _i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

// Fill in the blanks
fn ex8() {
    let mut n = 0;
    for _i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        continue;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

// Fill in the blanks
fn ex9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

// Fill in the blank
fn ex10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

// Fill in the blank
fn ex11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}

pub fn flow_control_ex() {
    println!("Flow Control:");
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
