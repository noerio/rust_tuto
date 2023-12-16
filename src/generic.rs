// Fill in the blanks to make it work
struct A; // Concrete type `A`.
struct S(A); // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn ex1() {
    let a = A;
    let s = S(a);
    let sa = SGen::<A>(A);
    // Using the non-generic functions
    reg_fn(s); // Concrete type.
    gen_spec_t(sa); // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen::<i32>(5)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen::<char>('c'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen::<char>('v'));

    println!("Success!");
}

// Implement the generic function below.
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn ex2() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}

// Implement struct Point to make it work.
#[derive(Debug)]
struct Point3<T> {
    _x: T,
    _y: T,
}

fn ex3() {
    let integer = Point3 { _x: 5, _y: 10 };
    let float = Point3 { _x: 1.0, _y: 4.0 };

    println!("{:?}, {:?}", integer, float);
    println!("Success!");
}

// Modify this struct to make the code work
struct Point4<T1, T2> {
    _x: T1,
    _y: T2,
}

fn ex4() {
    // DON'T modify this code.
    let _p = Point4 {
        _x: 5,
        _y: "hello".to_string(),
    };

    println!("Success!");
}

// Add generic for Val to make the code work, DON'T modify the code in `ex5`.
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn ex5() {
    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}

struct Point6<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point6<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<T2, U2>(self, p: Point6<T2, U2>) -> Point6<T, U2> {
        Point6 { x: self.x, y: p.y }
    }
}

fn ex6() {
    let p1 = Point6 { x: 5, y: 10 };
    let p2 = Point6 {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

// Fix the errors to make the code work.
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn ex7() {
    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_from_origin());
}

pub fn generic_ex() {
    println!("Generic:");
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
}
