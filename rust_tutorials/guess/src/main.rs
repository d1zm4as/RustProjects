use rand::Rng;// define os métodos rand que serão usados 
use std::cmp::Ordering;
use std::io;// para obter o input do usuário e depois printar o output é necessário trazer a biblioteca io, que vem na biblioteca std:
fn main() {
    println!("Guess game");
    
    
    let sec = rand::thread_rng().gen_range(1..=10);
    loop{    
        println!("Digite um número: ");
    //rand::thread_rng() ,função que nos dá um gerador de numeros random
    //.gen_range(1..=100) , determinando os números que serão considerados.
    /* 
        .read_line(&mut guess)
        //pegando o input, pega qualquer coisa inputada e colaca numa string essa precisa ser mutável, caso contrário não conseguiremos mudar seu conteúdo
        .expect("Erro!");// tratando possíveis erros
    */
    //io::stdin().read_line(&mut guess).expect("Erro!");
    // numa linha só
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Erro!");// se não for colocado vai receber um aviso do compilador
    
    let guess:u32 = match guess.trim().parse(){
        Ok(num)=> num,
        Err(_)=>continue,
    // comparando o num inputado com o random
    // trim->elimina todo espaço em branco
    // parse-> converte a string para outro type de dados
    //:u32 diz pro rust qual tipo é desejado
    };
    println!("Você inseriu: {guess}");
    match guess.cmp(&sec) {
        // cmp, compara dois valores e pode ser chamada em qualquer coisa que possa ser comparada
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("you win!");
            break;
        }
    }
    }



    
    
    // para gerar numéros aleatórios,é preciso adicionar o crate
    // no Cargo.toml,abaixo de dependências, rand não faz parte da biblioteca padrão
    // cargo update,para atualizar os crates do projeto
}
