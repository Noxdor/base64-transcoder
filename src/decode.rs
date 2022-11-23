// ascii map of base64 characters
const BYTE_MAP: [u8; 123] = [
    0, // index 0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // index 10
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // index 20
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // index 30
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // index 40
    0, 0, 0, // index 43 - +
    0, 0, 0, 0b11_1111, // index 47 - /
    0b11_0100, // index 48 - 0
    0b11_0101, 0b11_0110, // index 50
    0b11_0111, 0b11_1000, 0b11_1001, 0b11_1010, 0b11_1011, 0b11_1100,
    0b11_1101, // index 57 - 9
    0, 0, 0, // index 60
    0, 0, 0, 0, 0b00_0000, // index 65 - A
    0b00_0001, 0b00_0010, 0b00_0011, 0b00_0100, 0b00_0101, // index 70
    0b00_0110, 0b00_0111, 0b00_1000, 0b00_1001, 0b00_1010, 0b00_1011, 0b00_1100, 0b00_1101,
    0b00_1110, 0b00_1111, // index 80
    0b01_0000, 0b01_0001, 0b01_0010, 0b01_0011, 0b01_0100, 0b01_0101, 0b01_0110, 0b01_0111,
    0b01_1000, 0b01_1001, // index 90 - Z
    0, 0, 0, 0, 0, 0, 0b01_1010, // index 97 - a
    0b01_1011, 0b01_1100, 0b01_1101, // index 100
    0b01_1110, 0b01_1111, 0b10_0000, 0b10_0001, 0b10_0010, 0b10_0011, 0b10_0100, 0b10_0101,
    0b10_0110, 0b10_0111, // index 110
    0b10_1000, 0b10_1001, 0b10_1010, 0b10_1011, 0b10_1100, 0b10_1101, 0b10_1110, 0b10_1111,
    0b11_0000, 0b11_0001, // index 120
    0b11_0010, 0b11_0011, // index 122 - z
];

pub fn decode(base64_string: &str) -> Vec<u8> {
    // let mut base64_string = base64_string.trim_end_matches('=').to_owned();
    let mut base64_string = base64_string.to_owned();

    let mut bit24_groups: Vec<u32> = vec![];

    let mut i = 0;
    let mut current_group: u32 = 0;
    while let Some(char_) = base64_string.pop() {
        if char_ != '=' {
            println!("char: {char_}");
            let char_ = BYTE_MAP[char_ as usize] as u32;
            current_group |= char_ << (i * 6);
        }
        if i == 3 {
            bit24_groups.push(current_group);
            current_group = 0;
        }
        i = (i + 1) % 4;
    }

    bit24_groups.reverse();

    let mut output_vector: Vec<u8> = vec![];
    for group in bit24_groups {
        let (u8_1, u8_2, u8_3) = (
            ((group & 0b00000000_11111111_00000000_00000000) >> 16) as u8,
            ((group & 0b00000000_00000000_11111111_00000000) >> 8) as u8,
            (group & 0b00000000_00000000_00000000_11111111) as u8,
        );
        output_vector.push(u8_1);
        output_vector.push(u8_2);
        output_vector.push(u8_3);
    }

    output_vector.into_iter().filter(|&val| val != 0).collect()
}
