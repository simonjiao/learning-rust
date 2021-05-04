use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::digest::{Context, Digest, SHA256};
use ring::rand::SecureRandom;
use ring::{hmac, rand};
use std::fs::File;
use std::io::{BufReader, Read, Write};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Decode(data_encoding::DecodeError);
    }
//ring::error::Unspecified
    errors{
        UnspecifiedError(t:String)
    }
}
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn cal_sha256() -> Result<()> {
    let path = "file.txt";
    let mut file = File::create(path)?;
    write!(file, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("{}", HEXUPPER.encode(digest.as_ref()));
    Ok(())
}

pub fn verify_hmac() -> Result<()> {
    let mut key_value = [0u8, 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)
        .map_err(|e| ErrorKind::UnspecifiedError(e.to_string()))?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and import message";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())
        .map_err(|e| ErrorKind::UnspecifiedError(e.to_string()))?;

    Ok(())
}