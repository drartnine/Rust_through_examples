use std::io;


fn main() {
    println!("Factorial calculation");

    loop {
        println!("Please input a number for factorial calculation. Input a letter to terminate the program!");

        let mut fact = String::new();

        io::stdin()
            .read_line(&mut fact)
            .expect("Failed to real line");

            let fact: i64 = match fact.trim().parse() {
                Ok(num) => num,
                Err(_) =>
                {
                    println!("You did not enter a number! Program terminated!");
                    break
                },
            };
            let mut i = 1;
            let mut total = 1;
            if fact < 0 {
                println!("Please enter a number larger or equal to 0");
            } else {
            while i <= fact {
                total *= i;
                i += 1;
            };
            println!("{} factorial equals {}",fact, total);
        };
        }
}
