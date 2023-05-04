use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
  attr: String,
  point: f32,
}

#[tokio::main]
async fn main() -> Result<()> {
  let test = Entity {
    attr: "string attr".to_string(),
    point: 0.123,
  };

  let encoded: Vec<u8> = bincode::serialize(&test).unwrap();
  let size = encoded.len();
  println!("size: {size:?}, encoded: {encoded:?}");

  let decoded: Entity = bincode::deserialize(&encoded).unwrap();
  println!("{decoded:?}");
  Ok(())
}
