use std::fmt;
use std::{env, u8};
use std::process::Command;

pub enum AESError {
    InvalidKeySize,
    ExternalLibError,
}

impl fmt::Display for AESError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            AESError::InvalidKeySize=>write!(f,"The required key size in bits is incorrect"),
            AESError::ExternalLibError=>write!(f,"Error while generating the AES Key"),
        }
    }
}

pub fn check_openssl_version()->bool{
    if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        println!("Current installation: {}", version);
        return true;
    }
    eprintln!("No OpenSSL installation found");
    return false;
}

pub fn check_openssl_version_2() -> bool{
    let openssl_version = Command::new("sh")
        .arg("-c")
        .arg("openssl version")
        .output()
        .expect("Can't fetch openssl version");

    let version = String::from_utf8_lossy(openssl_version.stdout.as_ref());
    if !version.as_ref().is_empty() {
        println!("Current installation version: {}", version.as_ref());
        return true;
    }
    eprintln!("No OpenSSL installation found");
    return false;
}

pub fn generate_aes_key(bits: u16) -> Result<Vec<u8>, AESError>{
    let lenght = match bits{
        128 => 16,
        192 => 24,
        256 => 32,
        _ => return Err(AESError::InvalidKeySize)
    };

    let mut buffer = vec![0u8; lenght];

    openssl::rand::rand_bytes(&mut buffer).map_err(|_| AESError::ExternalLibError)?;

    Ok(buffer)
}