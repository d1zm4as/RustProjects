fn main(){


    first();
    last();
    let x:i32 = add(1,1);

    let y:i32 = add(x,3);

    let z:i32 = add(y,x);

    println!("{}",x);
    println!("{}",y);
    println!("{}",z);

}

fn first(){print!("Chico ");}
fn last(){println!("Das cocadas");}

fn add(a:i32,b:i32)->i32{

    a+b
}

