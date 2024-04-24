fn main(){
    let square = |n: i32|{
        let result = n * n;
        println!("Квадрат числа {} равен {}", n, result);
    };
    square(4);
    square(5);
    square(6);

    let sum = |a: i32, b: i32| -> i32{
        a + b
    };
    let sum_of_5_and_4 = sum(5, 4);
    println!("Сумма 5 и 4 равна: {}", sum_of_5_and_4);

    let hello =||{          // определение анонимной функции
        println!("hello");
    };
    hello();    // вызов анонимной функции


    let sum1 = |a, b|{   // типы параметров и результата не указаны
        a + b
    };
    let sum_of_5_and_4 = sum1(5, 4);
    println!("Сумма 5 и 4 равна: {}", sum_of_5_and_4);


    let sum2 = |a, b| a + b;
    let sum_of_5_and_4 = sum2(5, 4);
    println!("Сумма 5 и 4 равна: {}", sum_of_5_and_4);
}