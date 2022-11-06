use std::io;

fn main() {
    println!("Target coversion (C/F): ");

    let mut convert_to = String::new();

    io::stdin()
        .read_line(&mut convert_to)
        .expect("Error reading line");

    match convert_to.trim() {
        "C" => {
            println!("F: ");
            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read fahrenheit");
            let fahrenheit: f64 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!(),
            };

            println!("C: {}", (fahrenheit - 32.0) * (5.0 / 9.0));
        }
        "F" => {
            println!("C: ");
            let mut celcius = String::new();

            io::stdin()
                .read_line(&mut celcius)
                .expect("Failed to read celcius");
            let celcius: f32 = match celcius.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!(),
            };

            println!("F: {}", (celcius * 9.0 / 5.0) + 32.0);
        }
        _ => {
            println!("Bad input");
        }
    }
}
