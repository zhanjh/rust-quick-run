use std::path::Path;

use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{
  runtime::Runtime,
  sync::mpsc::{channel, Receiver},
};

#[tokio::main]
async fn main() -> Result<()> {
  let root = tokio::fs::canonicalize("./src").await?;
  println!("watch {root:?}");
  watch(&root).await?;
  Ok(())
}

fn watcher() -> Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
  let (tx, rx) = channel(1);
  let rt = Runtime::new()?;
  let watcher = RecommendedWatcher::new(
    move |res| {
      rt.block_on(async {
        tx.send(res).await.unwrap();
      })
    },
    Config::default(),
  )?;
  Ok((watcher, rx))
}

async fn watch(path: &Path) -> Result<()> {
  let (mut watcher, mut rx) = watcher()?;
  watcher.watch(path, RecursiveMode::Recursive)?;
  while let Some(res) = rx.recv().await {
    match res {
      Ok(event) => println!("changed: {event:?}"),
      Err(e) => println!("watch error: {e:?}"),
    }
  }
  Ok(())
}
