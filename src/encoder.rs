use std::fs::File;
use std::io::{BufWriter, Read, Write};

pub mod data;
const BUFFERMAX: usize = 4096 * 4;
// First byte for lossy = 0, non lossless = 1, Files with the same name must allways be compatible
// const VERSION: u8 = 0b00000001u8;
const VERSION: u8 = 0b00000001u8;

pub enum EncoderType {
    Lossy,
}

#[derive(Debug)]
enum State {
    Writing,
    Skipping(u8),
    Filling,
}

/// Goes from the back and checks how many of the characters could be collapsed
/// It takes in the rotation array and gives back a number from the back of many collapsable, so that they dont get written to file just yet
/// ## Example
/// ```
/// use ltc::encoder;
///        assert_eq!(
///            encoder::possible_rotatenumber(&[
///                'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8,
///                '#' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'l' as u8
///            ],) - 1,
///            5 as u8
///        );
/// ```
/// Here the sixed item is not encodable and so the first 5 items are O.K. and are held to the next round
///
pub fn possible_rotatenumber(rotation_array: &[u8; 13]) -> u8 {
    let mut from_back_decodable: u8 = 1;
    //Laeuft durch alle in Array durch und checkt ob alle decoded werden koennen
    for char_as_u8 in rotation_array.iter().rev() {
        if !data::check_if_colapsable(char_as_u8) {
            return from_back_decodable; //Ab diesem punkt ist es nicht mehr decodable und man kann die davor zu disk schreiben
        }
        from_back_decodable += 1;
    }
    return 14; // Alle im Array sind decodable
}

pub struct Encoder {
    read_file: File,
    write_file: File,
    compression_type: EncoderType,
}

/// Writes the remaining collapsable characters that are still in the rotation to file
/// ## Trailing
/// Since the code is rotating one by one, but I write two characters at once, this code has to incorporate this
/// trailing == true => There is one alone, collapable character in the array, this one is paired with an 00001111u8
/// trailing == false => There are an even amount of collapsable characters in roatation array, so am full 11111111u8 has to be inserted
/// ## Output
/// if trailing {
///     return [
///         collaps(
///             data::get_u4_from_char(rotation_array[5]),
///             data::get_u4_from_char(rotation_array[6]),
///         ),
///         collaps(
///             data::get_u4_from_char(rotation_array[7]),
///             data::get_u4_from_char(rotation_array[8]),
///         ),
///         collaps(data::get_u4_from_char(rotation_array[9]), 0b00001111u8),
///         rotation_array[10],
///         rotation_array[11],
///         rotation_array[12],
///         next_byte,
///     ];
/// }
/// return [
///     collaps(
///         data::get_u4_from_char(rotation_array[6]),
///         data::get_u4_from_char(rotation_array[7]),
///     ),
///     collaps(
///         data::get_u4_from_char(rotation_array[8]),
///         data::get_u4_from_char(rotation_array[9]),
///     ),
///     0b11111111u8,
///     rotation_array[10],
///     rotation_array[11],
///     rotation_array[12],
///     next_byte,
/// ];
pub fn cleanup(rotation_array: &[u8; 13], next_byte: u8, trailing: bool) -> [u8; 7] {
    let trailer = if trailing { 0 } else { 1 };
    return [
        collaps(
            data::get_u4_from_char(rotation_array[5 + trailer]),
            data::get_u4_from_char(rotation_array[6 + trailer]),
        ),
        collaps(
            data::get_u4_from_char(rotation_array[7 + trailer]),
            data::get_u4_from_char(rotation_array[8 + trailer]),
        ),
        match trailer {
            0 => collaps(data::get_u4_from_char(rotation_array[9]), 0b00001111u8),
            1 => 0b11111111u8,
            _ => panic!("Does not happen"),
        },
        rotation_array[10],
        rotation_array[11],
        rotation_array[12],
        next_byte,
    ];
}

#[inline]
pub fn collaps(first_char: u8, secound_char: u8) -> u8 {
    return (first_char << 4) | secound_char;
}

#[inline]
pub fn collaps_wraper(rotation_array: &[u8; 13]) -> u8 {
    return collaps(
        data::get_u4_from_char(rotation_array[4]),
        data::get_u4_from_char(rotation_array[5]),
    );
}

