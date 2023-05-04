use anyhow::Result;
use bytes::{BufMut, BytesMut};
use tokio::{
  io::{AsyncWriteExt, BufWriter},
  net::{TcpListener, TcpStream},
};

async fn process(mut stream: TcpStream) -> Result<()> {
  let (_, ws) = stream.split();

  let mut writer = BufWriter::new(ws);

  let content = "hello test is first frame";
  writer.write_u64(content.len() as u64).await?;
  writer.write_all(content.as_bytes()).await?;

  let digits: Vec<String> = (0..10_00).map(|x| x.to_string()).collect();
  let content2 = digits.join("");
  writer.write_u64(content2.len() as u64).await?;
  writer.write_all(content2.as_bytes()).await?;
  writer.flush().await?;

  let mut buf = BytesMut::new();

  let digits3: Vec<String> = (0..100_00).map(|x| x.to_string()).collect();
  let content3 = digits3.join("");
  buf.put_u64(content3.len() as u64);
  buf.put(content3.as_bytes());
  writer.write_all(&buf).await?;
  writer.flush().await?;
  /*
  let mut buf = BytesMut::new();

  for item in 0..10_00 {
    buf.put_u32(item);
  }
  writer.write_u64(buf.len() as u64).await?;
  writer.write_all(&buf).await?;
  writer.flush().await?;

  let mut buf2 = BytesMut::new();
  for item in 0..10_00 {
    buf2.put_u32(item);
  }

  writer.write_u64(buf2.len() as u64).await?;
  writer.write_all(&buf2).await?;
  writer.flush().await?;
  */

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let listener = TcpListener::bind("127.0.0.1:9090").await?;

  loop {
    let (stream, addr) = listener.accept().await?;
    println!("accept addr: {addr}");

    tokio::spawn(async move {
      process(stream).await.unwrap();
    });
  }
}
