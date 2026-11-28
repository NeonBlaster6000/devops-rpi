use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};
pub fn run(port: u16) -> i32 {
    let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) else {
        return 1;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));

    let req = b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    if stream.write_all(req).is_err() {
        return 1;
    }

    let mut resp = String::new();
    if stream.read_to_string(&mut resp).is_err() {
        return 1;
    }

    if resp.starts_with("HTTP/1.1 200") { 0 } else { 1 }
}
