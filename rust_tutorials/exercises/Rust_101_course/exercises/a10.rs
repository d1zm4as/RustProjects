
fn coordinates () -> (i32,i32){

    (3,5)

}

fn display(){
    let coord = coordinates();
    println!("{},{}",coord.0,coord.1);
    let y = coord.1;
    if y > 5{
        println!(">5");
    }else if y < 5{
        println!("<5");
    }else{
        println!("=5");

    }
    
}

fn main(){

    display();
    




    // let (name,age)= ("Cleiton",157);
    

}