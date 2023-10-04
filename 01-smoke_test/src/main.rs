use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<MyService, shuttle_runtime::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            echo_service(socket).await;
        });
    }
}

// Customize this struct with things from `shuttle_main` needed in `bind`,
// such as secrets or database connections
struct MyService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for MyService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start your service and bind to the socket address
        Ok(())
    }
}

async fn echo_service(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        let buffer_used = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        if let Err(e) = socket.write_all(&buffer[0..buffer_used]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}
