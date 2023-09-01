fn sub(a:i32,b:i32)->i32{a-b}
fn add(a:i32,b:i32)->i32{a+b}
fn mul(a:i32,b:i32)->i32{a*b}
fn div(a:i32,b:i32)->i32{a/b}
fn rem(a:i32,b:i32)->i32{a%b}


fn display(res:i32){println!("{:?}",res);}

fn main(){
    let a:i32 = 12;
    let b:i32 = 3;
    display(sub(a,b));
    display(add(a,b));
    display(mul(a,b));
    display(div(a,b));
    display(rem(a,b));


}