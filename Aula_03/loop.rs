fn main (){
    let mut counter = 0;

    loop {
        println!("Counter: {}", counter);
        
        counter+=1;

        if counter == 5{
            break;
        }
    }

    for number in 1..=5 {
        println!("Number; {}", number);
    }

    let mut countdown = 5;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
}