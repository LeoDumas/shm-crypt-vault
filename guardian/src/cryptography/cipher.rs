use std::fmt;
use std::process::Command;
use std::{env};

pub enum CipherError{
    InvalidKeySize,
    ExternalLibError,
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::InvalidKeySize => write!(f, "The required key size in bits is incorrect"),
            CipherError::ExternalLibError => write!(f, "External library error"),
        }
    }
}

pub fn check_openssl_version() -> bool {
    if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        println!("Current installation: {}", version);
        return true;
    }
    eprintln!("No OpenSSL installation found");
    false
}

pub fn check_openssl_version_2() -> bool {
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
    false
}

pub fn generate_aes_key(bits: u16) -> Result<Vec<u8>, CipherError> {
    let lenght = match bits {
        128 => 16,
        192 => 24,
        256 => 32,
        _ => return Err(CipherError::InvalidKeySize),
    };

    let mut buffer = vec![0u8; lenght];

    openssl::rand::rand_bytes(&mut buffer).map_err(|_| CipherError::ExternalLibError)?;

    Ok(buffer)
}

// pub fn generate_random_iv() -> Result<Vec<u8>, CipherError> {
//     let mut iv = vec![0u8, 32];
//     openssl::rand::rand_bytes(&mut iv).map_err(|_| CipherError::ExternalLibError)?;

//     Ok(iv)
// }

pub fn convert_hex_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
