use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// Rust Tokio Crate 이용 socket server 구현
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Start Server -----");

    // TODO
    let room_name_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, _) = listener.accept().await?;
        let map_clone = room_name_map.clone();
        tokio::spawn(async move {
            process(stream, map_clone).await.unwrap();
        });
    }
}

async fn process(
    mut stream: TcpStream,
    room_name_map: Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), Box<dyn Error>> {
    set_room(&mut stream, room_name_map.clone()).await.unwrap();
    println!("{:?}", room_name_map);
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

async fn set_room(
    stream: &mut TcpStream,
    room_name_map: Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), Box<dyn Error>> {
    let send_message = "Set your name: \n".to_string();
    stream.write_all(send_message.as_bytes()).await.unwrap();

    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf).await.unwrap();
    let buf_str = String::from_utf8(Vec::from(buf))?;

    let name = buf_str.split('\n').next().unwrap().trim().to_string();

    // TODO <ip_rooms : name>
    // room_name_map
    //     .lock()
    //     .unwrap()
    //     .insert(name.clone(), name.clone());

    Ok(())
}
