fn main() {
    let s1 = String::from("hello");

    let mut s2 = String::from("hey");

    let len = calculate_length(&s1);

    let s3 = change (&mut s2);

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize { //s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of
 // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
