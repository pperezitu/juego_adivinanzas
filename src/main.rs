use rand::Rng;
use std::cmp::Ordering;
use std::io; // es como los imports que se usan en frameworks de JS

fn main() {
    println!("Cual es el número secreto!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor ingrese una respuesta.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Su respuesta es: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy baja!"),
            Ordering::Greater => println!("Muy alta!"),
            Ordering::Equal => {
                println!("Usted gana!");
                break;
            }
        }
    }
}
