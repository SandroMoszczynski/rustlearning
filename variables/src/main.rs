// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
//     let spaces = "   ";
//     let spaces = spaces.len();
//     spaces = spaces.len();
// }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
/// ```
///     let index: usize = index
///         .trim()
///         .parse()
///         .expect("Index entered was not a number");
///

///     let element = match a.get(index) {
///         Some(val) => val,
///         None => {println!("Index out of range!"); return;}
///     };

///     println!("The value of the element at index {index} is: {element}");
/// }
/// ```

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     //let x = x + 1;
//     //return x;

//     //x + 1

//     return x + 1
// }

// fn main() {
//     let s1 = String::from("hello");
//     let test = 5;

//     let len = calculate_length(&s1);
//     let squre= square(test);

//     println!("The length of '{s1}' is {len}.");
//     println!("{squre}")

// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn square(i: u32) -> u32 { i * i}

// fn main() {
//     let mut s = String::from("hello");

//     //{
//         //change(&mut s);
//     //}

//     //println!("{s}");

//     //let s2 = &mut s;

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{r1}, {r2}, and {r3}");

// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}")
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}