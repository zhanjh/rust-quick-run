use anyhow::Result;
use bytes::{Buf, Bytes, BytesMut};
use tokio::net::{tcp::ReadHalf, TcpStream};

struct Receiver<'a> {
  reader: ReadHalf<'a>,
  buffer: BytesMut,
}

impl<'a> Receiver<'a> {
  fn new(reader: ReadHalf<'a>) -> Self {
    Self {
      reader,
      buffer: BytesMut::with_capacity(1024),
    }
  }

  async fn next_frame(&mut self) -> Result<Option<Bytes>> {
    let mut frame_size = 0;
    loop {
      self.reader.readable().await?;

      match self.reader.try_read_buf(&mut self.buffer) {
        Ok(0) => return Ok(None),
        Ok(_n) => {
          if frame_size == 0 {
            frame_size = self.buffer.get_u64() as usize;
          }
          if self.buffer.remaining() < frame_size {
            continue;
          }

          let frame = Bytes::copy_from_slice(&self.buffer[0..frame_size]);
          self.buffer.advance(frame_size);
          return Ok(Some(frame));
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
        Err(e) => return Err(e.into()),
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:9090").await?;
  let (rs, _) = stream.split();

  let mut recv = Receiver::new(rs);
  while let Ok(Some(frame)) = recv.next_frame().await {
    println!("\n-----\n");
    println!("{frame:?}");
  }
  // let frame = recv.next_frame().await?;

  Ok(())
}
