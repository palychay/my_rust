fn main(){
    let a = 0b0010 << 2;
    let b = 0b10000 >> 3;
    println!("a = {}  b= {}", a, b);

    let mut x = 8;
    println!("x = {}", x);
     
    x <<= 2;  // 8 в двоичной системе 1000, 
            // после сдвига на 2 разряда вправо 10000 или 32 в десятичной системе
    println!("x = {}", x);  // 32
     
    x >>= 3;  // 32 в двоичной системе 100000, 
            // после сдвига на 3 разряда вправо 100 или 4 в десятичной системе
    println!("x = {}", x);  // 4

    let a = 5 | 2;          // 101 | 010 = 111  - 7
    let b = 6 & 2;          // 110 & 010 = 10  - 2
    let c = 5 ^ 2;          // 101 ^ 010 = 111 - 7

    let a = 0b101 | 0b010;
    let b = 0b110 & 0b010;
    let c = 0b101 ^ 0b010;
}