fn main(){
    let x = 5;
    let y = 7;
    let result: bool;
    result = x == y; // result = false - x не равно y

    let x = 5;
    let y = 7;
    let result: bool;
    result = x != y; // result = true

    let x = 5;
    let y = 7;
    let result: bool = x > y; // result = false

    let x = 5;
    let y = 7;
    let result = x < y; // result = true

    let x = true;
    let y = false;
    let mut result = !x; // result = false
    result = !y; // result = true

    let x = true;
    let y = false;
    let mut result = x && y; // result = false
    println!("result = {}", result);
        
    result = 10 > 2 && 4 < 5; // result = true (10> 2 равно true и 4 < 5 равно true)
    println!("result = {}", result);

    let x = true;
    let y = false;
    let mut result = x || y; // result = true
    println!("result = {}", result);
    
    result = 2 > 10 || 5 < 4; // result = false (2 > 10 равно false и 5 < 4 равно false)
    println!("result = {}", result);
}