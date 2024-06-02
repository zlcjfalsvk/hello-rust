use std::collections::HashMap;
use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// Rust Tokio Crate 이용 socket server 구현
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Start Server -----");

    // TODO
    let mut room_name_map: Box<HashMap<&str, &str>> = Box::new(HashMap::new());

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(stream).await.unwrap();
        });
    }
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    set_room(&mut stream).await.unwrap();
    loop {
        let mut buf = [0; 1024];
        let n = match stream.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return Ok(());
            }
        };
        let bur_str = String::from_utf8(Vec::from(buf))?;
        let message = bur_str.split('\n').next().unwrap();

        if let Err(e) = stream.write_all(&buf[0..n]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return Ok(());
        } else {
            // let send_message = format!("Response message: {}\n", message);
            // stream.write_all(send_message.as_bytes()).await.unwrap();
        }
    }
}

async fn set_room(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let send_message = "Set your name: \n".to_string();
    stream.write_all(send_message.as_bytes()).await.unwrap();

    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf).await.unwrap();
    let buf_str = String::from_utf8(Vec::from(buf))?;
    let name = buf_str.split('\n').next().unwrap();

    Ok(())
}
