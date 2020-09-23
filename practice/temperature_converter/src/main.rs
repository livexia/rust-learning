// Convert temperatures between Fahrenheit and Celsius.
// °C = (°F - 32) / 1.8000

use std::io;

enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}

impl Temperature {
    fn converter(&self) {
        match self {
            Temperature::Fahrenheit(i) => {
                if i > &-459.67 {
                    println!("{}°F -> {}°C\n", i, (i - 32.0) / 1.8);
                } else {
                    println!("Input the Fahrenheit that you want convet, must greater than -459.67°F");
                }
            },
            Temperature::Celsius(i) => {
                if i > &-273.15 {
                    println!("{}°C -> {}°F\n", i, i * 1.8 + 32.0);
                } else {
                    println!("Input the Celsius that you want convet, must greater than -273.15°C");
                }
            },
        }
    }
}

fn main() {
    loop {
        println!("Welcome to use Temperature converter!");
        println!("Choose mode(1/2):");
        println!("\t1. Fahrenheit to Celsius");
        println!("\t2. Celsius to Fahrenheit");
        println!("\tInput 0 to quit");

        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        let mode: u32 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input mode 1 or 2, input 0 to quit!\n");
                continue;
            }
        };

        match mode {
            1 => {
                println!("Fahrenheit to Celsius");
                println!("Input the Fahrenheit that you want convet, must greater than -459.67°F");
                Temperature::Fahrenheit(input_temp()).converter();
            }
            2 => {
                println!("Celsius to Fahrenheit");
                println!("Input the Celsius that you want convet, must greater than -273.15°C");
                Temperature::Celsius(input_temp()).converter();
            }
            0 => break,
            _ => println!("Please input mode 1 or 2, input 0 to quit!\n"),
        }
    }
}

fn input_temp() -> f64 {
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f64 = temp
        .trim()
        .parse()
        .expect("Please input the temperature that you want convet!");
    return temp;
}
