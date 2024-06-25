pub fn get_u4_from_char(number: u8) -> u8 {
    let character = number as char;
    // println!("Char = {}", character);
    match character {
        'f' => return 0,
        'd' => return 0,
        'p' => return 1,
        'k' => return 1,
        'c' => return 2,
        'l' => return 2,
        'g' => return 3,
        'o' => return 3,
        'n' => return 4,
        'w' => return 4,
        'a' => return 5,
        'z' => return 5,
        'h' => return 6,
        'x' => return 6,
        'm' => return 7,
        'q' => return 7,
        'i' => return 8,
        's' => return 8,
        'j' => return 9,
        'e' => return 9,
        'y' => return 10,
        'v' => return 10,
        'b' => return 11,
        'u' => return 11,
        't' => return 12,
        'r' => return 12,
        ' ' => return 13,
        '.' => return 14,
        _ => panic!("Invalid character has been loaded into get_u4_from_char"),
    }
}

pub fn turn_to_char(array: &[u8; 13]) -> [char; 13] {
    return [
        array[0] as char,
        array[1] as char,
        array[2] as char,
        array[3] as char,
        array[4] as char,
        array[5] as char,
        array[6] as char,
        array[7] as char,
        array[8] as char,
        array[9] as char,
        array[10] as char,
        array[11] as char,
        array[12] as char,
    ];
}

pub fn check_if_colapsable(number: &u8) -> bool {
    match number {
        x if x == &('a' as u8)
            || x == &('b' as u8)
            || x == &('c' as u8)
            || x == &('d' as u8)
            || x == &('e' as u8)
            || x == &('f' as u8)
            || x == &('g' as u8)
            || x == &('h' as u8)
            || x == &('i' as u8)
            || x == &('j' as u8)
            || x == &('k' as u8)
            || x == &('l' as u8)
            || x == &('m' as u8)
            || x == &('n' as u8)
            || x == &('o' as u8)
            || x == &('p' as u8)
            || x == &('q' as u8)
            || x == &('r' as u8)
            || x == &('s' as u8)
            || x == &('t' as u8)
            || x == &('u' as u8)
            || x == &('v' as u8)
            || x == &('w' as u8)
            || x == &('x' as u8)
            || x == &('y' as u8)
            || x == &('z' as u8)
            || x == &(' ' as u8)
            || x == &('.' as u8) =>
        {
            return true
        }
        _ => return false,
    }
}
