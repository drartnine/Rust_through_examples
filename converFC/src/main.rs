use std::io;

fn main() {
    println!("Convert Fahrenheit and Celcius");

    loop {
        println!("Please input temperature in Fahrenheit. Input a letter to terminate the program");

        let mut fah = String::new();

        io::stdin()
            .read_line(&mut fah)
            .expect("Failed to read line");

            let fah: f32 = match fah.trim().parse() {
                Ok(num) => num,
                Err(_) =>
                {
                    println!("You did not enter a number! Program terminated!");
                    break},
            };

            let cel = (fah - 32.0)/1.80;

            println!("{} Fahrenheit is {:.2} Celcius", fah, cel);

    }

}
