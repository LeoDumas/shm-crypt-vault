use std::{env, fs, path::PathBuf};
use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

mod cryptography;
use cryptography::cipher;

async fn send_message(stream: &mut UnixStream, message_to_send: &str) -> u8 {
    if (stream.write_all(message_to_send.as_bytes()).await).is_err() {
        return 0;
    }
    1
}

async fn command_dispatcher(stream: &mut UnixStream, command: String) -> Result<(), Box<cipher::CipherError>> {
    let mut parts = command.split_whitespace();

    if let Some(cmd) = parts.next() {
        match cmd {
            "gen_key" => {
                let mut algo = "aes";
                let mut size: u16 = 256;
                while let Some(part) = parts.next() {
                    match part {
                        "-algo" => {
                            if let Some(value) = parts.next() {
                                println!("Setting algorithm to: {}", value);
                                algo = value;
                            } else {
                                println!("Error: -algo requires a value");
                            }
                        }
                        "-size" => {
                            if let Some(value) = parts.next() {
                                println!("Setting size to: {}", value);
                                size = value.parse().unwrap();
                            } else {
                                println!("Error: -size requires a value");
                            }
                        }
                        _ => {
                            if part.starts_with('-') {
                                println!("Unknown flag: {}", part);
                            } else {
                                println!("Found positional value: {}", part);
                            }
                        }
                    }
                }
                let res = cipher::generate_key(algo, size)?;
                let value_string = cipher::convert_hex_to_string(res.as_ref());
                send_message(stream, &format!("Key: {}", value_string)).await;
            }
            _ => println!("Unknown command: {}", cmd),
        }
    }
    Ok(())
}

// Create Unix domain socket
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // if !cipher::check_openssl_version() || !cipher::check_openssl_version_2(){
    //     println!("OpenSSL isn't installed in the Guardian part");
    //     return Ok(())
    // }

    // Get the path needed to create the socket
    let runtime_path: String = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
    let socket_path: PathBuf = PathBuf::from(runtime_path).join("aegis_bridge_guardian.sock");

    if socket_path.exists() {
        fs::remove_file(&socket_path)?;
    }

    let listener: UnixListener = UnixListener::bind(&socket_path)?;
    println!("Listening on: {}", socket_path.display());

    while let Ok((mut stream, _)) = listener.accept().await {
        println!("New client connected");
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Client disconnected");
                        break;
                    }
                    Ok(n) => {
                        let message: std::borrow::Cow<'_, str> =
                            String::from_utf8_lossy(&buffer[..n]);
                        println!("Client: {}", message);

                        let _ = command_dispatcher(&mut stream, message.to_string()).await;

                    }
                    Err(e) => {
                        eprintln!("Stream error {}", e);
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}
