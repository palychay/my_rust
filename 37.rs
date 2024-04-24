fn main(){
 
    let sum: fn(i32, i32) -> i32  = |a, b| a + b;
     
    do_operation(6, 4, sum);
    do_operation(6, 4, multiply);

    let operation1 = chose_operation(1);
    operation1(5, 4);
     
    let operation2 = chose_operation(2);
    operation2(5, 4);
     
    let operation3 = chose_operation(3);
    operation3(5, 4);
}

fn do_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32){
     
    let result = operation(a, b);
    println!("result: {}", result);
}

fn chose_operation(n:i32) -> fn(i32, i32){
    match n{
        1 => |a, b|{println!("{}", a + b);},
        2 => multiply1,
        _ => |a, b|{println!("a={}  b={}", a, b);},
    }
}

fn multiply(a: i32, b: i32) -> i32{
    a * b
}

fn multiply1(a: i32, b: i32){
    println!("{}", a * b);
}