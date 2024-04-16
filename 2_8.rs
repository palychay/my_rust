fn main(){
    let num = 23;
    match num
    {
        1=>println! ("один"),
        2=>println! ("два"),
        3=>println! ("три"),
        _=>println! ("непонятно")
    }

    let num = 2;
    match num
    {
        1=>println! ("один"),
        2=>println! ("два"),
        3=>println! ("три"),
        _=>println! ("непонятно")
    }

    let num = 3;
    let result = match num
    {
        1=> "один",
        2=> "два",
        3=> "три",
        _=> "непонятно"
    };
    println!("result = {}", result);    // result = три 

    let num = "жаба скрипт";
    let result = match num
    {
        "жаба скрипт" => "фронтэнд",
        _=> "непонятно"
    };
    println!("result = {}", result);
}