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

    let a: f32 = 2.04;
    println!("a: {}", a);
    let b: f64  = 100.080973;
    println!("b: {}", b);
    let c  = -45.78;            // тип f64
    println!("c: {}", c);

    let a: bool = true;
    println!("a: {}", a);
    let b = false;
    println!("b: {}", b);

    let a: char = 'a';
    println!("a: {}", a);
    let b = 'b';        // тип char
    println!("b: {}", b);

    let a: &str = "hello";
    println!("a: {}", a);
    let b = "Rust";     // тип &str
    println!("b: {}", b);

    let hello_world: String = String::from("Hello World!");
    // можно без ": &str", let hello_world = String::from("Hello World!");
    println!("{}", hello_world);

    //или если надо просто пустую строку:
    let empty_string: String = String::new();

    let mut number = 12;
    number /= 5;
    println!("{}", number);
}