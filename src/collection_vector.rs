fn ex1() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(v);

    let v = vec![1, 2, 3];
    is_vec(v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec![1, 2, 3];
    is_vec(v.clone());

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    let v1 = Vec::from([1, 2, 3]);
    is_vec(v1.clone());

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(_v: Vec<u8>) {}

// FILL in the blank
fn ex2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend(v1.clone());

    assert_eq!(v1, v2);

    println!("Success!");
}

// FILL in the blanks
fn ex3() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.to_vec();

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.as_bytes().to_vec();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}

// FIX the error and IMPLEMENT the code
fn ex4() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..v.len() {
        print!("{:?} ", v[i]);
    }

    for i in 0..v.len() {
        v[i] += 1;
    }

    v.push(5);
    v.push(6);

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}

// FIX the errors
fn ex5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..3].to_vec();
    slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}

// FIX the errors
fn ex6() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!");
}

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn ex7() {
    // FILL in the blank
    let v: Vec<IpAddr> = {
        let mut v = Vec::new();
        v.push(IpAddr::V4(String::from("127.0.0.1")));
        v.push(IpAddr::V6(String::from("::1")));
        v
    };

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}

trait IpAddr8 {
    fn display(&self);
}

struct V4(String);
impl IpAddr8 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddr8 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn ex8() {
    // FILL in the blank
    let v: Vec<Box<dyn IpAddr8>> = vec![Box::new(V4("127.0.0.1".to_string())), Box::new(V6("::1".to_string()))];

    for ip in v {
        ip.display();
    }
}

pub fn collection_vector_ex() {
    println!("Collection Vector:");
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
}
