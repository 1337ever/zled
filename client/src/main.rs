use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
};
use std::error::Error;

mod font;
//mod render;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:1337").await?;
    let res = stream.write(b"rvWinsize").await;
    println!("wrote to stream: {:?}", res.is_ok());
    let res = stream.write(b"rfTest").await;
    println!("wrote to stream: {:?}", res.is_ok());
    //render::run();

    Ok(())
}

async fn sendw(stream: &mut TcpStream, msg: &str) -> Result<(), Box<dyn Error>> {
    let res = stream.write(msg.as_bytes()).await;
    let mut buf = [0u8; 1024];
    loop {
        let n = match stream.read(&mut buf).await {
            //if socket closed
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket: {:?}", e);
                return;
            }
        };
    }

    Ok(())
}
