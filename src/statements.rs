// Make it work with two ways
fn ex1() {
    let v = {
        let x = 1;

        x + 2
    };

    let v_bis = {
        let mut x = 1;
        x += 2;

        x
    };

    assert_eq!(v, 3);
    assert_eq!(v_bis, 3);

    println!("Success!");
}

fn ex2() {
    let v = {
        let x = 3;

        x
    };

    assert!(v == 3);

    println!("Success!");
}

fn ex3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

pub fn statements_ex() {
    println!("Statements & Expressions:");
    print!("Exercice 01: ");
    ex1();
    print!("\nExercice 02: ");
    ex2();
    print!("\nExercice 03: ");
    ex3();
}
