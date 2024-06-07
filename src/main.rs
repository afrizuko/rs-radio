use std::{fs::File, io::Read};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on: 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    let mut handler = File::open("/Users/olu/recording.mp3").expect("failed to read file");

    loop {
        let n = match handler.read(&mut buf) {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from file {}", e);
                return;
            }
        };

        if let Err(e) = socket.write_all(&buf[0..n]).await {
            eprintln!("failed to write to socket: {}", e);
            return;
        }
    }
}
