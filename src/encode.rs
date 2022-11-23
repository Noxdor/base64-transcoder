const CHAR_MAP: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn encode(bits: &[u8]) -> String {
    let mut bit24_groups: Vec<u32> = vec![];

    let mut i = 0;
    while bits.len() > i + 2 {
        let num = (bits[i] as u32) << 16 | (bits[i + 1] as u32) << 8 | (bits[i + 2] as u32);
        bit24_groups.push(num);
        i += 3;
    }

    let mut base64_string = String::new();

    for four_chars in bit24_groups {
        for i in 0..4 {
            let bit_masked_number = four_chars & (0b11111100_00000000_00000000 >> (6 * i));
            let char: usize = (bit_masked_number >> (6 * (3 - i))) as usize;
            let char = CHAR_MAP[char];
            base64_string.push(char);
        }
    }

    // last hexteds with padding
    let mut last_group: u32 = 0;
    let mut j = 0;

    while bits.len() > i {
        last_group |= (bits[i] as u32) << (8 * (2 - j));
        j += 1;
        i += 1;
    }

    if j != 0 {
        for k in 0..=j {
            let char =
                ((last_group & (0b11111100_00000000_00000000 >> (k * 6))) >> 6 * (3 - k)) as u8;
            let char = CHAR_MAP[char as usize];
            base64_string.push(char);
        }

        // add padding
        for _ in 0..(3 - j) {
            base64_string.push('=');
        }
    }

    base64_string
}
