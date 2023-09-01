enum Acess{
    Admin, 
    Manager,
    User,
    Guest,
}


fn main(){
    let acess = Acess::Guest;
    let can = match acess{
        Acess::Admin => true,
        _=>false,
    };
    println!("{}",can);
}


