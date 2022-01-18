use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let (mut rd, mut wd) = socket.split();

            if io::copy(&mut rd, &mut wd).await.is_err() {
                eprintln!("failed to copy")
            }
        });
    }
}