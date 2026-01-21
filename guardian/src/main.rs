use tokio::{self, io::{AsyncReadExt, AsyncWriteExt}, net::{UnixListener, UnixStream}};
use std::{env, fs, path::PathBuf};

async fn send_message(stream: &mut UnixStream, message_to_send: &str)-> u8{
    if let Err(_) = stream.write_all(message_to_send.as_bytes()).await {
        return 0;
    }
    return 1;
}

// Create Unix domain socket
#[tokio::main]
async fn main() -> std::io::Result<()>{

    // Get the path needed to create the socket
    let runtime_path: String = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set");
    let socket_path: PathBuf = PathBuf::from(runtime_path).join("aegis_bridge_guardian.sock");

    if socket_path.exists(){
        fs::remove_file(&socket_path)?;
    }

    let listener: UnixListener = UnixListener::bind(&socket_path)?;
    println!("Listening on: {}", socket_path.display());

    while let Ok((mut stream, _)) = listener.accept().await {
        println!("New client connected");
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop{
                match stream.read(&mut buffer).await{
                    Ok(0) => {
                        println!("Client disconnected");
                        break;
                    }
                    Ok(n)=>{
                        let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);
                        println!("Client: {}", message);

                        match message.as_ref(){
                            "hello" => {
                                send_message(&mut stream, "Hello from server").await;
                            }
                            "ping" => {
                                send_message(&mut stream, "pong").await;
                            }
                            "finish" => {
                                let result: Result<(), std::io::Error> = stream.shutdown().await;
                                match result{
                                    Ok(_)=> {
                                        println!("The socket is shutting down");
                                        break;
                                    }
                                    Err(error) => {
                                        eprintln!("The socket can't be shut down {}", error)
                                    }
                                }
                            }
                            _ => {
                                send_message(&mut stream, "Not a valid option").await;
                            }

                        }
                    }
                    Err(e) =>{
                        eprintln!("Stream error {}", e);
                        break;
                    }
                }
            }

        });
    }

    Ok(())
}