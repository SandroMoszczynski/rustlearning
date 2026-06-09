fn main() {
    let s = String::from("Hello, world!");
    let s_lit = "Hello, world!";
    let first = first_word(&s);
    println!("{first}");

    let second = second_word(&s);
    println!("{second}");
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut first_break: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_break == 0 {
            first_break = i;
        }
        else if item == b' ' || i == s.len() -1 {
            return &s[first_break..i];
        }
    }

    &s[..]
}