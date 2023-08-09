#![allow(unused)]
use clap::Parser;


//procure por um padrao no arquivo  e mostre as linhas que contem o padrao
#[derive(Parser)]
struct Cli {
    pattern: String,// o padrao a ser procurado
    path: std::path::PathBuf,//o caminho ate o arquivo a ser lido
}





fn main() {
    let args = Cli::parse();
    println!("{:?}",args);
}
