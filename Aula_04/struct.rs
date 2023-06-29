struct Person {
    name: String,
    age: u32,
}

fn main(){
    let person = Person{
        name:String::from("Alice"),
        age: 27,
    };

    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Blue;

    println!("Name: {}\n Age: {}", person.name, person.age);
    //println!("Color: {}", color);

}