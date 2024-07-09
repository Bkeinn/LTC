use hdf5;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
pub mod algerithem;

use algerithem::get_variable;

const BUFFERMAX: usize = 2048 * 8;
const INFLATEDARRAY: [[char; 2]; 16] = [
    ['f', 'd'], //0
    ['p', 'k'], //1
    ['c', 'l'], //2
    ['g', 'o'], //3
    ['n', 'w'], //4
    ['a', 'z'], //5
    ['h', 'x'], //6
    ['m', 'q'], //7
    ['i', 's'], //8
    ['j', 'e'], //9
    ['y', 'v'], //10
    ['b', 'u'], //11
    ['t', 'r'], //12
    [' ', ' '], //13
    ['.', '.'], //14
    [' ', ' '], //15
];

pub enum EncoderType {
    Lossy,
}

enum State {
    Skipping,
    Decoding,
}

enum Wait {
    Stop,
    Unfolding(u8),
}

#[inline]
fn breakwater_to_start(byte: u8) -> bool {
    if byte == 0b11111111u8 {
        return true;
    }
    return false;
}

#[inline]
fn breakwater_to_end(byte: u8) -> bool {
    if (byte & 0b00001111u8) == 0b00001111u8 {
        return true;
    }
    return false;
}

fn tanh(x: f64) -> f64 {
    x.tanh()
}

#[inline]
fn get_two_possible_chars(number: u8) -> [char; 2] {
    return INFLATEDARRAY[number as usize];
}

// Das hier muss man vielleicht umdrehen
#[inline]
fn crack_collaps(byte: u8) -> (u8, u8) {
    return ((byte & 0b11110000u8) >> 4, byte & 0b00001111u8);
}

fn unfold(
    befor_array: [u8; 4],
    questenion_char: u8,
    after_array: [u8; 3],
    dataset: &ndarray::Array3<f64>,
) -> char {
    if questenion_char > 15 {
        return questenion_char as char;
    }
    if questenion_char == 13 {
        return ' ';
    }
    if questenion_char == 14 {
        return '.';
    }
    let first_char = get_two_possible_chars(questenion_char)[0];
    let secound_char = get_two_possible_chars(questenion_char)[1];
    let secound_option: f64 = [0, 1, 2, 3, 5, 6, 7]
        .par_iter()
        .map(|&value| {
            tanh(get_variable_value(
                secound_char,
                value,
                &befor_array,
                &after_array,
                dataset,
            )) as f64
                * match value {
                    3 | 5 => 2.0,
                    6 => 1.4,
                    2 => 1.3,
                    7 => 1.16,
                    1 => 1.09,
                    0 => 1.05,
                    _ => panic!("NOT POSSIBLE"),
                }
        })
        .sum();

    let first_option: f64 = [0, 1, 2, 3, 5, 6, 7]
        .par_iter()
        .map(|&value| {
            tanh(get_variable_value(
                first_char,
                value,
                &befor_array,
                &after_array,
                dataset,
            )) as f64
                * match value {
                    3 | 5 => 2.0,
                    2 => 1.4,
                    6 => 1.3,
                    7 => 1.16,
                    1 => 1.09,
                    0 => 1.05,
                    _ => panic!("NOT POSSIBLE"),
                }
        })
        .sum();

    if first_option > secound_option {
        return first_char;
    } else {
        return secound_char;
    }
}

//Proprably: Looks at base char and then at the other and how propable is it that when base char is given, the other appear
fn get_variable_value(
    character_base: char,
    depth: u8,
    befor_array: &[u8; 4],
    after_array: &[u8; 3],
    dataset: &ndarray::Array3<f64>,
) -> f64 {
    let search_char = match depth {
        0 => befor_array[0],
        1 => befor_array[1],
        2 => befor_array[2],
        3 => befor_array[3],
        5 => after_array[0],
        6 => after_array[1],
        7 => after_array[2],
        _ => panic!("NOT POSSIBLE"),
    };
    if search_char > 15 {
        return get_variable(character_base, depth, search_char as char, dataset) * 2.0;
    } else if search_char < 13 {
        let two_options: [char; 2] = get_two_possible_chars(search_char);
        return get_variable(character_base, depth, two_options[0], dataset)
            + get_variable(character_base, depth, two_options[1], dataset);
    } else if search_char == 13 {
        return get_variable(character_base, depth, ' ', dataset);
    } else if search_char == 14 {
        return get_variable(character_base, depth, '.', dataset);
    } else {
        eprintln!("character base = {}", character_base);
        eprintln!("search_char = {}", search_char);
        eprintln!("depth = {}", depth);
        panic!("Your magical, keep it this way")
    }
}

