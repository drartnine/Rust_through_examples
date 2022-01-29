fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false); // false
    println!("true OR false is {}", true || false); // true
    println!("NOT true is {}", !true); //false

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); //b is binary - 0011 & 0101 = 0b0011u32
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // 0011 | 0101 = 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0011 ^ 0101 = 0110
    println!("1 << 5 is {}", 1u32 << 5); // << left shift 5 binary so 1 shift 5 would be 10000 = 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // >> right shifts 2 in hexadecimal to 0x20

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32); // _ is for easy readability
}
