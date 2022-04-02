use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

error_chain! {
    errors {
        Unspecified(t:String)
    }
}

pub fn salt_hash_passwd() -> Result<()> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)
        .map_err(|e| ErrorKind::Unspecified(e.to_string()))?;

    let passwd = "Guess me if you can";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        passwd.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("{}", HEXUPPER.encode(&salt));
    println!("pbkdf2 {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        passwd.as_bytes(),
        &pbkdf2_hash,
    );

    let wrong_passwd = "Definitely not the correct passwd";
    let should_failed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_passwd.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_failed.is_ok());

    Ok(())
}
