const NUM4 : i32 = 225;

fn main(){
    const PI : f32 = 3.14;
    println!("PI = {}", PI);

    const NUM : i32 = if true { 3 } else { 5 };
    println!("NUM = {}", NUM);  // 3 
}