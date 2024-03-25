#![allow(unused_variables)] // #![allow|warn|error(rule)] -> global to file
#![allow(dead_code)]

use std::io; // to take user's input
use std::cmp::Ordering; // comparing values
use rand::Rng; // Random


fn chapter_1() {
    // 1. print et variable simple
    let mut toto = 1;
    toto += 2;
    println!("Hello, world!");
    println!("Test format 1 : {}", toto);
    println!("Test format 2 : {toto}");
}

fn chapter_2() {
    // 2. generating random number
    let random_number =
        rand::thread_rng() // ± random basé sur une seed liée au thread
        .gen_range(1..=10); // interval inclusif [start..=end] ou exclusif [start..end[
    println!("generated number : {random_number}");

    loop {
        // 2. take user's input
        println!("Enter number :");
        let mut user_input_str = String::new();
        io::stdin()
            .read_line(&mut user_input_str)
            .expect("Failed to read line");

        // 2. parsing string to int, or crash if it fails
        // let user_input_number: u32 = user_input_str.trim().parse()
        //     .expect("Please type a number!");


        // 2. parsing string to int v2, or ignore if it fails
        let user_input_number: u32 = match user_input_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 2. comparing values
        match user_input_number.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn chapter_3_1() {
    // 3.1 const vs let
    let toto = 3; // type infer from usage
    const TOTO: i32 = 3; // type must be explicitly given

    const fn test() -> i32 {
        return 34;
    }
    // 3.1 only const functions can be assigned to const variables (value is computed during compilation time)
    const TT: i32 = test();
    println!("toto={toto}, TOTO={TOTO}, TT={TT}");

    // 3.1 shadowing
    let x = 5;
    let x = x + 1; // create a new variable, which can have a different type
    // const TT: i32 = TT+1; // does not work with const
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x=12
    }
    println!("The value of x is: {x}"); // x=6
}

fn chapter_3_2() {
    // 3.2 when converting types, we must provide expecting type if it's not obvious
    let parsed_number: i32 = "-42".parse().expect("Not a number!");
    println!("parsed_number={parsed_number}");

    // N* numbers (unsigned) : u8, u16, u32, u64, u128, usize (32 or 64 bits)
    // N numbers (signed <=> have a + or - sign) i8, i16, i32, i64, i128, isize (32 or 64 bits)
    let decimal = 1_000; // 1 000
    let hex = 0x_F0; // 240
    let octal = 0o_14; // 12
    let binary = 0b_0011_0000; // 48
    let byte = b'@'; // ASCII : b'ascii_character'
    println!("decimal={decimal}, hex={hex}, octal={octal}, binary={binary}, byte={byte}");

    // 3.2 floating numbers
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    let _sum = 5 + 10; // addition
    let _difference = 95.5 - 4.3; // subtraction
    let _product = 4 * 30; // multiplication
    let _quotient = 56.7 / 32.2; // division
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5; // remainder

    // 3.2 booleans
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // 3.2 characters
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // 3.2 Compound types : to group multiple values into one type

    // 3.2 Compound types : tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1); // explicit types
    let tup = (500, 6.4, 1);
    // 3.2 use pattern matching to destructure tuple into values
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    // 3.2 accessing tuple values
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // 3.2 Compound types : array
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type
    let a = [3; 5]; // [value_to_copy; x_times] -> [3, 3, 3, 3, 3]
    let _first = a[0];
    let _second = a[1];

}

fn chapter_3_3() {
    // 3.3 expression
    let y = {
        let x = 3;
        x + 1 // if we add ";" it become a statement and won't work
    };
    println!("The value of y is: {y}");

    // 3.3 functions with or without return
    fn _five() -> i32 {
        5
    }
    fn _also_five() -> i32 {
        return 5; // the ";" is optionnal here
    }

    // 3.3 other example with function declared after being called
    let x = plus_one(5);
    println!("The value of x is: {x}");
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
}

fn chapter_3_4() {
    // on utilise plutôt les commentaires comme ça
    let _lucky_number = 7; // que ceux comme ça :
}

fn chapter_3_5() {
    // 3.5 if expression used as a statement
    let number = 3;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 3.5 if used as en expression
    let condition = true;
    let number = if condition { 5 } else { 6 }; // then and else values must have the same type here
    println!("The value of number is: {number}");

    // 3.5 loop expression used as a statement
    let mut counter = 0;
    let result: i32;
    loop {
        counter += 1;

        if counter == 10 {
            result = counter * 2;
            break;
        } else {
            continue; // optionnal here
        }
    };
    println!("The result is {result}");

    // 3.5 loop used as an expression
    counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // result of the expression
        }
    };
    println!("The result is {result}");

    // 3.5 loop label
    let mut count = 0;
    'counting_up: loop { // loop labels must begin with a single quote
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break; // break innermost loop
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // 3.5 while expression
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    };

    // 3.5 looping through a collection with a while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // 3.5 looping through a collection with a for (faster and safer)
    for element in a {
        println!("the value is: {element}");
    }

    // 3.5 looping through a collection of numbers with a for
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn chapter_4_1() {
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // 4.1 scope with the stack
    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // 4.1 scope with the heap
    {
        // Let's request space on the heap
        let _s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }  // this scope is now over, and s is no longer valid (and the space on the hean have been freed)

    // 4.1 copy variable values on the stack
    let x = 5;
    let _y = x; // copy of the value
    // in this case, clone would do exactly the same and the performances wouldn't be impacted

    // 4.1 copy variable values on the heap : move
    let s1 = String::from("test");
    let _s2 = s1; // copy the pointer s1 on the stack, not the data on the heap
    // now s2 is the owner on the data on the heap and s1 is no longer valid
    // so only when s2 goes out of the scope, the drop function will be called (to free the heap)
    // Because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move

    // 4.1 deeply copy variable values on the heap
    let s1 = String::from("hello");
    let _s2 = s1.clone();
    // now both s1 and s2 are alid and has the same value on the heap, in different places

    // 4.1 ownershit and functions
    {
        let s = String::from("hello");  // s comes into scope
        takes_ownership(s); // s's value moves into the function and so is no longer valid then
        // s is not valid anymore
        let x = 5; // x comes into scope
        makes_copy(x);  // x would move into the function,
                        // but i32 is Copy, so it's okay to still use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    // 4.1 returning value and scope
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let _s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back,
        // which also moves its return value into s3
    } // Here, s3 goes out of scope and is dropped.
      // s2 was moved, so nothing happens.
      // s1 goes out of scope and is dropped.

    fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
        let some_string = String::from("yours"); // some_string comes into scope
        some_string // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
        a_string  // a_string is returned and moves out to the calling function
    }
}

