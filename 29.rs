fn main(){
    let mut n = 1;
    loop{
        println!("n = {}", n);
        n = n + 1;
        if n == 10{
            break;
        }
    }
    println!("Конец программы");


    let mut num = 1;
    let result = loop
    {
        if num == 4 { break num * 2;}
        num = num + 1;
    };
    println!("result = {}", result);


    let mut n = 1;
    while n < 10
    {
        println!("n = {}", n);
        n = n + 1;
    }
    println!("Конец программы");


    for num in 1..6
    {
        println!("num = {}", num);
    }
    println!("Конец программы");  


    let mut i = 1;
    let mut j = 1;
    while i < 10
    {
        while j < 10
        {
            print!("{}\t", i * j);
            j = j + 1;
        }
        println!();
        i = i + 1;
        j = 1;
    }


    for i in 1..10
    {
        for j in 1..10
        {
            print!("{}\t", i * j);
        }
        println!();
    }
}