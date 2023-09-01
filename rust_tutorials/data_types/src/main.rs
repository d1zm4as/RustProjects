fn main() {
    println!("Hello, world!");
   /*
   A memória so guarda a informação em binário, qualquer coisa pode ser representada em binário, o programa determina o que os binários representam.
    
    Os tipo básicos já universais são providenciados pela linguagem
   Signed integer, representa inteiros positivos e negativos
   Unsigned integer, representa apenas inteiros positivos  
   
    */
    // addition
    // let sum = 5 + 10;
    
    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Results in 0

    // // remainder
    // let remainder = 43 % 5;
    // let t = true;

    // let f: bool = false;

    // o tipo tupla
    // tem um tamanho fixo, mas pode ter vários tipos
    //let tup: (i32, f64, u8) = (500, 6.4, 1); 
    
    //let (x,y,z)=tup;
    let a = 24%5;
    let lol = (7,3.14,0);
    println!("{}",lol.0);
    println!("{}",lol.1);
    println!("{}",lol.2);
    //o tipo array, tem um tamanho fixo, aceita apenas um tipo
    let arr = [1,2,3,4,5];

    
    println!("{}",arr[0]);
    println!("{}",a);

}
