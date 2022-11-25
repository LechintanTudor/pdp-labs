use crate::reponse::Response;
use std::fmt::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_get_request(address: &str, host: &str, path: &str) -> anyhow::Result<Response> {
    let mut socket = TcpStream::connect(address).await?;

    let request = {
        let mut request = String::new();
        write!(request, "GET {path} HTTP/1.1\r\n")?;
        write!(request, "Host: {host}\r\n")?;
        write!(request, "User-Agent: zeul-suprem\r\n")?;
        write!(request, "Accept: text/html\r\n")?;
        write!(request, "\r\n")?;
        request
    };

    socket.write_all(request.as_bytes()).await?;
    socket.shutdown().await?;

    let response_bytes = {
        let mut response_bytes = Vec::new();
        socket.read_to_end(&mut response_bytes).await?;
        response_bytes
    };

    let response_str = String::from_utf8_lossy(&response_bytes);
    Ok(Response::from_str(&response_str)?)
}
