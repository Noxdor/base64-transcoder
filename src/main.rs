use base64_transcoder::decode;
use base64_transcoder::encode;

fn main() {
    let test_str = "hhA";
    let res = encode(test_str.as_bytes());

    println!("base 64 encode result: {}", res);

    let res = decode(&res);

    println!("base64 decode result: {:?}", res);
    println!(
        "base64 decode result str: {}",
        std::str::from_utf8(&res).unwrap()
    );
}
