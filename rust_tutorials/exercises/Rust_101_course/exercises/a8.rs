/*
Structure 

Um tipo que contem vários tipos de informação



*/
//ex

enum Flavor{
    Uva,
    Laranja,
    Goiaba

}
struct Drink{
    sabor:Flavor,
    preco:f64,
}

fn print_flavor(drink:Drink){
    match drink.sabor{
        Flavor::Uva =>println!("go Uva"),
        Flavor::Laranja =>println!("go Laranja"),
        Flavor::Goiaba =>println!("go Goiaba"),
    }
    println!("{}",drink.preco);
}

fn main(){
    let toma  = Drink{
        sabor: Flavor::Goiaba,
        preco:3.50
    };

    print_flavor(toma)


}
