fn error(){
    match result() {
        Ok(s) => println!("Ok! {}", s),
        Err(num) => println!("Error Code: {}", num)
    };
}

fn result() -> Result<String, u8> {
    //Ok(String::from("It works!"))
    Err(1)
}

fn main (){
    result();
    error();

}

