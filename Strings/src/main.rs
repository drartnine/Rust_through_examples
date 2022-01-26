fn main() {
    let company:&str = "TutorialsPoint";
    let location:&str = "Hong Kong";
    println!("company is : {} location {}", company, location);

    let company:&'static str = "1301B";
    let location:&'static str = "HK";
    println!("company is : {} location {}", company, location);

    //String object
    let mut empty_string = String::new();
    println!("length is {}", empty_string.len());

    let mut content_string = String::from("Yeung Kin Man");
    println!("length is {}", content_string.len());

    empty_string.push_str("hello");
    println!("{} ", empty_string);

    let replace_string = content_string.replace("Man","Leung               ");
    println!("{}", replace_string);

    print_literal(replace_string.as_str());

    println!("{} has length of {}",replace_string, replace_string.len());
    println!("{} has length of {}",replace_string.trim(), replace_string.trim().len());

    let msg = "Splitting strings into different words with whitespace".to_string();
    let mut i = 1;

    for words in msg.split_whitespace() {
        println!("token {} {}", i, words);
        i+=1;
    }

    let fullname = "White, Space, Wash, To, Day".to_string();
    for token in fullname.split(",") {
        println!("token is {}", token);
    }

    for n in fullname.chars() {
        println!("{}", n);
    }

    println!("{}", fullname + " " + &msg);

}

fn print_literal(data:&str) {
    println!("{} AC1 {}", data, data);
}
