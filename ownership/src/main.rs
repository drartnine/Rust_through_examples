fn main() {
    let s = String::from("hello"); //s comes into scope

    takes_ownership(s);  //s's value move into a function
                        //... and so is no longer valid here
    let x = 5;          // x comes into scope

    makes_copy(x);      // x would move into the function,
                        // but i32 is Copy, so it's ok to still
                        // use x afterward

    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s,length)
}
