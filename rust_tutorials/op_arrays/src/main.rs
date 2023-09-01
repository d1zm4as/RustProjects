fn sum(array:[i32;10])->i32{let mut soma:i32 = 0; for x  in array{ soma+=x;}return soma;}
fn max(array:[i32;10])->i32{let mut maior:i32 = array[0]; for x  in array{if x > maior{maior=x;}}return maior;}   
fn min(array:[i32;10])->i32{let mut menor:i32 = array[0]; for x  in array{if x < menor{menor=x;}}return menor;}   









fn main() {
    let array = [-1,2,-3,4,0,10,3,9,5,8];
    

    println!("A soma do array é :{}",sum(array));
    println!("O máximo do array é :{}",max(array));
    println!("O mínimo do array é :{}",min(array));
    
}
