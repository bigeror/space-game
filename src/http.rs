use std::env;

use tokio::{fs, io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

use crate::debug_print;

/// tcp server to ship static files, *hopefully safe*
pub async fn handle_http_server(server: TcpListener) {
    let client_folder_path = env::current_exe().expect("failed to get path of current executable")
        .parent().expect("failed to get path to executable parent").join("client").to_str()
        .expect("Couldn't cast client folder to string").to_string();

    loop {
        let client_folder = client_folder_path.clone();
        let Ok((mut socket, address)) = server.accept().await
            else {debug_print!("Failed to accept http connection"); return};
        debug_print!("New HTTP connection: {}", address);

        tokio::spawn( async move {
            let mut buffer = [0; 1024];
            let Ok(n) = socket.read(&mut buffer).await else {return};

            let request = String::from_utf8_lossy(&buffer[..n]);
            debug_print!("Received HTTP request: {}", request);

            let request_line = request.lines().next().unwrap_or("");
            let mut iterator = request_line.split_whitespace();
            if iterator.next() != Some("GET") {return} // only GET requests are allowed
            let path = match iterator.next() {
                Some(path) => path,
                None => "/"
            };
            if path.get(0..1) != Some("/") || path.contains("..") {
                response(&mut socket, 403, "text/plain", b"403 Forbidden").await;
                return
            };

            let file_path = if path == "/" { "/index.html" } else { path };
            let file_path = format!("{}{}", client_folder, file_path);
            debug_print!("{}", file_path);

            match fs::read(&file_path).await {
                Ok(contents) => {
                    let mime = mime_type(&file_path);
                    response(&mut socket, 200, mime, &contents).await;
                }
                Err(_) => response(&mut socket, 404, "text/plain", b"404 Not Found").await
            }
        });
    }
}

async fn response(socket: &mut TcpStream, status: u16, content_type: &str, body: &[u8]) {
    let status_text = match status {
        200 => "OK",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "Internal Server Error",
    };

    let header = format!(
        "HTTP/1.1 {status} {status_text}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );

    let _ = socket.write_all(header.as_bytes()).await;
    let _ = socket.write_all(body).await;
}

/// Map file extensions to MIME types
fn mime_type(path: &str) -> &'static str {
    if path.ends_with(".html") { "text/html" }
    else if path.ends_with(".css")  { "text/css" }
    else if path.ends_with(".js")   { "application/javascript" }
    else if path.ends_with(".json") || path.ends_with(".json") { "application/json" }
    else if path.ends_with(".png")  { "image/png" }
    else if path.ends_with(".jpg") || path.ends_with(".jpeg") { "image/jpeg" }
    else if path.ends_with(".svg")  { "image/svg+xml" }
    else if path.ends_with(".ico")  { "image/x-icon" }
    else { "application/octet-stream" }
}
