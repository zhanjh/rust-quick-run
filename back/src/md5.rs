use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let digest1 = md5::compute(b"123");
  println!("{digest1:?}");

  let mut cxt = md5::Context::new();
  cxt.consume(b"1");
  cxt.consume(b"2");
  cxt.consume(b"3");
  let digest2 = cxt.compute();
  println!("{digest2:?}");

  assert_eq!(digest1, digest2);

  Ok(())
}
