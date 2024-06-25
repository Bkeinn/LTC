pub mod decoder;
pub mod encoder;
pub mod prelude;
use std::fs::{read_to_string, File};
use std::io::{Read, Result};

fn assert_files_equal(file_path1: &str, file_path2: &str) -> Result<()> {
    let mut file1 = File::open(file_path1)?;
    let mut file2 = File::open(file_path2)?;

    let mut content1 = Vec::new();
    let mut content2 = Vec::new();

    file1.read_to_end(&mut content1)?;
    file2.read_to_end(&mut content2)?;

    println!(" File 1 : {:?}", file1);
    println!(" File 2 : {:?}", file2);

    assert_eq!(content1, content2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn construct() {
        let file =
            std::fs::File::open("./basetest_encode.txt").expect("Failed to open the input file.");
        let mut output_file = std::fs::File::create("./basetest_decode.hm")
            .expect("Failed to create the output file.");
        encoder::Encoder::new(file, output_file, encoder::EncoderType::Lossy);
    }
    #[test]
    #[serial]
    fn algemein_encoder() {
        let file =
            std::fs::File::open("./basetest_encode.txt").expect("Failed to open the input file.");
        let mut output_file =
            std::fs::File::create("./basetest_decode.hm").expect("Could not open file");
        let mut encoder = encoder::Encoder::new(file, output_file, encoder::EncoderType::Lossy);
        encoder.encode().expect("Could not encode");
        assert_files_equal("basetest_decode.hm", "basetest_decode_verify.hm")
            .expect("Problem with comparing files");
    }
    #[test]
    #[serial]
    fn algemein_encoder_biger() {
        let file = std::fs::File::open("./encode.txt").expect("Failed to open the input file.");
        let mut output_file =
            std::fs::File::create("./decoder.hm").expect("Failed to create the output file.");
        encoder::Encoder::new(file, output_file, encoder::EncoderType::Lossy)
            .encode()
            .expect("Could not encode file");
    }
    #[test]
    fn check_if_collapsable_working() {
        let array = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        for i in array {
            assert_eq!(encoder::data::check_if_colapsable(&(i as u8)), true)
        }

        let array2 = ['#', ',', '?', '!', '$', 'A', 'S'];

        for i in array2 {
            assert_eq!(encoder::data::check_if_colapsable(&(i as u8)), false)
        }
    }

    #[test]
    fn checks_if_writes_non_decodable_and_returns_rotatenumber() {
        let mut output_file =
            std::fs::File::create("./decode.hm").expect("Failed to create the output file.");
        assert_eq!(
            encoder::possible_rotatenumber(&[
                'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8,
                'h' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'm' as u8
            ],),
            14 as u8
        );
        assert_eq!(
            encoder::possible_rotatenumber(&[
                'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8,
                '#' as u8, 'i' as u8, 'j' as u8, 'k' as u8, ',' as u8, 'm' as u8
            ],) - 1,
            1 as u8
        );
        assert_eq!(
            encoder::possible_rotatenumber(&[
                'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8,
                '#' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, 'l' as u8
            ],) - 1,
            5 as u8
        );

        assert_eq!(
            encoder::possible_rotatenumber(&[
                'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8,
                '#' as u8, 'i' as u8, 'j' as u8, 'k' as u8, 'l' as u8, ';' as u8
            ],),
            1 as u8
        )
    }
    #[test]
    fn check_if_collaps_working() {
        assert_eq!(
            encoder::collaps_wraper(&[0, 0, 0, 0, 'w' as u8, 'e' as u8, 0, 0, 0, 0, 0, 0, 0]),
            0b01001001u8
        );
        assert_eq!(
            encoder::collaps_wraper(&[0, 0, 0, 0, 'a' as u8, 'q' as u8, 0, 0, 0, 0, 0, 0, 0]),
            0b01010111u8
        );
        assert_eq!(
            encoder::collaps_wraper(&[0, 0, 0, 0, 'a' as u8, 'b' as u8, 0, 0, 0, 0, 0, 0, 0]),
            0b01011011u8
        );
    }
    #[test]
    fn check_if_normal_cleanup() {
        assert_eq!(
            encoder::cleanup(
                &[
                    0, 0, 0, 0, 0, 0, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8,
                    'f' as u8, 'g' as u8
                ],
                '#' as u8,
                false
            ),
            [
                0b01011011u8,
                0b00100000,
                0b11111111,
                'e' as u8,
                'f' as u8,
                'g' as u8,
                '#' as u8,
            ]
        );
    }
    #[test]
    fn check_if_trailing_cleanup() {
        assert_eq!(
            encoder::cleanup(
                &[
                    0, 0, 0, 0, 0, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8,
                    'f' as u8, 'g' as u8, 'h' as u8
                ],
                '#' as u8,
                true
            ),
            [
                0b01011011u8,
                0b00100000,
                0b10011111,
                'f' as u8,
                'g' as u8,
                'h' as u8,
                '#' as u8,
            ]
        );
    }
}

#[cfg(test)]
mod decoder_test {
    use super::*;

    #[test]
    fn blank_decoder_test() {
        let file = std::fs::File::open("./basetest_decoder.hm").expect("could not open file : ( ");
        let mut output_file =
            std::fs::File::create("./basetest_decoder.txt").expect("Could not create file");
        decoder::Decoder::new(file, output_file, decoder::EncoderType::Lossy)
            .decode()
            .expect("Blank decoder failed");
        assert_files_equal("basetest_decoder.hm", "basetest_decoder.txt")
            .expect("Files not the same");
    }

    #[test]
    fn decoder_test() {
        let file = std::fs::File::open("./test_decoder.hm").expect("could not open file : ( ");
        let mut output_file =
            std::fs::File::create("./test_decoder.txt").expect("Could not create file");
        decoder::Decoder::new(file, output_file, decoder::EncoderType::Lossy)
            .decode()
            .expect("Decoder failed");
        assert_files_equal("test_decoder.txt", "test_decoder_verify.txt")
            .expect("Files not the same");
    }
}
