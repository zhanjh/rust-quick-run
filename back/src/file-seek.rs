use std::{io::SeekFrom, path::Path};

use anyhow::Result;
use tokio::{
  fs::File,
  io::{AsyncSeekExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> Result<()> {
  // let mut file = File::create("test.txt").await?;
  let mut file = get_file("test.txt").await?;
  file.seek(SeekFrom::Start(100)).await?;
  file.write_all(b"123,").await?;

  file.seek(SeekFrom::Start(10)).await?;
  file.write_all(b"45678,").await?;

  file.seek(SeekFrom::Start(50)).await?;
  file.write_all(b"901234,").await?;
  Ok(())
}

async fn get_file(path: impl AsRef<Path>) -> Result<File> {
  let full_path = Path::new(path.as_ref());
  if full_path.exists() {
    Ok(
      tokio::fs::OpenOptions::new()
        .write(true)
        .open(full_path)
        .await?,
    )
  } else {
    let parent_dir = full_path.parent().unwrap();
    if !parent_dir.exists() {
      tokio::fs::create_dir_all(parent_dir).await?;
    }
    Ok(tokio::fs::File::create(full_path).await?)
  }
}
