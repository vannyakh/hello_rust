fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    // variable binding
    let x = 5;
    let (y, z) = (1, 2);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    // mutable variable binding
    let mut a = 5;
    println!("The value of a is: {}", a);
    a = 6;
    println!("The value of a is: {}", a);
    // type annotations
    let b: i32 = 5;
    println!("The value of b is: {}", b);
    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // shadowing
    let c = 5;
    let c = c + 1;
    let c = c * 2;
    println!("The value of c is: {}", c);
    // floating-point types
    let d = 2.0; // f64
    let e: f32 = 3.0; // f32
    println!("The value of d is: {}", d);
    println!("The value of e is: {}", e);
    // numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5; // remainder
    println!("The value of sum is: {}", sum);   
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);   
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);
    // boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);   
    // character type
    let g = 'z';
    let h = 'ℤ';
    let i = '😻';
    println!("The value of g is: {}", g);
    println!("The value of h is: {}", h);
    println!("The value of i is: {}", i);
    // compound types
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (j, k, l) = tup;
    println!("The value of j is: {}", j);
    println!("The value of k is: {}", k);
    println!("The value of l is: {}", l);
    // array type
    let m = [1, 2, 3, 4, 5];
    let n: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
    let o = [3; 5]; // [3, 3, 3, 3, 3]
    let first = m[0];
    let second = m[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    // functions
    another_function();
    another_function_with_parameter(5);
    let p = another_function_with_return_value();
    println!("The value of p is: {}", p);
    let q = another_function_with_parameter_and_return_value(5);
    println!("The value of q is: {}", q);
    // control flow
    // if expression
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if expression with else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // using if in a let statement
    let condition = true;
    let r = if condition {
        5
    } else {
        6
    };
    println!("The value of r is: {}", r);
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);
    // while
    let mut s = 3;
    while s != 0 {
        println!("{}!", s);
        s -= 1;
    }
    println!("LIFTOFF!!!");
    // for
    let t = [10, 20, 30, 40, 50];
    for element in t.iter() {
        println!("the value is: {}", element);
    }
    // ownership
    let u = String::from("hello");
    takes_ownership(u);
    // println!("The value of u is: {}", u); // error: value borrowed here after move
    let v = 5;
    makes_copy(v);
    println!("The value of v is: {}", v);
    let w = gives_ownership();
    println!("The value of w is: {}", w);
    let x = 5;
    let y = takes_and_gives_back(x);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // references and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is: {}", s1, len);
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The value of s2 is: {}", s2);
    // slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("The value of hello is: {}", hello);
    println!("The value of world is: {}", world);
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];
    // structs
    let mut user1 = User {
        username: String::from("someone"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("example@gmail.com");
    let user2 = build_user(String::from("another"), String::from("gust"));
    let user3 = User {
        username: String::from("another"),
        email: String::from("user3@gmail.com"),
        ..user1 // struct update syntax
    };
    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // unit-like structs
    let unit = Unit;
    // methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is: {}", rect1.area());
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    let rect3 = Rectangle::square(3);
    println!("The area of the square is: {}", rect3.area());
    // enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("about:home"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let home = IpAddrKind::V4(String::from("192.323.332.123"));
    let loopback = IpAddrKind::V6(String::from("::1"));
    // option enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // match
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of coin is: {}", value);
    let coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(coin);
    println!("The value of coin is: {}", value);
    // if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // while let
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    while let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        count += 1;
        if count == 3 {
            break;
        }
    }
    // let
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    let (x, y, z): (i32, f64, u8) = (1, 2.0, 3);
    // function pointers
    let f: fn(i32) -> i32 = plus_one;
    let six = f(5);
    println!("The value of six is: {}", six);   
    // closures
    let plus_one = |x: i32| -> i32 { x + 1 };
    let six = plus_one(5);
    println!("The value of six is: {}", six);
    let plus_two = |x| { x + 2 };
    let eight = plus_two(6);
    println!("The value of eight is: {}", eight);
    let plus_three = |x| x + 3;
    let eleven = plus_three(8);
    println!("The value of eleven is: {}", eleven);
    let num_list = vec![34, 50, 25, 100, 65];
    let largest = largest(&num_list);
    println!("The largest number is: {}", largest);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = largest(&char_list);
    println!("The largest char is: {}", largest);
    // iterators
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
    // consuming adaptors
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("The value of v2 is: {:?}", v2);
    // iterator trait
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("The total is: {}", total);
    // iterator trait methods
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    // using closures that capture their environment
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    // move
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // error: value borrowed here after move
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    // ownership of variables in closures
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // error: value borrowed here after move
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    // string slices
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The value of word is: {}", word);
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let word = first_word(&s);
    println!("The value of word is: {}", word);
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The value of word is: {}", word);
    let s = String::from("hello");
    let word = first_word(&s);
    println!("The value of word is: {}", word);
    let s = "hello world";
    let word = first_word(&s);
    println!("The value of word is: {}", word);
    // string literals are slices
}


// function definition
fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_with_return_value() -> i32 {
    5
}

fn another_function_with_parameter_and_return_value(x: i32) -> i32 {
    x
}

fn takes_ownership(some_string: String) {
    println!("The value of some_string is: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("The value of some_integer is: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(some_integer: i32) -> i32 {
    some_integer
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Unit;

// struct methods
struct Rectangle {
    width: u32,
    height: u32,
}

