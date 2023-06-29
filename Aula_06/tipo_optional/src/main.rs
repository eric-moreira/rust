fn read_file(path: String) -> Option<String> {
    Some(path) 
    //None

}



fn main() {

    let conteudo = read_file(String::from("/home/eric/text.txt"));

    match &conteudo {
        Some(valor) => println!("{}", valor),
        None => println!("arquivo nao existe")
    };

    if let Some(valor) = conteudo {
        println!("o comando (if let) me garante que o valor passado é o Some(valor), caso eu não queira usar o match.")
    }
    
    //Caso seja None, ele passa direto.


    // Print debug
    // println!("{:?}", conteudo);
}
