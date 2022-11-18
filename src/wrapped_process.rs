#![allow(dead_code)]

use std::io;

use tokio::process::{Child, Command};
use tokio::sync::{Mutex, MutexGuard};

pub struct WrappedProcess {
  command: Mutex<Command>,
  child: Mutex<Option<Child>>,
}

impl WrappedProcess {
  pub fn new(c: Command) -> WrappedProcess{
    WrappedProcess{ command: Mutex::new(c), child: Mutex::new(None) }
  }

  //Return a lock on the child process after verifying its status
  pub async fn process(&self) -> io::Result<MutexGuard<Option<Child>>>{
    let mut child = self.child.lock().await;
    if let Some(process) = &mut *child {
      if let Some(_status) = process.try_wait()? {
        *child = None;
      }
    }
    Ok(child)
  }

  pub async fn start(&self) -> io::Result<()> {
    let mut child = self.process().await?;
    if let None = &*child {
      *child = Some(self.command.lock().await.spawn()?);
    }
    Ok(())
  }

  pub async fn kill(&self) -> io::Result<()> {
    let mut child = self.process().await?;
    if let Some(process) = &mut *child {
      process.kill().await?;
      *child = None;
    }
    Ok(())
  }
}