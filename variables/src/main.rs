fn main() {
    //variables
    /*
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    */

    //shadowing
    // shadowing variable x with x + 1 and then inside the loop x * 2;
    // must use let to shadow variable
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x); //12
    }

    println!("The value of x outside is : {}", x); //6
    //change the type from string type to number type
    let spaces = "    ";
    let spaces = spaces.len(); //4
    println! ("There are {} spaces", spaces);
}
