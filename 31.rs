fn main(){
    square(5);
    square(6);
    square(4);
    show_user("Tom", 36);
    square1(45);
}
 
fn square(n: i32)
{
    let result = n * n;
    println!("Квадрат числа {} равен {}", n, result);
}

fn show_user(name: &str, age: i32)
{
    println!("Информация о пользователе");
    println!("Имя: {}", name);
    println!("Возраст: {}", age);
}

fn square1(mut n: i32)
{
    n = n * n;
    println!("Квадрат числа равен {}", n);
}