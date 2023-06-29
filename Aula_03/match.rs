fn main(){
    let linguagem = "Rust";

    let proposito = match linguagem {
        "PHP" => "WEB",
        "C" => "Kernel",
        "Rust" => "Melhor que C",
        &_ => "Nenhuma linguagem"
    };

    println!("O proposito de {} é: {}", linguagem, proposito);

    let number = 42;

    match number {
        0 => println!("Zero"),
        1..=9 => println!("Single digit"),
        n if n % 2 == 0 => println!("Even number"),
        _ => println!("Other number"),
    }
}