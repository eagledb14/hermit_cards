//server

use tokio::{
    net::{TcpStream, TcpListener},
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::{Builder, Runtime},
    time::{sleep, Duration},
};

/*struct Connection {
    p1: Option<TcpStream>,
    p2: Option<TcpStream>
}*/

//#[tokio::main]
fn main() {
    println!("Listening on port 1234");
    let thread_pool = Builder::new_multi_thread()
        .worker_threads(100)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    run_thread(thread_pool);
}

fn run_thread(thread_pool: Runtime) {
    thread_pool.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:1234").await.unwrap();

        loop {

            let (mut stream, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let mut buf = [0; 512];

                stream.read(&mut buf).await.unwrap();

                //stream.write_all(b"received").await.unwrap();

                println!("{:?}: {i}", String::from_utf8_lossy(&buf).replace("\0", "").replace("\"", ""));
            });
        }
    });
}

