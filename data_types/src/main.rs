use std::io;

fn main() {
    //data types
    // integer
    /*let x =2 ;
    println! ("x is {}", x);
    let x : u8 = 255;
    let y : u32 = 256;
    println! ("x is {0} and y is {1}", x, y);
    // float

    let z = 2.0;
    */

    //tuple
    /*let tup : (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println! ("the value of y is {}", y); //6.4

    let x0 = tup.0;

    let x1 = tup.1;

    let x2 = tup.2; //1

    println! ("the value of z is {}", x2);
    */

    //array
    /*let a = [1,2,3,4,5];

    let b: [i32; 5] = [1,2,3,4,5];

    let c = [3; 5];

    println!("{}",c[0])
    */
    let a = [1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fail to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!(
            "The value of the element at index {} is: {}",
        index, element
    );

}
