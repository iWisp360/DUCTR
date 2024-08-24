use std::{
    io::{self, Write},
    process::exit,
};

const VALUE_OF_A_KIBIBYTE_IN_BYTES: usize = 1024;
#[derive(PartialEq, Eq)]
enum StorageUnit {
    B,
    KiB,
    MiB,
    GiB,
    TiB,
}

fn main() {
    println!("Data Units Conversion Tool, in Rust.");
    println!("Enter 'h' for help.");
    loop {
        print!("(DUCTR) ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't store input!");
        match input.to_lowercase().trim() {
            "h" => print_help(),
            "e" => exit(0),
            "c" => loop {
                let mut user_choice = String::new();
                print!("Which is your input unit? (1: B, 2: KiB, 3: MiB, 4: GiB, 5: TiB): ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut user_choice)
                    .expect("Couldn't store input!");
                let input = match user_choice.trim().parse::<i8>().unwrap_or(0) {
                    1 => StorageUnit::B,
                    2 => StorageUnit::KiB,
                    3 => StorageUnit::MiB,
                    4 => StorageUnit::GiB,
                    5 => StorageUnit::TiB,
                    _ => {
                        println!("Not a valid input!");
                        continue;
                    }
                };
                loop {
                    let mut user_choice = String::new();
                    print!("Which is your output unit? (1: B, 2: KiB, 3: MiB, 4: GiB, 5: TiB): ");
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut user_choice)
                        .expect("Couldn't store input!");
                    let output = match user_choice.trim().parse().unwrap_or(0) {
                        1 => StorageUnit::B,
                        2 => StorageUnit::KiB,
                        3 => StorageUnit::MiB,
                        4 => StorageUnit::GiB,
                        5 => StorageUnit::TiB,
                        _ => {
                            println!("Not a valid input!");
                            continue;
                        }
                    };
                    if input == output {
                        println!("Same data unit not allowed!");
                        continue;
                    }
                    loop {
                        let mut number = String::new();
                        print!("Now type the number to convert: ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut number)
                            .expect("Couldn't store String!");
                        let number = match number.trim().parse() {
                            Ok(number) => number,
                            Err(_) => {
                                println!("That's not a number!");
                                continue;
                            }
                        };
                        let input_unit = match input {
                            StorageUnit::B => "B",
                            StorageUnit::KiB => "KiB",
                            StorageUnit::MiB => "MiB",
                            StorageUnit::GiB => "GiB",
                            StorageUnit::TiB => "TiB",
                        };
                        let output_unit = match output {
                            StorageUnit::B => "bytes",
                            StorageUnit::KiB => "kibibytes",
                            StorageUnit::MiB => "mebibytes",
                            StorageUnit::GiB => "gibibytes",
                            StorageUnit::TiB => "tebibytes",
                        };
                        let result = convert(input, output, number);
                        println!(
                            "The result from {} {} to {} is {} {}.",
                            number, input_unit, output_unit, result, output_unit
                        );
                        break;
                    }
                    break;
                }
                break;
            },

            "" => continue,
            _ => println!("Not a valid input!"),
        }
    }
}

fn print_help() {
    println!("Data Units Conversion Tool, in Rust.\n");
    println!("'c': Convert");
    println!("'h': Help");
    println!("'e': Exit\n");
}

fn convert(input: StorageUnit, output: StorageUnit, number: f64) -> f64 {
    match input {
        StorageUnit::B => match output {
            StorageUnit::KiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            StorageUnit::MiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(2) as f64,
            StorageUnit::GiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(3) as f64,
            StorageUnit::TiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(4) as f64,
            _ => number,
        },
        StorageUnit::KiB => match output {
            StorageUnit::B => number * VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            StorageUnit::MiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            StorageUnit::GiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(2) as f64,
            StorageUnit::TiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(3) as f64,
            _ => number,
        },
        StorageUnit::MiB => match output {
            StorageUnit::B => number * VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(2) as f64,
            StorageUnit::KiB => number * VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            StorageUnit::GiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            StorageUnit::TiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(2) as f64,
            _ => number,
        },
        StorageUnit::GiB => match output {
            StorageUnit::B => number * VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(3) as f64,
            StorageUnit::KiB => number * VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(2) as f64,
            StorageUnit::MiB => number * VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            StorageUnit::TiB => number / VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            _ => number,
        },
        StorageUnit::TiB => match output {
            StorageUnit::B => number * VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(4) as f64,
            StorageUnit::KiB => number * VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(3) as f64,
            StorageUnit::MiB => number * VALUE_OF_A_KIBIBYTE_IN_BYTES.pow(2) as f64,
            StorageUnit::GiB => number * VALUE_OF_A_KIBIBYTE_IN_BYTES as f64,
            _ => number,
        },
    }
}

