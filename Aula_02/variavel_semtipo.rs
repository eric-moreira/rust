fn main(){
    let variavel = 300;
    println!("Valor da variável: {} , tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));


    let decimal = 2.5;
    println!("Valor da variável: {} , tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let booleano = false;
    println!("Valor da variável: {} , tamanho = {} bytes", booleano, std::mem::size_of_val(&booleano));

    let letra:char = 'a';
    println!("Valor da variável: {} , tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}