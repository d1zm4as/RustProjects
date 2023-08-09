use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

// porta maxima
const MAX: u16 = 65535;

// endereço de fallback.
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// argumentos da cli.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    // aceita -a ou --adress como argumento
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    /// o endereço para o processo de sniff, tem que ser um ipv4 valido
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    /// a porta inicial do sniff
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    /// o valor final da porta, tem quer menor ou igual a const max
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}


// Scaneia a porta
async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    // tenta a conexaõ com o endereço e porta
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        // se for sucesso (caso a porta esteja aberta), vai printar um .
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        // caso contrario a porta esta fechada
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    // lendo os argumentos
    let opts = arguments().run();
    // iniciando o canal
    let (tx, rx) = channel();
    // interando sobre todas as portas e criando uma task para cada.
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move { scan(tx, i, opts.address).await });
    }
    // vetor de outputs
    let mut out = vec![];
    
    drop(tx);
    // aguarda todos os scans acabarem e coloca no vetor out.

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        // printando os outputs(portas abertas)
        println!("{} is open", v);
    }
}