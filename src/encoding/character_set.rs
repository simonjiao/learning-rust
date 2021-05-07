use error_chain::error_chain;
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use url::form_urlencoded::{byte_serialize, parse};
use data_encoding::HEXUPPER;
use std::str;

error_chain! {
    foreign_links {
        Utf8Err(std::str::Utf8Error);
        HexDecodeErr(data_encoding::DecodeError);
        Base64DecodeErr(base64::DecodeError);
    }
}

pub fn run() {
    percent_encoding_string().unwrap();
    urlencoding_string();
    hex_decode_encode().unwrap();
    base64_code().unwrap();
}

fn base64_code() -> Result<()> {
    println!("====base64 encode and decode======");
    let hello = b"hello rustaceans";
    let encoded = base64::encode(hello);
    let decoded = base64::decode(&encoded)?;

    println!("\torigin: {}", str::from_utf8(hello)?);
    println!("\tbase64 encoded: {}", encoded);
    println!("\tback to origin: {}", str::from_utf8(&decoded)?);
    Ok(())
}

fn hex_decode_encode() -> Result<()> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);
    Ok(())
}

fn urlencoding_string() {
    let urlecoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlecoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded {}", urlecoded);

    let decoded: String = parse(urlecoded.as_bytes()).map(|(key, value)|[key, value].concat()).collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded: {}", decoded);
}

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'?');

fn percent_encoding_string() -> Result<()> {
    let input = "confident, productive systems programming?";

    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming%3F");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming?");

    Ok(())
}