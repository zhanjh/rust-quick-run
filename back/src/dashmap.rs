use dashmap::DashMap;

fn main() {
  let map = DashMap::new();
  map.insert("a".to_string(), "abc".to_string());
  map.insert("b1".to_string(), "abcf".to_string());
  map.insert("c12".to_string(), "abcf".to_string());
  map.insert("d123".to_string(), "abcd".to_string());

  // let mut keys = vec![];
  for item in map.iter() {
    let key = item.key();
    let val = item.value();
    // keys.push(key.clone());
    println!("{key:?} - {val:?}");
  }

  let mut keys = map
    .iter()
    .map(|item| item.key().to_string())
    .collect::<Vec<String>>();

  let mut keys: Vec<String> = map.iter().map(|item| item.key().to_string()).collect();

  keys.sort();

  for key in keys {
    println!("{key}");
  }

  // println!("{keys:?}");
}
