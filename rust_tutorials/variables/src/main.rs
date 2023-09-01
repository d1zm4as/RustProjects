
const VALOR_REAL_DA_QUEIJADA:f32 = 3.40;
fn main() {
    /*
    Variável é uma maneira de alocar temporiariamente um espaço na memória, facilitando, para os programadors, o trabalho e a manipulação de memória




    Por padrão(e por segurança) as variáveis em Rust são imutáveis, devem ser criadas com o let 
    
    */
    
    // let x = 5;
    // println!("{x}");
    // x = 6;
    // println!("{x}");
    /*
    se o código acima for compilado, resultará em erro, pois damos dois valores a um variável imutável
    
    entretanto ainda é possível tornar as variáveis mutáveis
     */

    // let mut x = 5;
    // println!("{x}");
    
    // x = 3;
    // println!("{x}");
     /*
    Q1->which statemente best describes what it means if a variable x is immutable?


    R -> x cannot be changed after being assigned a value
    
    Q2 -> which keyword used after let indicats that a variable can be mutated?

    R-> mut

    Q3 -> it compiles or not?
    let y = 5;
    println!("{y}");
    
    y += 3;
    println!("{y}");

    R-> nop, you cant change a immutable variable

     */
    
    
    // constantes
    // são valores que estão "amarrados" a um nome e uma variável e não são permitidos de trocar.
    // há algumas diferenças entre const e let
    // não é possível usar mut em const, porque const são SEMPRE imutáveis
    // e o tipo do valor deveer ser anotado
    // constantes podem ser declaradas em qualquer escopo
    // dever ser usadas em casos de expressões, não decorrentes de variáveis terceiras
    
    //const QUEIJADA:u32 = 4;     
    //bom exemplo de declaração de constante
    println!("Quanto é a queijada gld?");
    let joao = 1;
    let chico = 3;
    //const CINTIA :u32 = joao+chico;
    // forma errada
    let cintia :u32 = joao+chico;
    
    println!("João: Eu acho que é {joao} ");
    println!("Chico: Eu acho que é {chico} ");
    println!("Cintia: O valor exato da QUEIJADA pode ser obitido pela soma exata das perspectivas estimidades de Jão e de Chico, no caso {cintia} ");

    println!("Mlk lá: O valor Real da QUEIJADA é R$ {VALOR_REAL_DA_QUEIJADA}");
    












}   
