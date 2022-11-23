#[cfg(test)]
mod tests {
    use base64_transcoder::encode;

    #[test]
    fn encode_hello_world() {
        let encoding = encode("Hello World!".as_bytes());
        assert_eq!("SGVsbG8gV29ybGQh", encoding);
    }

    #[test]
    fn encode_light_work() {
        let encoding = encode("light work.".as_bytes());
        assert_eq!("bGlnaHQgd29yay4=", encoding);
    }

    #[test]
    fn encode_light_work_no_dot() {
        let encoding = encode("light work".as_bytes());
        assert_eq!("bGlnaHQgd29yaw==", encoding);
    }

    #[test]
    fn encode_light_wor() {
        let encoding = encode("light wor".as_bytes());
        assert_eq!("bGlnaHQgd29y", encoding);
    }

    #[test]
    fn encode_light_wo() {
        let encoding = encode("light wo".as_bytes());
        assert_eq!("bGlnaHQgd28=", encoding);
    }

    #[test]
    fn encode_light_w() {
        let encoding = encode("light w".as_bytes());
        assert_eq!("bGlnaHQgdw==", encoding);
    }
}
