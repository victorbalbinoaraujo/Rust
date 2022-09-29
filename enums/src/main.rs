enum EnderecoIP {
    V4(u8, u8, u8, u8, u8),
    V6(String),
}

enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

impl Mensagem {
    fn invocar(&self) {}
}

fn main() {
    let local: EnderecoIP = EnderecoIP::V4(127, 0, 0, 0, 1);
    let loopback: EnderecoIP = EnderecoIP::V6(String::from("::1"));

    let msg: Mensagem = Mensagem::Escrever(String::from("Olá"));
    msg.invocar();
}