impl Encoder {
    pub fn new(read_file: File, write_file: File, compression_type: EncoderType) -> Encoder {
        Encoder {
            read_file,
            write_file,
            compression_type,
        }
    }
    /// Geht durch den input file, checked was encoded werden kann,
    /// schreibt das encodeable encoded und das nicht encodeable in utf8 to file
    /// ## States
    /// ### Skipping
    /// This stage fills up an temporary holding array called rotation_array
    /// If this is full, it check how many of the characters are encodable
    /// #### Not fully encodable
    /// If the rotation array is not fully encodable, it rotates it, until only encodable characters are inside and then starts again with the filling
    /// ### fully encodable
    /// 1. It writes the first 4 characters to file, to help decoding
    /// 2. It writes the indicator change for the decoder to file
    /// 3. It changes the State to Writting
    /// ## Writing
    /// 1. Writes the characters to file
    /// 2. Checks if the next character still can be decoded
    /// ### No
    /// Cleans up the decodable characters in rotation array and sets State to Skipping]
    /// ## Yes
    /// Rotates the rotation array and adds the new character to the end
    /// Changes to Filling
    /// ## Filling
    /// The same as Writing, just that nothing gets written to file
    /// has to be done, since the codes goes through the file, character by character, but my code
    /// when it is encoding writes two, so every second time, it is not possible to write just yet and the code
    /// has to wait to the next character to write
    pub fn encode(&mut self) -> Result<(), std::io::Error> {
        //Hier kann man einstellen wie viel Memory das Program brauchen wird
        let mut buffer = [0; BUFFERMAX];
        let mut state: State = State::Skipping(0);
        let mut rotation_array = [0; 13];
        // let mut output_writer = BufWriter::new(&mut self.write_file);
        let mut output_writer = BufWriter::with_capacity(BUFFERMAX, &mut self.write_file);
        loop {
            let byte_read = self.read_file.read(&mut buffer)?;
            // if the chunk is emty
            if byte_read == 0 {
                break;
            }

            for i in 0..byte_read {
                match state {
                    // Wenn der array noch befuehlt wird
                    State::Skipping(val) if val < 12 => {
                        rotation_array[val as usize] = buffer[i];
                        state = State::Skipping(val + 1);
                    }
                    // Wenn der array voll mit neuen Zahlen ist
                    State::Skipping(val) if val > 11 => {
                        rotation_array[12] = buffer[i];
                        match possible_rotatenumber(&rotation_array) {
                            14 => {
                                state = State::Writing;
                                let write_array = [
                                    rotation_array[0],
                                    rotation_array[1],
                                    rotation_array[2],
                                    rotation_array[3],
                                    0b11111111u8,
                                ];
                                output_writer
                                    .write(&write_array)
                                    .expect("File to write to stoped being accesible");
                            }
                            1 => {
                                // Extra fuer 1, weil dann muss nicht rotiert werden
                                state = State::Skipping(0);
                                output_writer
                                    .write(&rotation_array[0..13 as usize])
                                    .expect("File to write to stoped being accesible");
                            }
                            rotation => {
                                output_writer
                                    .write(&rotation_array[0..((13 - rotation) + 1) as usize])
                                    .expect("File to write to stoped being accesible");
                                rotation_array.rotate_left(((13 - rotation) + 1) as usize);
                                state = State::Skipping(rotation - 1);
                            }
                        }
                    }

                    State::Writing => {
                        // self.write_file
                        output_writer.write(&[collaps_wraper(&rotation_array)])?;
                        if data::check_if_colapsable(&buffer[i]) {
                            rotation_array.rotate_left(1);
                            rotation_array[12] = buffer[i];
                            state = State::Filling;
                        } else {
                            state = State::Skipping(0);
                            output_writer.write(&cleanup(&rotation_array, buffer[i], false))?;
                        }
                    }
                    // Weil man immer zwei zusammenlegen muss, muss man jedes zweite mal
                    // aussetzen um zwei zusammen zu haben, welche man zusammenlegen kann
                    State::Filling => {
                        if data::check_if_colapsable(&buffer[i]) {
                            rotation_array.rotate_left(1);
                            rotation_array[12] = buffer[i];
                            state = State::Writing;
                        } else {
                            state = State::Skipping(0);
                            output_writer.write(&cleanup(&rotation_array, buffer[i], true))?;
                        }
                    }
                    _ => panic!("This is not possible"),
                };
            }
        }
        match state {
            State::Writing => output_writer.write_all(&cleanup(&rotation_array, VERSION, false)),
            State::Filling => output_writer.write_all(&cleanup(&rotation_array, VERSION, true)),
            State::Skipping(value) => Ok(for i in 0..value {
                output_writer.write(&[rotation_array[i as usize]])?;
            }),
        }?;
        output_writer
            .flush()
            .expect("Could not write buffer to file");
        Ok(())
    }
}
