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

fn tanh(x: f32) -> f32 {
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

// 0 is neares to the unfold number
struct Return_array_4 {
    filled_to: u8,
    array: [u8; 4],
}

impl Return_array_4 {
    fn add_one(&mut self, bit: u8) {
        if self.filled_to <= 3 {
            self.array[3 - self.filled_to as usize] = bit;
            self.filled_to += 1;
        }
    }
    fn add_two(&mut self, bit: (u8, u8)) {
        match self.filled_to {
            3 => self.add_one(bit.1),
            4 => {}
            _ => {
                self.array[3 - self.filled_to as usize] = bit.1;
                self.array[3 - (self.filled_to as usize + 1)] = bit.0;
                self.filled_to += 2;
            }
        }
    }
    fn out(&self) -> [u8; 4] {
        return self.array;
    }
}

struct ReturnArray3 {
    filled_to: u8,
    array: [u8; 3],
}

impl ReturnArray3 {
    fn add_one(&mut self, bit: u8) {
        if self.filled_to <= 2 {
            self.array[self.filled_to as usize] = bit;
            self.filled_to += 1;
        }
    }
    fn add_two(&mut self, bit: (u8, u8)) {
        match self.filled_to {
            2 => self.add_one(bit.0),
            3 => {}
            _ => {
                self.array[self.filled_to as usize] = bit.0;
                self.array[self.filled_to as usize + 1] = bit.1;
                self.filled_to += 2;
            }
        }
    }
    fn out(&self) -> [u8; 3] {
        return self.array;
    }
}

fn get_previous_bytes(
    previous_bytes: &[u8; 5],
    iteration_number: usize,
    buffer: &[u8; BUFFERMAX],
    current_part: bool,
) -> [u8; 4] {
    let mut array = Return_array_4 {
        filled_to: 0,
        array: [0; 4],
    };
    // Wenn der zweite Teil eines Bytes berechnet wird muss der erste auch mit einbezogen werden
    if current_part {
        array.add_one(crack_collaps(buffer[iteration_number]).0);
    }
    let mut umgebrochen = false;
    for i in 1..6 {
        let current_number = if iteration_number >= i {
            buffer[iteration_number - i]
        } else {
            previous_bytes[5 - (i - iteration_number)]
        };
        if breakwater_to_start(current_number) && umgebrochen == false {
            umgebrochen = true;
        } else {
            match umgebrochen {
                true => array.add_one(current_number),
                false => array.add_two(crack_collaps(current_number)),
            }
        }
        if array.filled_to >= 4 {
            return array.out();
        }
    }
    // }
    panic!("Could not fill looking back array");
}

fn get_previous_bytes_promis(
    iteration_number: usize,
    buffer: &[u8; 14],
    current_part: bool,
) -> [u8; 4] {
    let mut array = Return_array_4 {
        filled_to: 0,
        array: [0; 4],
    };
    // Wenn der zweite Teil eines Bytes berechnet wird muss der erste auch mit einbezogen werden
    if current_part {
        array.add_one(crack_collaps(buffer[iteration_number]).0);
    }
    let mut umgebrochen = false;
    for i in 1..6 {
        if breakwater_to_start(buffer[iteration_number - (i)]) && umgebrochen == false {
            umgebrochen = true;
        } else {
            match umgebrochen {
                true => array.add_one(buffer[iteration_number - (i)]),
                false => array.add_two(crack_collaps(buffer[iteration_number - (i)])),
            }
        }
        if array.filled_to >= 4 {
            return array.out();
        }
    }
    panic!("Could not fill looking back array");
}

#[inline]
fn lookforward_local(iteration_number: usize) -> bool {
    if iteration_number >= BUFFERMAX - 5 {
        return false;
    }
    return true;
}

fn unfold(befor_array: [u8; 4], questenion_char: u8, after_array: [u8; 3]) -> char {
    if questenion_char == 13 {
        return ' ';
    }
    if questenion_char == 14 {
        return '.';
    }
    let first_char = get_two_possible_chars(questenion_char)[0];
    let secound_char = get_two_possible_chars(questenion_char)[1];
    let secound_option: f64 = [4, 3, 2, 1, -1, -2, -3]
        .par_iter()
        .map(|&value| {
            tanh(get_variable_value(
                secound_char,
                value,
                &befor_array,
                &after_array,
            )) as f64
                * match value {
                    1 | -1 => 2.0,
                    -2 => 1.4,
                    2 => 1.3,
                    -3 => 1.16,
                    3 => 1.09,
                    4 => 1.05,
                    _ => panic!("NOT POSSIBLE"),
                }
        })
        .sum();

    let first_option: f64 = [4, 3, 2, 1, -1, -2, -3]
        .par_iter()
        .map(|&value| {
            tanh(get_variable_value(
                first_char,
                value,
                &befor_array,
                &after_array,
            )) as f64
                * match value {
                    1 | -1 => 2.0,
                    -2 => 1.4,
                    2 => 1.3,
                    -3 => 1.16,
                    3 => 1.09,
                    4 => 1.05,
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
    depth: i16,
    befor_array: &[u8; 4],
    after_array: &[u8; 3],
) -> f32 {
    let search_char = match depth {
        4 => befor_array[0],
        3 => befor_array[1],
        2 => befor_array[2],
        1 => befor_array[3],
        -1 => after_array[0],
        -2 => after_array[1],
        -3 => after_array[2],
        _ => panic!("NOT POSSIBLE"),
    };
    if search_char > 15 {
        return get_variable(character_base, depth, search_char as char) * 2.0;
    } else if search_char < 13 {
        let two_options: [char; 2] = get_two_possible_chars(search_char);
        return get_variable(character_base, depth, two_options[0])
            + get_variable(character_base, depth, two_options[1]);
    } else if search_char == 13 {
        return get_variable(character_base, depth, ' ');
    } else if search_char == 14 {
        return get_variable(character_base, depth, '.');
    } else {
        eprintln!("character base = {}", character_base);
        eprintln!("search_char = {}", search_char);
        eprintln!("depth = {}", depth);
        panic!("Your magical, keep it this way")
    }
}

// Current part, muss immer anders sein. Wenn get_next_byte = true dann muss get_previous_byte = false und anders rum
fn get_next_bytes_local(
    iteration_number: usize,
    buffer: &[u8; BUFFERMAX],
    current_part: bool,
) -> [u8; 3] {
    let mut array = ReturnArray3 {
        filled_to: 0,
        array: [0; 3],
    };
    let mut umgebrochen = false;
    if current_part {
        if breakwater_to_end(buffer[iteration_number]) {
            umgebrochen = true;
        } else {
            array.add_one(crack_collaps(buffer[iteration_number]).1);
        }
    }
    for i in 1..5 {
        if breakwater_to_end(buffer[iteration_number + i]) && umgebrochen == false {
            umgebrochen = true;
            if !breakwater_to_start(buffer[iteration_number + i]) {
                array.add_one(crack_collaps(buffer[iteration_number + i]).0);
            }
        } else {
            match umgebrochen {
                false => array.add_two(crack_collaps(buffer[iteration_number + i])),
                true => array.add_one(buffer[iteration_number + i]),
            }
        }
        if array.filled_to == 3 {
            return array.out();
        }
    }
    panic!("could not fill next bytes")
}

fn get_next_bytes_local_promis(
    iteration_number: usize,
    buffer: &[u8; 14],
    current_part: bool,
) -> [u8; 3] {
    let mut array = ReturnArray3 {
        filled_to: 0,
        array: [0; 3],
    };
    let mut umgebrochen = false;
    if current_part {
        if breakwater_to_end(buffer[iteration_number]) {
            umgebrochen = true;
        } else {
            array.add_one(crack_collaps(buffer[iteration_number]).1);
        }
    }
    for i in 1..5 {
        if breakwater_to_end(buffer[iteration_number + i]) && umgebrochen == false {
            umgebrochen = true;
            if !breakwater_to_start(buffer[iteration_number + i]) {
                array.add_one(crack_collaps(buffer[iteration_number + i]).0);
            }
        } else {
            match umgebrochen {
                false => array.add_two(crack_collaps(buffer[iteration_number + i])),
                true => array.add_one(buffer[iteration_number + i]),
            }
        }
        if array.filled_to == 3 {
            return array.out();
        }
    }
    panic!("could not fill next bytes")
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
    fn return_array_test() {
        let mut struct_test = Return_array_4 {
            filled_to: 0,
            array: [0; 4],
        };
        // 4 3 2 1 0
        struct_test.add_one(0);
        struct_test.add_two((2, 1));
        struct_test.add_two((4, 3));
        assert_eq!([3, 2, 1, 0], struct_test.out())
    }
    #[test]
    fn get_previous_bytes_test() {
        let mut results = [[0; 4]; 4];
        let mut buffer = [0; BUFFERMAX];
        buffer[0] = 86;
        buffer[1] = 120;
        buffer[2] = 154;
        buffer[3] = 188;
        let previous_bytes = [1, 2, 3, 4, 255];
        for i in 0..4 {
            let boolean = false;
            results[i] = get_previous_bytes(&previous_bytes, i, &buffer, boolean);
        }
        assert_eq!(
            results,
            [[1, 2, 3, 4], [3, 4, 5, 6], [5, 6, 7, 8], [7, 8, 9, 10]]
        );
        assert_eq!(
            get_previous_bytes(&[105, 110, 99, 255, 43], 0, &buffer, false),
            [110, 99, 2, 11]
        );
    }
    #[test]
    fn crack_collaps_test() {
        assert_eq!(crack_collaps(86), (5, 6));
        assert_eq!(crack_collaps(120), (7, 8));
        assert_eq!(crack_collaps(154), (9, 10));
        assert_eq!(crack_collaps(188), (11, 12));
    }

    #[test]
    fn get_next_bytes_local_test() {
        let mut buffer: [u8; BUFFERMAX] = [0; BUFFERMAX];
        buffer[0] = 1;
        buffer[1] = 47;
        buffer[2] = 3;
        buffer[3] = 4;
        assert_eq!(get_next_bytes_local(0, &buffer, true), [1, 2, 3]); // true == es wird der erste teil des Bytes entschluesselt, also wird erst der zweite mit genommen
        assert_eq!(get_next_bytes_local(0, &buffer, false), [2, 3, 4]); // false == es wird der zweite teil entschluesselt, also wird erst der naechste byte mit genommen
        buffer[1] = 48;
        buffer[2] = 205;
        buffer[3] = 255;
        buffer[4] = 104;
        assert_eq!(get_next_bytes_local(1, &buffer, false), [12, 13, 104])
    }
    #[test]
    fn unfold_test() {
        assert_eq!(unfold([44, 32, 105, 110], 13, [116, 104, 101]), ' '); // If everything is clear
        assert_eq!(unfold([13, 8, 4, 13], 12, [9, 13, 11]), 't');
        // Everything unsertant
    }
}

pub struct Decoder {
    read_file: File,
    write_file: File,
    compress_type: EncoderType,
}

impl Decoder {
    pub fn new(read_file: File, write_file: File, compress_type: EncoderType) -> Decoder {
        Decoder {
            read_file,
            write_file,
            compress_type,
        }
    }
    pub fn decode(&mut self) -> Result<(), std::io::Error> {
        let mut promis = [0; 14];
        let mut promis_made = false;
        let mut buffer = [0; BUFFERMAX];
        let mut state: State = State::Skipping;
        let mut output_writer = BufWriter::new(&self.write_file);
        // ----- Previous bytes management
        /// This is needed if it is decoding while the buffer changes
        let mut previous_last_bytes: [u8; 5] = [0; 5];
        loop {
            let byte_read = self.read_file.read(&mut buffer)?;
            if byte_read == 0 {
                break;
            }

            if promis_made {
                promis[10] = match buffer.get(0) {
                    Some(&value) => value,
                    None => 0,
                };
                promis[11] = match buffer.get(1) {
                    Some(&value) => value,
                    None => 0,
                };
                promis[12] = match buffer.get(2) {
                    Some(&value) => value,
                    None => 0,
                };
                promis[13] = match buffer.get(3) {
                    Some(&value) => value,
                    None => 0,
                };
                // eprintln!("PROMIS = {:?}", promis);
                for i in 5..10 {
                    match state {
                        State::Skipping => {
                            if breakwater_to_start(promis[i]) {
                                state = State::Decoding;
                            } else {
                                output_writer.write(&[promis[i]])?;
                            }
                        }
                        State::Decoding => match breakwater_to_end(promis[i]) {
                            true => match breakwater_to_start(promis[i]) {
                                true => state = State::Skipping,
                                false => {
                                    output_writer
                                        .write_all(&[unfold(
                                            get_previous_bytes_promis(i, &promis, false),
                                            crack_collaps(promis[i]).0,
                                            get_next_bytes_local_promis(i, &promis, true),
                                        )
                                            as u8])
                                        .expect("Could not write to file");
                                    state = State::Skipping;
                                }
                            },
                            false => output_writer
                                .write_all(&[
                                    unfold(
                                        get_previous_bytes_promis(i, &promis, false),
                                        crack_collaps(promis[i]).0,
                                        get_next_bytes_local_promis(i, &promis, true),
                                    ) as u8,
                                    unfold(
                                        get_previous_bytes_promis(i, &promis, true),
                                        crack_collaps(promis[i]).1,
                                        get_next_bytes_local_promis(i, &promis, false),
                                    ) as u8,
                                ])
                                .expect("Could not write to file"),
                        },
                    }
                }
            }

            for i in 0..byte_read {
                if !lookforward_local(i) {
                    break;
                }
                match state {
                    State::Skipping => {
                        if breakwater_to_start(buffer[i]) {
                            state = State::Decoding;
                        } else {
                            output_writer.write(&[buffer[i]])?;
                        }
                    }
                    State::Decoding => match breakwater_to_end(buffer[i]) {
                        true => match breakwater_to_start(buffer[i]) {
                            true => state = State::Skipping,
                            false => {
                                output_writer
                                    .write_all(&[unfold(
                                        get_previous_bytes(&previous_last_bytes, i, &buffer, false),
                                        crack_collaps(buffer[i]).0,
                                        get_next_bytes_local(i, &buffer, true),
                                    ) as u8])
                                    .expect("Could not write to file");
                                state = State::Skipping;
                            }
                        },
                        false => {
                            output_writer
                                .write_all(&[
                                    unfold(
                                        get_previous_bytes(&previous_last_bytes, i, &buffer, false),
                                        crack_collaps(buffer[i]).0,
                                        get_next_bytes_local(i, &buffer, true),
                                    ) as u8,
                                    unfold(
                                        get_previous_bytes(&previous_last_bytes, i, &buffer, true),
                                        crack_collaps(buffer[i]).1,
                                        get_next_bytes_local(i, &buffer, false),
                                    ) as u8,
                                ])
                                .expect("Could not write to file");
                        }
                    },
                }
            }
            if byte_read >= 5 {
                previous_last_bytes.copy_from_slice(&buffer[(byte_read - 5)..byte_read]);
            }
            /// Wenn der Buffer endet aber der file noch nicht, kann es sein dass man die letzten Zeichen zum dekodieren, des naechsten buffers braucht
            if byte_read == BUFFERMAX {
                promis_made = true;
                promis.copy_from_slice(&buffer[(BUFFERMAX - 14)..BUFFERMAX])
            }
        }
        output_writer.flush()?;
        Ok(())
    }
}
