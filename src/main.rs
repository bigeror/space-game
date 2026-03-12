use futures_util::{SinkExt, StreamExt};
use tokio::{io::{self, ErrorKind, AsyncReadExt, AsyncWriteExt}, net::TcpListener, task::JoinSet};
use tokio_tungstenite::accept_async;

macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        { println!($($arg)*); }
    };
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let http_server = TcpListener::bind("0.0.0.0:6060").await?;
    let ws_server = TcpListener::bind("0.0.0.0:6061").await?;

    let mut tasks = JoinSet::new();
    tasks.spawn(async move {handle_http_server(http_server).await});
    tasks.spawn(async move {handle_ws_server(ws_server).await});

    _ = tasks.join_all().await;

    Err(io::Error::new(ErrorKind::Other, "Reached an unreachable code"))
}

async fn handle_http_server(server: TcpListener) {
    loop {
        let Ok((mut socket, address)) = server.accept().await
            else {debug_print!("Failed to accept http connection"); return};
        debug_print!("New HTTP connection: {}", address);

        tokio::spawn( async move {
            let mut buffer = [0; 1024];
            let Ok(n) = socket.read(&mut buffer).await else {return};

            let request = String::from_utf8_lossy(&buffer[..n]);
            debug_print!("Received HTTP request: {}", request);

           let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
           let Err(e) = socket.write_all(response).await else {return};
           debug_print!("Got error sending data: {}", e);
        });
    }
}

async fn handle_ws_server(server: TcpListener) {
    loop {
        let Ok((socket, address)) = server.accept().await
            else {debug_print!("Failed to accept websocket connection"); return};
        debug_print!("New WebSocket connection: {}", address);

        tokio::spawn( async move {
            let Ok(ws_stream) = accept_async(socket).await
                else {debug_print!("Failed ws handshade with {}", address); return};

            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(message)) = read.next().await {
                debug_print!("Got new message {} from {}", message.to_string(), address);
                if message.is_text() || message.is_binary() {
                    if let Err(_) = write.send(message.clone()).await
                        {debug_print!("failed to send message {} to {}", message.to_string(), address)}
                }
            }
        });
    }
}




