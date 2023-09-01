fn main() {  
    let x = soma(3,4);
    let y = soma(5,2);
    let z = soma(10,x);

    println!("{x}");
    println!("{y}");
    println!("{z}");



}

/*
statementes -> são instruções que perfomam alguma ação e não retornam valor

Expressions -> são instruções que performa alguma coisa e retorna valor

 */
// o rust não se importa aonde você declara as suas funções, se eles estão definidas em algum lugar o copilador vai resolver 


// parâmetros-> são variáveis especiais que são parte de função
// quando a função tem parâmetros, você podera passar valores concretos

fn soma(a:i32,b:i32)->i32{a+b}
    