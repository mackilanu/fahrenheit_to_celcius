use std::io;

fn main() {

    let mut fahr = String::new();

    println!("Welcome to the Fahremheit-to-celcius converter program!");

    println!("Please enter a value in fahrenheit: ");

    io::stdin()
        .read_line(&mut fahr)
        .expect("Failed to read line.");

    let fahr: f32 = match fahr.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };

    let celc = (fahr - 32.0) / 1.8;

    println!("{}", celc);
}
