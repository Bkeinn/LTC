mod decoder;
mod encoder;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::option::Option;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Action,
    #[arg(value_name = "type", long, default_value_t = 0)]
    compression_type: u8,
}

#[derive(Subcommand)]
enum Action {
    Encode {
        #[arg(short, long)]
        ///File to encode
        input: String,
        #[arg(short, long)]
        ///File the output is written to
        output: String,
    },
    Decode {
        #[arg(short, long)]
        ///File to decode
        input: String,
        #[arg(short, long)]
        ///File the decoded is written to
        output: String,
        #[arg(short, long)]
        ///HDF5 file with the weights
        hdf5: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Action::Encode { input, output } => {
            if !get_file_existing(&input) {
                eprintln!("Input file does not exist");
                std::process::exit(1);
            }
            if get_file_existing(&output) {
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
            let file = match std::fs::File::open(&input) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            let mut output_file = match std::fs::File::create(output) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            print!(
                "Input file: {:?}\nOutput file: {:?}\nStatus: ",
                file, output_file
            );
            match args.compression_type {
                0 => {
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
        Action::Decode {
            input,
            output,
            hdf5,
        } => {
            let file = match std::fs::File::open(input) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };
            let mut output_file = match std::fs::File::create(output) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open input file {}", e),
            };

            let hdf5 = match hdf5::File::open(hdf5) {
                Ok(file) => file,
                Err(e) => panic!("Error: unable to open weights file {e}"),
            };

            print!(
                "Input file: {:?}\nOutput file: {:?}\nStatus: ",
                file, output_file
            );
            match decoder::Decoder::new(file, output_file, decoder::EncoderType::Lossy, hdf5)
                .decode()
            {
                Ok(_) => println!("Finished"),
                Err(e) => println!("Error: Something went wrong {}", e),
            }
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
