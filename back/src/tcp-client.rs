use anyhow::Result;
use bytes::BytesMut;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
  let (rs, _) = stream.split();

  let mut buf = BytesMut::with_capacity(8 * 1024);
  loop {
    rs.readable().await?;

    match rs.try_read_buf(&mut buf) {
      Ok(0) => break,
      Ok(n) => {
        println!("read {n} bytes")
      }
      Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
      Err(e) => return Err(e.into()),
    }
  }

  let buf_len = buf.len();
  println!("{buf_len}");

  Ok(())
}
