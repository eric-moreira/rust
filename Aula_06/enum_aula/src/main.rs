enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8 }
}
fn main() {
    let cor = Color::CymkColor{cyan: 10, magenta: 100, yellow: 100, black: 0};

    println!("Cor; {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0,0,0) => "preto",
        Color::RgbColor(_,_,_) => "RGB desconhecido",
        Color::CymkColor{cyan: 10, magenta: 100, yellow: 100, black: 0} => "Uma cor CYMK qualquer",
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "Qualquer coisa CYMK"
    }
)
}