fn outputbuffer_writer(
    output_writer: &mut BufWriter<&File>,
    to_write: &[u8],
    pass_counter: &mut u8,
) {
    if *pass_counter == 0 {
        output_writer
            .write_all(to_write)
            .expect("Could not write to file");
    } else {
        *pass_counter -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print<T>(value: T) -> T
    where
        T: std::fmt::Debug, // Ensure T can be printed with debug formatting
    {
        value // Return the original value
    }

    #[test]
    fn breakwater_start_test_small() {
        assert_eq!(breakwater_to_start(99), false);
        assert_eq!(breakwater_to_start(255), true);
        assert_eq!(breakwater_to_start(47), false);
    }
    #[test]
    fn breakwater_end_test_small() {
        assert_eq!(breakwater_to_end(99), false);
        assert_eq!(breakwater_to_end(255), true);
        assert_eq!(breakwater_to_end(47), true);
    }
    #[test]
    fn crack_collaps_test() {
        assert_eq!(crack_collaps(86), (5, 6));
        assert_eq!(crack_collaps(120), (7, 8));
        assert_eq!(crack_collaps(154), (9, 10));
        assert_eq!(crack_collaps(188), (11, 12));
    }
    #[test]
    fn unfold_test() {
        let hdf_file = hdf5::File::open("full.h5").unwrap();
        let dataset = hdf_file.dataset("normalized").unwrap().read().unwrap();
        assert_eq!(
            unfold([44, 32, 105, 110], 13, [116, 104, 101], &dataset),
            ' '
        ); // If everything is clear
        assert_eq!(unfold([13, 8, 4, 13], 12, [9, 13, 11], &dataset), 't');
        // Everything unsertant
    }
}

pub struct Decoder {
    read_file: File,
    write_file: File,
    compress_type: EncoderType,
    weight_file: hdf5::File,
}

impl Decoder {
    pub fn new(
        read_file: File,
        write_file: File,
        compress_type: EncoderType,
        weight_file: hdf5::File,
    ) -> Decoder {
        Decoder {
            read_file,
            write_file,
            compress_type,
            weight_file,
        }
    }
    pub fn decode(&mut self) -> Result<(), std::io::Error> {
        let dataset = self
            .weight_file
            .dataset("normalized")
            .unwrap()
            .read()
            .unwrap();

        let mut rotation_array: [u8; 8] = [0; 8];
        let mut buffer = [0; BUFFERMAX];
        let mut state: State = State::Skipping;
        let mut output_writer = BufWriter::new(&self.write_file);
        let mut waiter: Wait = Wait::Stop;
        // ----- Beginnign
        let mut pass_counter: u8 = 8;
        // ----- End
        loop {
            let byte_read = self.read_file.read(&mut buffer)?;
            if byte_read == 0 {
                break;
            }
            for i in 0..byte_read {
                match waiter {
                    Wait::Unfolding(val) if val > 0 => {
                        waiter = Wait::Unfolding(val - 1);
                        let befor_array = [
                            rotation_array[0],
                            rotation_array[1],
                            rotation_array[2],
                            rotation_array[3],
                        ];
                        let after_array = [rotation_array[5], rotation_array[6], rotation_array[7]];
                        rotation_array[4] =
                            unfold(befor_array, rotation_array[4], after_array, &dataset) as u8;
                        state = State::Skipping;
                    }
                    Wait::Unfolding(val) if val == 0 => waiter = Wait::Stop,
                    _ => (),
                }
                match state {
                    State::Skipping => {
                        if breakwater_to_start(buffer[i]) {
                            state = State::Decoding;
                        } else {
                            outputbuffer_writer(
                                &mut output_writer,
                                &[rotation_array[0]],
                                &mut pass_counter,
                            );
                            rotation_array.rotate_left(1);
                            rotation_array[7] = buffer[i];
                        }
                    }
                    State::Decoding => match breakwater_to_end(buffer[i]) {
                        true => {
                            match breakwater_to_start(buffer[i]) {
                                true => state = State::Skipping,
                                false => {
                                    outputbuffer_writer(
                                        &mut output_writer,
                                        &[rotation_array[0]],
                                        &mut pass_counter,
                                    );
                                    rotation_array.rotate_left(1);
                                    rotation_array[7] = crack_collaps(buffer[i]).0;
                                }
                            }
                            let befor_array = [
                                rotation_array[0],
                                rotation_array[1],
                                rotation_array[2],
                                rotation_array[3],
                            ];
                            let after_array =
                                [rotation_array[5], rotation_array[6], rotation_array[7]];
                            rotation_array[4] =
                                unfold(befor_array, rotation_array[4], after_array, &dataset) as u8;
                            state = State::Skipping;
                            waiter = Wait::Unfolding(4);
                        }
                        false => {
                            for push in 0..2 {
                                outputbuffer_writer(
                                    &mut output_writer,
                                    &[rotation_array[0]],
                                    &mut pass_counter,
                                );
                                rotation_array.rotate_left(1);
                                if push == 0 {
                                    rotation_array[7] = crack_collaps(buffer[i]).0;
                                } else {
                                    rotation_array[7] = crack_collaps(buffer[i]).1;
                                }
                                let befor_array = [
                                    rotation_array[0],
                                    rotation_array[1],
                                    rotation_array[2],
                                    rotation_array[3],
                                ];
                                let after_array =
                                    [rotation_array[5], rotation_array[6], rotation_array[7]];
                                rotation_array[4] =
                                    unfold(befor_array, rotation_array[4], after_array, &dataset)
                                        as u8;
                            }
                        }
                    },
                }
            }
        }
        match waiter {
            Wait::Unfolding(val) => {
                let befor_array = [
                    rotation_array[0],
                    rotation_array[1],
                    rotation_array[2],
                    rotation_array[3],
                ];
                let after_array = [rotation_array[5], rotation_array[6], rotation_array[7]];
                rotation_array[4] =
                    unfold(befor_array, rotation_array[4], after_array, &dataset) as u8;
            }
            _ => (),
        }
        output_writer.write_all(&rotation_array);
        output_writer.flush()?;
        Ok(())
    }
}