fn chapter_4_2() {
    fn calculate_length(s: &String) -> usize { // s is a reference of String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, it is not dropped.

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 is a reference of s1
    println!("The length of '{}' is {}.", s1, len);

    // Note : The opposite of referencing by using `&` is dereferencing,
    // which is accomplished with the dereference operator, `*`

    // The action of creating a reference is called **borrowing**

    fn change(some_string: &mut String) { // doesn't work with &String
        some_string.push_str(", world");
    }
    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");

    let _r1 = &mut s;
    // let r2 = &mut s; // doesn't work :
    // cannot borrow `s` as mutable more than once at a time

    // we can create multiple imutable references
    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    // let _r3 = &mut s; // but not with mutable : BIG PROBLEM

    // we cannot have a mutable reference while we have an immutable one to the same value.

    // imutable/mutable references scopes
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // end of r1/r2's scope
    let r3 = &mut s; // no problem : no mutable references in scope
    println!("{}", r3);

    // we cannot return reference to variable out of scope
    fn _no_dangle() -> String {
        let s = String::from("hello");
        // &s; // danger because s will go out of scope
        s // but we can return the value
    }


}

fn chapter_4_3() {
    let s = String::from("hello world");

    let _hello = &s[0..5]; // [start_index..stop_index] with stop_index excluded
    let _hello = &s[..5]; // also works
    let _world = &s[6..11];
    let _wolrd = &s[6..s.len()]; // same
    let _wolrd = &s[6..]; // same

    let mut s = String::from("hello world");
    let word = &s[..]; // now `s` is borrowed as immutable while `word` is in scope
    // s.clear(); // error!
    println!("the first word is: {}", word); // `word` is out of scope now
    s.clear(); // `s` is mutable again

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn chapter_5_1() {
    // defining a struct
    struct User {
        _active: bool,
        username: String,
        email: String,
        _sign_in_count: u64,
    }

    // instanciating a struct
    let mut user1 = User {
        _active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };

    // accessing values
    user1.email = String::from("anotheremail@example.com");

    // Field Init Shorthand
    fn _build_user(email: String, username: String) -> User {
        User {
            _active: true,
            username, // instead of `username: username`
            email,
            _sign_in_count: 1,
        }
    }

    // Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // fill any remaining fields using user1 values :
        // active: user1.active,
        // username: user1.username,
        // sign_in_count: user1.sign_in_count,
    };
    // struct update syntax is equivalent to `=`, so user1.username was moved into user2 and is no longer available
    // println!("user1.username={}", user1.username); // error
    println!("user1.username={}", user2.username); // ok

    // Tuple Structs Without Named Fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _color = Color(1, 2, 3);
    let _point = Point(1, 2, 3);

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn chapter_5_2() {
    #[derive(Debug)] // using outer attribute `#` to enable debugging information printing
    struct Rectangle {
        _width: u32,
        _height: u32,
    }
    let rect1 = Rectangle {
        _width: 30,
        _height: 50,
    };
    println!("rect1 is {:?}", rect1); // one line "JSON.stringify" print
    println!("rect1 is {:#?}", rect1); // multiline "JSON.stringify" print

    // dbg takes ownership, print debug infos with line number, then give back value with ownership
    let rect1 = Rectangle {
        _width: dbg!(30 * 2),
        _height: 50,
    };
    dbg!(&rect1); // creating reference here is optionnal
    dbg!(rect1); // creating reference here is optionnal
}

fn chapter_5_3() {
    // Defining Methods
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn _area(&self) -> u32 { // `&self` means `self: &Self`, which is here `self: &Rectangle`
            self.width * self.height
        }
        fn _width(&self) -> bool { // still use `&` because we don't want ownership but read only
            self.width > 0 // function can have he same name than attribute
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1._area());
    if rect1._width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // In C/C++ we have `objectPointer->method(…)` and `object.method(…)`
    // Rust automatically adds in `&`, `&mut`, or `*` so object matches the signature of the method
    rect1._width();
    (&rect1)._width(); // same
    (*(&rect1))._width(); // same

    impl Rectangle { // Multiple impl Blocks are allowed
        fn _can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1._can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1._can_hold(&rect3));

    impl Rectangle {
        fn _square(size: u32) -> Self { // method (not function) -> no self parameter
            Self {
                width: size,
                height: size,
            }
        }

        fn _new(width: u32, height: u32) -> Self { // new isn’t a special name and isn’t built into the language
            Self { width, height }
        }
    }
    let _square = Rectangle::_square(12);
    let _rect1 = Rectangle::_new(4, 8);

}


fn chapter_6_1() {
    // def enum
    enum IpAddrKind {
        V4,
        V6,
    }

    // use enum
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    // with tuple values
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let home_bis = IpAddr::V4(127, 0, 0, 2);
    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(home);
    dbg!(home_bis);
    dbg!(loopback);

    // with tuple and struct values
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let m = Message::Quit;

    // optionnal
    // enum MyOption<T> { // this is how it is defined in the standard library
    //     None,
    //     Some(T),
    // }
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

fn chapter_6_2() {
    enum Coin {
        Penny,
        Nickel,
        Dime(u32),
        Quarter { year: u32 },
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin { // must cover all possibilities
            Coin::Penny => 1,
            Coin::Nickel => { // we can also use this type of expression
                println!("Lucky nickel!");
                5
            },
            Coin::Dime(number) => { // and use tuple value
                println!("number: {}", number);
                10
            },
            Coin::Quarter { year } => { // or struct value
                println!("year: {}", year);
                25
            },
        }
    }
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime(23);
    let q = Coin::Quarter { year: 37 };
    for coin in [p, n, d, q] {
        value_in_cents(coin);
    }

    // match with option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // match all other values, and use them
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        any_other => move_player(any_other),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("move player : {}", num_spaces);
    }

    // match all other values, but don't use them
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {}

    // or if we don't want to do anything
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

#[allow(unused_assignments)]
fn chapter_6_3() {
    let config_max = Some(3u8);

    // instead of using match and do nothing if there is none
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // we can use let if
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    enum Coin {
        Penny,
        Nickel,
        Dime(u32),
        Quarter { year: u32 },
    }

    // same here with else
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter { year } => println!("State quarter from {:?}!", year), // not easy to read
        _ => count += 1,
    }

    // more easy to read
    if let Coin::Quarter { year } = coin {
        println!("State quarter from {:?}!", year);
    } else {
        count += 1;
    }
}

#[allow(unused_imports)]
fn chapter_7() {
    // crate -> binary file, can include librairies/multiple compiled rust files
    // package -> bundle of one or more crates that provides a set of functionality, contains Cargo.toml file

    // Module starts with root crate :
    //    - `src/lib.rs` for a library crate
    //    - `src/main.rs` for a binary crate
    // If a module is declared in the root crate `mod module_name;`, the associate files will be add to compilation
    //    - `src/module_name.rs`
    //    - `src/module_name/mod.rs`
    // This works with submodules declared in module_name files, the associated files are :
    //    - src/module_name/submodule.rs
    //    - src/module_name/submodule/mod.rs
    // Accessing module : `crate::module_name::submodule::SomeStruct`
    // Default access is private for parents, we cas use `pub mod` instead of `mod` to avoid this
    // To avoid using full module path we can add it to scope using `use module_path`

    // The module tree should be defined in src/lib.rs.

    // Example :
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
            fn seat_at_table() {}
        }
        mod serving {
            fn take_order() {}
            fn serve_order() {}
            fn take_payment() {}
        }
    }
    // Refer to :
    // crate
    // └── front_of_house
    //     ├── hosting
    //     │   ├── add_to_waitlist
    //     │   └── seat_at_table
    //     └── serving
    //         ├── take_order
    //         ├── serve_order
    //         └── take_payment
    front_of_house::hosting::add_to_waitlist();
    // if file exists
    // crate::front_of_house::hosting::add_to_waitlist();

    // SUPER
    // Filename: src/lib.rs
    mod why_not {
        fn deliver_order() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::deliver_order();
            }
            fn cook_order() {}
        }
    }

    // public struct / enum
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }
    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
    pub fn eat_at_restaurant_bis() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }

    // USE
    use back_of_house::Appetizer as Toto;
    pub use Toto::Salad as Sa;
    pub fn eat_at_restaurant_bis_bis() {
        let order1 = Toto::Soup;
        let order2 = Sa;
    }

    // USE with nested paths
    {
        use std::io;
        use std::io::Write;
    }
    // BECOME
    {
        use std::io::{self, Write};
    }

    // GLOBAL
    use std::collections::*;

}

