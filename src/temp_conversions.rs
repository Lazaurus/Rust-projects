/*

Lord forgive me I'm trying to learn rust
Aleksandr S
March 2, 2024


Note: I'll probably in the future be adding "renewed" versions after getting more comfortable w the lang

*/


use std::io;

fn main() {

    loop {
        println!("Would you like to convert to celcius, or farenheit? \nType QUIT to quit program.");
        let mut celc_or_faren = String::new();

        io::stdin()
            .read_line(&mut celc_or_faren)
            .expect("Failed to read line");
        let fucked_off_whitespace = celc_or_faren.trim();

        if fucked_off_whitespace == "celcius" {
            println!("Please enter a number to convert: ");
            let mut input_c = String::new();

            io::stdin()
                .read_line(&mut input_c)
                .expect("Failed to read line!");

            let celcius: f64 = input_c
                .trim()
                .parse()
                .expect("Please enter a number: ");

            let fahrenheit = celsius_to_fahrenheit(celcius);
            println!("{} degrees celcius is equal to {} degrees fahrenheit.", celcius, fahrenheit);
        } 
        else if fucked_off_whitespace == "farenheit" {
            println!("Please enter a number to convert: ");
            let mut input_f = String::new();

            io::stdin()
                .read_line(&mut input_f)
                .expect("Failed to read line!");

            let farenheit: f64 = input_f
                .trim()
                .parse()
                .expect("Failed to read line!");

            let celcius = farenheit_to_celcius(farenheit);
            println!("{} degrees farenheit is equal to {} degrees celcius.", farenheit, celcius);
        }
        if fucked_off_whitespace == "QUIT" {
            break();
        }
    }
}

fn celsius_to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}

fn farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * 5.0 / 9.0
}
