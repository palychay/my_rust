fn main(){
    let message = "Hello Rust"; // переменная - внешнее окружение замыкания
     
    let hello =||{              // определение замыкания
        println!("{}", message);
    };
     
    hello();    // вызов замыкания


    let mut n = 5;
    let mut increase =||{
        n = n + 1;
        println!("n = {}", n);
    };
    increase();
    increase();
    increase();
     
    println!("Переменная n в функции main равна {}", n);
}