#![allow(dead_code,unused_variables)]
use std::future::Future;

fn main() {
    println!("Hello, world!");
    let y = foo();
    // let x:usize  = foo2().await();

    // a variavel x ainda não recebe um valor do tipo usize
    // a variavel recebe uma promessa dque um dia ira receber um valor com esse tipo
    // o metodo await significa diz ao compilador "Nao faça o resto do codigo ate a variavel receber o valor prometido"
}


async fn foo() -> usize{  
    println!("foo");
    0


}


// a funçao foo == foo1()

fn foo1()-> impl Future<Output = usize>{
    async {
    println!("foo");
        
        0
    }
}