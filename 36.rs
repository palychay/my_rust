fn main(){
    let display: fn() = ||{println!("hello");};
    display();

    let display: fn(&str) = message;
    display("Hello Rust!");

    let operation: fn(i32, i32) -> i32 = multiply;
    let result = operation(5, 6);
    println!("Result: {}", result); // Result: 30
}

fn message(text:&str){
    println!("{}", text);
}

fn multiply(a: i32, b: i32) -> i32{
    a * b
}