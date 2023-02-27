use anyhow::Result;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> Result<()> {
  // let dir = tokio::fs::canonicalize
  let dir = tokio::fs::canonicalize(".").await?;
  let path = dir.join("src/async-file.rs");
  let mut file = File::open(&path).await?;
  // let mut buf = BytesMut::new(); // not work
  let mut buf = vec![];
  file.read_to_end(&mut buf).await?;

  let digest = md5::compute(&buf);
  let size = buf.len();
  let content = std::str::from_utf8(&buf);

  println!("path: {path:?}");
  println!("md5: {digest:?}");
  println!("size: {size:?}");
  println!("content: {content:?}");
  Ok(())
}
