const PI:f32 = 3.141592;
static VARIAVEL_GLOBAL:u8 = 1;

fn main (){
    println!("Valor da constante: {} , tamanho = {} bytes", PI, std::mem::size_of_val(&PI));

    println!("Valor da variável global: {} , tamanho = {} bytes", VARIAVEL_GLOBAL, std::mem::size_of_val(&VARIAVEL_GLOBAL));
}