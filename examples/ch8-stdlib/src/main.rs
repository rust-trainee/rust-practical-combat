use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // 必须显示指定端口, TcpSteam 并不知道这将成为一个 HTTP 请求
    let host = "www.rustinaction.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: www.rustinaction.com")?;
    // 两个换行符表示请求结束
    conn.write_all(b"\r\n\r\n")?;

    // std::io::copy 会把字节流从一个 Reader 写入到一个 Writer 中
    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}
