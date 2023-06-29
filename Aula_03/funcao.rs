fn greet (name:&str){
    println!("Hello, {}", name);
}

fn add(a:i32, b:i32) -> i32{
    a + b
}

fn main(){
    greet("Alice");
    let sum = add(3,5);
    println!("Sum: {} ", sum);
}