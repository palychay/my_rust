fn main(){

    let age: u32; age = 36;
    let age1: u32 = 30;
    let age2 = 36;
    let name = "Tom";

    let mut age3 = 36;
    println!("Начальное значение: {}", age3);
    age3 = 25;
    println!("Конечное значение: {}", age3);

    let number = 10;
    println!("number = {}", number);
    let number = 15;
    println!("number = {}", number);
    let number = 254;
    println!("number = {}", number);

    println!("Name = {}  Age = {}", name, age);
    println!("Age = {}", age);
    println!("Hello, world");

    let a: i8  = 10;
    println!("a: {}", a);
    let b: u16  = 1000;
    println!("b: {}", b);
    let c: isize  = 1234;
    println!("c: {}", c);

    let a = 10; // переменная a имеет тип i32
    let a = 0b0101;     // 5 в десятичной системе
    let a = 0o11;       // 9 в десятичной системе
    let a = 0xA1;       // 161 в десятичной системе
}