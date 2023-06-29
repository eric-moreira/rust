fn main() {
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];
    let matriz = [
        [2.2, 3.4, 5.5, 6.0],
        [0.0, 1.2, 7.3, 7.9]
        ];

    for nota in notas {
        println!("{}", nota);

    }
    println!("");

    for linha in matriz {
        for item in linha{
            print!("{} \t", item);
        }
        println!("");
    }

    vectors();
}

fn vectors(){
    let mut notas:Vec<f32> = vec![10.0, 8.8, 6.5];
    
    println!("{:?}", notas);
    
    notas.push(5.5);

    println!("{:?}", notas);

    //Como tratar um index out of bounds
    println!("Nota 6: {}", match notas.get(5) {
        Some(n) => *n,
        None => 0.0
    });

    if let Some(nota) = notas.pop() {
        println!("Removendo nota: {}", nota);
    }

    while let Some(nota) = notas.pop() {
        println!("Removendo com while nota: {}", nota);
    }

    let mut valores:Vec<f32> = Vec::new();

    for count in 10..=20 {
        valores.push(count as f32 / 1.33);
        //println!("{}", count as f32 / 1.33);
    }

    for valor in &valores {
        println!("Nota: {}", valor);
    }

}
