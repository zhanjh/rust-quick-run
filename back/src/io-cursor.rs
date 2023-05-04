use std::io::Cursor;

use bytes::{Buf, BufMut, BytesMut};

fn main() {
  let mut buf = BytesMut::new();
  for i in 0..100 {
    buf.put_u32(i as u32);
  }
  let slice = buf.get(0..4);
  println!("{slice:?}");

  let b0 = buf.get_u32();
  let buf_remaining = buf.remaining();
  println!("{b0} - {buf_remaining}");

  let slice = buf.get(0..4);
  println!("{slice:?}");
  println!("{buf:?}");

  let buf_remaining = buf.remaining();
  println!("{buf_remaining}");

  let mut peek = Cursor::new(&buf[..]);
  let b0 = peek.get_u32();
  println!("{b0}");

  let peek_pos = peek.position();
  let buf_len = peek.get_ref().len();
  let peek_remaining = peek.remaining();
  println!("{peek_pos} - {peek_remaining} - {buf_len}");

  peek.set_position(peek_pos + 4 * 5);

  let peek_pos = peek.position();
  let buf_len = peek.get_ref().len();
  let peek_remaining = peek.remaining();
  println!("{peek_pos} - {peek_remaining} - {buf_len}");
}
