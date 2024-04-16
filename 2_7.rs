fn main(){
    let number = 5;
 
    if number == 3 
    {
        println!("number равно 3");
    }
    else if number == 4
    {
        println!("number равно 4");
    }
    else if number == 5
    {
        println!("number равно 5");
    }
    else
    {
        println!("значение number не известно");
    }

    let condition = true;
    let number = if condition { 4 } 
                 else {  5 };
    println!("number = {}", number);    // number = 4

    let a = 5;
    let b = 2;
    let operation = 2;
 
    let number = if operation == 1 { a + b } 
                else if operation == 2 { a - b }
                else {  a * b };
    println!("number = {}", number);    // number = 3
}