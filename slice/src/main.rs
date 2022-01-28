fn main() {
    /*
    let mut s = String::from("Hello, world!");

    let word = first_word(&s);

    s.clear();
}

fn first_word (s: &String) -> usize {
    let bytes = s.as_bytes(); // convert String to an array of bytes using as_bytes

    for (i, &item) in bytes.iter().enumerate() { //iterator over the array of bytes using iter method
        if item == b' ' {
            return i;
        }
    }

    s.len()

    let s  = String::from("hello world");

    let hello = &s[..5]; //drop 0
    let world = &s[6..]; //drop the last to get the last index

    println!("{}", hello);
    */
    let s = String::from("Hello, world!");

    let word = first_word(&s);

    //s.clear();

    println!("the first word is {}", word);
}
fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes(); // convert String to an array of bytes using as_bytes

    for (i, &item) in bytes.iter().enumerate() { //iterator over the array of bytes using iter method
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
