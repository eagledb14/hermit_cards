//client

use std::{
    env,
};

use tokio::{
    net::{TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let num = env::args().nth(1).unwrap();
    for i in 0..100 {
        let mut stream = TcpStream::connect("127.0.0.1:1234").await.unwrap();

        //let mut buf = [0; 512];
        stream.write_all(format!("{}", i).as_bytes()).await.unwrap();
        //stream.read(&mut buf).await.unwrap();

        //println!("{:?}", String::from_utf8_lossy(&buf).replace("\0", ""));
    }
    Ok(())
}