fn chapter_8_1() {
    // Creating a New Vector
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v.push(5);
    v2.push(4);

    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100];
    // thread 'main' panicked at src/main.rs:835:28:
    // index out of bounds: the len is 5 but the index is 100
    let does_not_exist = v.get(100);

    // Loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // #[derive(Debug)]
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    dbg!(v);

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(row);

}

fn chapter_8_2() {
    // Creation

    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // Update
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('.'); // single character
    println!("s is {s}");

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved into s3 here and can no longer be used
    // + operator ± -> `fn add(self, s: &str) -> String`
    println!("s2, s3 : {s2} {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s1, s2, s3, s : {s1}, {s2}, {s3}, {s}");

    let text = "नमस्ते"; // 4 "letters"
    let chars = text.chars();
    let bytes = text.bytes();
    let graphemes = text.graphemes(true);
    for c in text.chars() {
        print!("{c} ");
    }
    print!("\n");
    for b in text.bytes() {
        print!("{b} ");
    }
    print!("\n");
    use unicode_segmentation::UnicodeSegmentation;
    for g in text.graphemes(true) {
        print!("{g} "); // this one is what we want
    }
    print!("\n");
}

fn chapter_8_3() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    let score = scores.get("Blue").copied().unwrap_or(0);
    let score = scores.get("Blue").unwrap_or(&0);
    let score = scores.get("Blue").unwrap();

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Blue", 25);

    // `entry` return a mutable reference to the value for the corresponding Entry key if that key exists,
    // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value
    scores.entry("Yelow").or_insert(50);
    scores.entry("Blue").or_insert(50);

    println!("{:?}", scores);
    println!("{:?}", scores.keys());

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn chapter_9() {
    // In Cargo.toml add :
    // [profile.release]
    // panic = 'abort'
    // to stop the program by default if it panics

    // panic!("crash and burn");

    // RUST_BACKTRACE=1 cargo run
    // let v = vec![1, 2, 3];
    // v[99];

    use std::fs::File;
    use std::io::ErrorKind;

    // Error or result
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // Same as:
    let greeting_file = File::open("hello.txt").unwrap();

    // Or with except:
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // Result or error => result or error
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // same using closures and the unwrap_or_else method
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    use std::io::Read;
    // Error propagation
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // Same using ? syntax
    fn read_username_from_file_shorter() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?; // if error return error else continue
        let mut username = String::new();
        username_file.read_to_string(&mut username)?; // if error return error else continue
        Ok(username)
    }
    fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    use std::fs;
    fn read_username_from_file_even_more_shorter() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    use std::error::Error;
    // if we want tu use ? in the main
    // change signature: Ok => 0, Err => not 0 like in C
    fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;

        Ok(())
    }

    // Example where we know it won't fail
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid"); // returns parse Ok value

    // Smart way to check values using type
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }
    let guess = Guess::new(50); // won't panic

}

