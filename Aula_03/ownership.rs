fn print_message(message:String){
    println!("{}", message);
}

fn process_data(data:&str){
    //process the data
    println!("Data: {}", data);
}

fn main (){
    let message = String::from("Hello!");

    print_message(message);
    //Ownership transferred to the function
    // println!("{}", message); < Isso retorna um erro, 
    // porque a ownership passou pra print_message, sem referenciar o endereço de memoria 
    let data = "some data";
    process_data(&data);
    //Borrowed as reference
    // println!("{}", data); < isso funciona, porque process_data recebe só um &(endereço) da heap
}