use anyhow::Result;
use bytes::{Bytes, BytesMut};
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> Result<()> {
  // let dir = tokio::fs::canonicalize
  let dir = tokio::fs::canonicalize(".").await?;
  let path = dir.join("src/async-file.rs");
  let mut file = File::open(&path).await?;
  let mut buf = BytesMut::with_capacity(10);

  loop {
    match file.read_buf(&mut buf).await? {
      0 => break,
      _ => continue,
    }
  }

  let digest = md5::compute(&buf);
  let size = buf.len();
  let content = std::str::from_utf8(&buf);

  println!("path: {path:?}");
  println!("md5: {digest:?}");
  println!("size: {size:?}");
  println!("content: {content:?}");

  let bytes: Bytes = buf.into();

  let bytes_digest = md5::compute(&bytes);
  let bytes_size = bytes.len();
  let bytes_content = std::str::from_utf8(&bytes);

  println!("bytes md5: {bytes_digest:?}");
  println!("bytes size: {bytes_size:?}");
  println!("bytes content: {bytes_content:?}");
  Ok(())
}
