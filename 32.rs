fn main(){
     
    let num1 = sum(2, 3);   // 5
    let num2 = sum(5, 6);   // 11
    println!("num1={}  num2={}", num1, num2);

    let result1 = check_age(18);
    println!("result1={}", result1);
    
    let result2 = check_age(127);
    println!("result2={}", result2);

    let num1 = sum1(2, 3);       // 5
    println!("num1={}", num1);
}

fn sum(a: i32, b: i32) -> i32    // установка типа возвращаемого значения
{
    a + b       // возвращение значения
}

fn check_age(age: u8) -> bool
{
    println!("В функцию chek_age передано значение: {}", age);
    age <= 110
}

fn sum1(a: i32, b: i32) -> i32
{
    return a + b;   // возвращение значение
}