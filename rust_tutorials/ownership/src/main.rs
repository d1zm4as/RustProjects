

fn add_sufix(mut name: String)->String{name.push_str(" JR.");name}



fn main() {
    // o que é ownership?
    // segurança é a absência de comportamento indefinido
    // Um objetivo fundamental do Rust é garantir que nenhum programa tenha um comportamento indefinido
    /*
    Um objetivo secundario do Rust é previnir os erros no run-time, o compilador tenta previnir esses erros no compile time, o que melhora confiança, peformance e segurançada aplicação
    
    Focando no ponto da operações com memoria;

    memoria é um espaço onde a informação está armazenada durante a execução
    
    Copiar valores não é recomendado(principalmente se for usar o valor fora da função de origem ), porque cria duas variáveis na stack com o mesmo valor,se possível usar 
    box::new([]), para guardar os dados na memória heap, essa poderá ser usada dentro e fora da função
    
    O Rust não permite controle manual de memoria, porque esse tipo de liberdade facialmente se transforma em bugs

     */
    let first = String::from("Pedro");
    //obs: as variáveis não podem ser usadas depois de serem movidas. Mas isso pode consertado usando .clone()
    let full = add_sufix(first);
    println!("{full}");


}