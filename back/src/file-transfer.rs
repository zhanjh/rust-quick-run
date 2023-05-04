use anyhow::Result;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> Result<()> {
  let dir = tokio::fs::canonicalize(".").await?;
  let path = dir.join("src/file-transfer.rs");
  let file = File::open(&path).await?;
  let meta = file.metadata().await?;
  let size = meta.len();
  println!("{meta:?}");
  println!("file size: {size:?}");
  // let s = String::new();
  // tokio::io::copy();
  // std::mem::replace();
  Ok(())
}
