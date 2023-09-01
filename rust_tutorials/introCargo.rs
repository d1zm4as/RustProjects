fn main(){



    println!("Hello,World!");
    /*
    Cargo é o sistema de gerenciamento de pacotes do Rust, a maioria dos programadores Ruts usa a ferramenta para gerenciar seus projetos Rust, porque Cargo cuida de muitas tarefas pra você
    Como construir código, baixar bibliotecas que o seu código depende(famosas depêndencias)

    cargo --version, para checar se o cargo está instalado.
    
    cargo new hello_cargo,cria um novo diretório e projeto chamado hello_cargo

    dois arquivos foram gerados-> 
        cargo.toml -> toml(toms obvious minimal language) que o formato configurado, no arquivo é listado o nome do projeto,versão  e a edição.
        As dependências também são listadas no fim do arquivo
        e um diretorio src -> dentro do diretório haveŕa um arquivo main.rs
        que terá o mesmo código da intro.

        No Rust os pacotes de código são referenciados como crates(caixotes)

    Pode-se perceber que o Cargo espera que o nosso código esteja dentro do diretório src

    Usar o cargo ajuda a organizar o projeto

    Rodando um projeto Cargo

    cargo build, rodar esse comando pela primeira vez cria um novo arquivo,Cargo.lock,esse arquivo mantem o controle das versões exatas de cada dependência do projeto

    o Cargo coloca os binários num diretório target

    também é possível usar cargo run para compilar o projeto

    o comando cargo check apenas compila o código mas não gera o executável

    recapitulando ->
    criar projeto -> cargo new
    compilar projeto->cargo build
    compilar e executar -> cargo run
    compilar sem produzir binários-> cargo check

    Uma das vantagens de usar os comandos cargo é que os comandos são os mesmo não importa a plataforma.

    Quando o projeto estiver pronto para ser publicado,usamos o
    cargo build --release, para compilar com otimizações.


    Quanto mais complexo o projeto mais o cargo se torna importante




    */
    
}