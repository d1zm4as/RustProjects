#![allow(unused)]
use clap::Parser;
use std::io::BufReader;

//procure por um padrao no arquivo  e mostre as linhas que contem o padrao
#[derive(Parser)]
struct Cli {
    pattern: String,// o padrao a ser procurado
    path: std::path::PathBuf,//o caminho ate o arquivo a ser lido
}





fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string("test.txt");
    // a função read_to_String não retorna uma string, retorna um resultado, que contem ou uma string ou um erro
    // como result é um enum, a implementação com o result 
    let content = match result {
        Ok(content) => { content }
        Err(error) => { println!("cant deal with {}, the program will exit", error); }
    };
    println!("file content: {}",content);









}
