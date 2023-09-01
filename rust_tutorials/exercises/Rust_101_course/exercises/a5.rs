// fn fib(n:i32)->i32{
//    let (mut a,mut b) = (0,1);
//    let mut cont = 0;
//    while cont<n{
//        (a,b) = (b,a+b);
//        cont+=1;
//    }
//    a
// }




fn main(){
   let mut cont:i32 = 1;
   loop{
      println!("{}",cont);
      
      cont+=1;
      if cont>4{
         break;
      }
   }
} 