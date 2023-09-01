/*
Enumeration 

uma informação que pode ser uma de multiplas possibiliadades, cada possibilidade é chamada de variante, fornece informação sobre o seu programa para o compilador

Gera programas mais robustos quando usado junto com o match


*/
enum Colors{
    Blue,
    Red,
    Black,
}

fn print_color(color:Colors){
    let go = Colors::Black;
    match go{
        Colors::Blue =>println!("go Blue"),
        Colors::Red =>println!("go Red"),
        Colors::Black =>println!("go Black"),
       
        _=> println!("just go wtf")
    }
}

fn main(){
    print_color(Colors::Blue);   
}