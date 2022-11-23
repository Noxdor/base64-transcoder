#[cfg(test)]
mod tests {
    use base64_transcoder::decode;
    // use std::collections::HashMap;

    // fn setup_byte_table() -> HashMap<char, u8> {
    //     let mut byte_table = HashMap::new();
    //     base64_transcoder::setup_byte_table(&mut byte_table);
    //     byte_table
    // }

    #[test]
    fn encode_small() {
        // let byte_table = setup_byte_table();
        let decoding = decode("aGhB");
        assert_eq!("hhA".as_bytes().to_vec(), decoding);
    }

    #[test]
    fn encode_hello_world() {
        // let byte_table = setup_byte_table();
        let decoding = decode("SGVsbG8gV29ybGQh");
        assert_eq!("Hello World!".as_bytes().to_vec(), decoding);
    }

    #[test]
    fn encode_light_work() {
        // let byte_table = setup_byte_table();
        let decoding = decode("bGlnaHQgd29yay4=");
        assert_eq!("light work.".as_bytes().to_vec(), decoding);
    }

    #[test]
    fn encode_light_work_no_dot() {
        // let byte_table = setup_byte_table();
        let decoding = decode("bGlnaHQgd29yaw==");
        assert_eq!("light work".as_bytes().to_vec(), decoding);
    }

    #[test]
    fn encode_light_wor() {
        // let byte_table = setup_byte_table();
        let decoding = decode("bGlnaHQgd29y");
        assert_eq!("light wor".as_bytes().to_vec(), decoding);
    }

    #[test]
    fn encode_light_wo() {
        // let byte_table = setup_byte_table();
        let decoding = decode("bGlnaHQgd28=");
        assert_eq!("light wo".as_bytes().to_vec(), decoding);
    }

    #[test]
    fn encode_light_w() {
        // let byte_table = setup_byte_table();
        let decoding = decode("bGlnaHQgdw==");
        assert_eq!("light w".as_bytes().to_vec(), decoding);
    }
}
