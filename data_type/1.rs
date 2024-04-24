fn main(){
    let user: (&str, u8, f32) = ("Tom", 36, 1.78);
    println!("Имя: {}", user.0);    
    println!("Возраст: {}", user.1);
    println!("Рост: {}", user.2);

    let mut user: (&str, u8, f32) = ("Tom", 36, 1.78);
    user.0 = "Bob";
    println!("Имя: {}", user.0);    // Bob

    let user = ("Tom", 36, 1.78);
    // декомпозиция кортежа на переменные
    let (name, age, height) = user;
    println!("Имя: {}", name);  
    println!("Возраст: {}", age);
    println!("Рост: {}", height);

   // let (name, age, _) = user; Нам обязательно надо указать столько же переменных, сколько значений в кортеже. 
   //Однако если какое-то значение нам не нужно, мы можем вместо соответствующей переменной указать прочерк

   let user = ("Tom", 36);
     
    if ("Tom", 36) == user {
        println!("name: Tom, age: 36");
    }


    let tom = ("Tom", 36);
    display(tom);

    let (x, y) = get_default_point();
    println!("x: {}  y:{}", x, y);
}

fn display((name, age):(&str, i32)){
     
    println!("name: {}  age:{}", name, age);
}

fn get_default_point() -> (i32, i32){
    (4, 25)
}