fn chapter_10_1() {
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { // means all T having comparaison implemented
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    struct MultiPoint<T, U> {
        x: T,
        y: U,
    }
    let both_integer = MultiPoint { x: 5, y: 10 };
    let both_float = MultiPoint { x: 1.0, y: 4.0 };
    let integer_and_float = MultiPoint { x: 5, y: 4.0 };

    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p = Point { x: 1.0, y: 4.0 };
    println!("p.distance_from_origin = {}", p.distance_from_origin());

    impl<X1, Y1> MultiPoint<X1, Y1> {
        fn mixup<X2, Y2>(self, other: MultiPoint<X2, Y2>) -> MultiPoint<X1, Y2> {
            MultiPoint {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = MultiPoint { x: 5, y: 10.4 };
    let p2 = MultiPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

fn chapter_10_2() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        // We can have a default implementation
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    notify(&tweet);

    pub fn notify_generics<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    notify_generics(&tweet);

    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {} -> can be long for complex cases
    // pub fn notify<T: Summary>(item1 &T, item2 &T) {} -> shorter here

    // Multiple implementations
    use std::fmt::Display;
    use std::fmt::Debug;
    pub fn notify_multiple_impl(item: &(impl Summary + Display)) {}
    pub fn notify_multiple_generics_impl<T: Summary + Display>(item: &T) {}

    // Clearer syntax for complex cases
    // Instead of
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {42}
    // Use
    fn some_function_clearer<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        42
    }

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    println!("{}", returns_summarizable().summarize());

    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) { // only implemented if T has Display and PartialOrd trait
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

fn chapter_10_3() {
    // lifetime annotations describe a relationshib between references
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // all the references in the signature must have the same lifetime 'a
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // the lifetime of the reference returned by the longest function
    // is the same as the smaller of the lifetimes of the values
    // referred to by the function arguments
    // we’re specifying that the borrow checker should reject
    // any values that don’t adhere to these constraints

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // if we uncomment this line, the compiler tells us string2 borrowed value does not live long enough
    // println!("The longest string is {}", result);

    fn longest_bis<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_bis(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Automatic lifetime anotations

    // The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference
    // fn foo<'a>(x: &'a i32);
    // fn foo<'a, 'b>(x: &'a i32, y: &'b i32);

    // The second rule is that, if there is exactly one input lifetime parameter,
    // that lifetime is assigned to all output lifetime parameters
    // fn foo<'a>(x: &'a i32) -> &'a i32

    // The third rule is that, if there are multiple input lifetime parameters,
    // but one of them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str { // third rule apply here
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // all string literals have static lifetime because they are always in the binary
    let s: &'static str = "I have a static lifetime.";

    // Lifetime with generics
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn chapter_11() {
    // cargo new adder --lib
    // mesure performance : https://doc.rust-lang.org/unstable-book/library-features/test.html

    // no multithreading
    // cargo test -- --test-threads=1

    // show standard output
    // cargo test -- --show-output

    // run specific tests
    // cargo test keyword_in_function_name

    // run ignored tests only
    // cargo test -- --ignored

    // run all tests including ignored
    // cargo test -- --include-ignored

    // run only integration tests
    // cargo test --test integration_test
}

fn chapter_12() {
    // cargo new minigrep
    // run our program: cargo run -- searchstring example-filename.txt

    // use case_sensitive or not
    // IGNORE_CASE=1 cargo run -- to poem.txt
    // IGNORE_CASE=0 cargo run -- to poem.txt
}

fn main() -> () {
    let still_testing = false;
    if still_testing {
        chapter_1();
        chapter_2();
        chapter_3_1();
        chapter_3_2();
        chapter_3_3();
        chapter_3_4();
        chapter_3_5();
        chapter_4_1();
        chapter_4_2();
        chapter_4_3();
        chapter_5_1();
        chapter_5_2();
        chapter_5_3();
        chapter_6_1();
        chapter_6_2();
        chapter_6_3();
        chapter_7();
        chapter_8_1();
        chapter_8_2();
        chapter_8_3();
        chapter_9();
        chapter_10_1();
        chapter_10_2();
        chapter_10_3();
        chapter_11();
        chapter_12();
    } else {
    }

    // todo mesure performance (see chapter 11)
}
