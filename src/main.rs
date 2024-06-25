mod decoder;
mod encoder;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::option::Option;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "h" | "H" => get_help(args),
        "d" | "D" => {
            let arguments = get_arguments_decoding(args);
            let file = match std::fs::File::open(&arguments[0]) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            let mut output_file = match std::fs::File::create(arguments[1].as_str()) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            print!(
                "Input file: {:?}\nOutput file: {:?}\nStatus: ",
                file, output_file
            );
            match decoder::Decoder::new(file, output_file, decoder::EncoderType::Lossy).decode() {
                Ok(_) => println!("Finished"),
                Err(e) => println!("Error: Something went wrong {}", e),
            }
        }
        "e" | "E" => {
            let arguments = get_arguments_encoding(args);
            if !get_file_existing(arguments[0].as_str()) {
                eprintln!("Input file does not exist");
                std::process::exit(1);
            }
            if get_file_existing(arguments[1].as_str()) {
                eprintln!("Outputfile does already exist, don't want to overwrite something");
                let mut line = String::new();
                print!("Really want to overwrite the Outputfile. (y/n) ");
                loop {
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut line).unwrap();
                    match line.trim().to_lowercase().as_str() {
                        "y" | "yes" => break,
                        "n" | "no" => std::process::exit(1),
                        _ => println!("Please enter 'y' or 'n'."),
                    }
                    line.clear();
                }
            }
            let file = match std::fs::File::open(&arguments[0]) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            let mut output_file = match std::fs::File::create(arguments[1].as_str()) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            print!(
                "Input file: {:?}\nOutput file: {:?}\nStatus: ",
                file, output_file
            );
            match arguments[2].as_str() {
                "0" => {
                    match encoder::Encoder::new(file, output_file, encoder::EncoderType::Lossy)
                        .encode()
                    {
                        Ok(_) => println!("Finished"),
                        Err(e) => println!("Error: Something went wrong {}", e),
                    }
                }
                _ => panic!("This compression type is not implemented"),
            }
        }
        _ => {
            eprintln!("Usage: {} -I/--input <input_file> -O/--output <output_file> -T/--type <encoding type>", args[0]);
            std::process::exit(1);
        }
    }
}
fn get_help(help_arg: Vec<String>) {
    if help_arg[1] == "-h" || help_arg[1] == "--help" {
        println!(
            "Usage: {} -I/--input <input_file> -O/--output <output_file> -T/--type <encoding type>",
            help_arg[0]
        );
        println!("Type only if encoding, types:");
        println!("  0 : lossy");
    }
}

fn get_file_existing(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        if metadata.len() == 0 {
            true // File exists and is empty
        } else {
            true // File exists and is not empty
        }
    } else {
        false // File does not exist
    }
}

// Next two funktions check the arguments and return a vec with, 0. input 1. output 2.compression type, so praktically the 3 inputs a the encoder strukt has
fn get_arguments_decoding(args: Vec<String>) -> Vec<String> {
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;

    for i in 1..args.len() {
        if args[i] == "-I" || args[i] == "--input" {
            if i + 1 < args.len() {
                input_file = Some(args[i + 1].clone());
            } else {
                eprintln!("Missing input file argument");
                std::process::exit(1);
            }
        } else if args[i] == "-O" || args[i] == "--output" {
            if i + 1 < args.len() {
                output_file = Some(args[i + 1].clone());
            } else {
                eprintln!("Missing output file argument");
                std::process::exit(1);
            }
        }
    }
    let input = match input_file {
        Some(file) => file,
        None => {
            eprintln!("Error: No input file given");
            get_help(args);
            std::process::exit(1);
        }
    };
    let output = match output_file {
        Some(file) => file,
        None => {
            eprintln!("Error: No output file given");
            get_help(args);
            std::process::exit(1);
        }
    };
    return vec![input, output];
}

fn get_arguments_encoding(args: Vec<String>) -> Vec<String> {
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut compression_type: Option<String> = None;

    for i in 1..args.len() {
        if args[i] == "-I" || args[i] == "--input" {
            if i + 1 < args.len() {
                input_file = Some(args[i + 1].clone());
            } else {
                eprintln!("Missing input file argument");
                std::process::exit(1);
            }
        } else if args[i] == "-O" || args[i] == "--output" {
            if i + 1 < args.len() {
                output_file = Some(args[i + 1].clone());
            } else {
                eprintln!("Missing output file argument");
                std::process::exit(1);
            }
        } else if args[i] == "-T" || args[i] == "--type" {
            if i + 1 < args.len() {
                compression_type = Some(args[i + 1].clone());
            } else {
                eprintln!("Missing type argument");
                std::process::exit(1);
            }
        }
    }
    let input = match input_file {
        Some(file) => file,
        None => {
            eprintln!("Error: No input file given");
            get_help(args);
            std::process::exit(1);
        }
    };
    let output = match output_file {
        Some(file) => file,
        None => {
            eprintln!("Error: No output file given");
            get_help(args);
            std::process::exit(1);
        }
    };
    let compression = match compression_type {
        Some(compression) => compression,
        None => {
            eprintln!("Error: No compression type given");
            get_help(args);
            std::process::exit(1);
        }
    };
    return vec![input, output, compression];
}
