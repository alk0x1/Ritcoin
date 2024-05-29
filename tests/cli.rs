use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use std::process::{Command, Child};
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

lazy_static! {
  static ref SERVER: Mutex<Child> = Mutex::new(start_server());
}

fn start_server() -> Child {
  let mut cmd = Command::cargo_bin("ritcoin").unwrap();
  cmd.args(&["blockchain", "--create"]);
  let child = cmd.spawn().expect("Failed to start blockchain");
  // Give the server some time to start
  std::thread::sleep(Duration::from_secs(2));
  child
}

#[cfg(test)]
mod tests {
  use super::*;

  fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
  }

  fn run_command(args: &[&str]) -> assert_cmd::assert::Assert {
    let _server = SERVER.lock().unwrap(); // Ensure server is started
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.args(args);
    let output = cmd.output().expect("Failed to execute command");
    println!("Output: {:?}", output);
    cmd.assert()
  }

  #[tokio::test]
  async fn test_blockchain_create() {
    setup_logger();
    // No need to start server here; it is started globally
  }

  #[tokio::test]
  async fn test_blockchain_show_blocks() {
    setup_logger();
    run_command(&["blockchain", "--show-blocks"])
      .success()
      .stdout(predicate::str::contains("block"));
  }

  #[tokio::test]
  async fn test_blockchain_info() {
    setup_logger();
    run_command(&["blockchain", "--info"])
      .success()
      .stdout(predicate::str::contains("Blockchain data:"));
  }

  #[tokio::test]
  async fn test_block_insert() {
    setup_logger();
    run_command(&["block", "--insert"])
      .success()
      .stdout(predicate::str::contains("Block inserted."));
  }

  #[tokio::test]
  async fn test_block_show_details() {
    setup_logger();
    run_command(&["block", "--show-details", "0"])
      .success()
      .stdout(predicate::str::contains("Info for block 0 displayed."));
  }

  #[tokio::test]
  async fn test_block_list_transactions() {
    setup_logger();
    run_command(&["block", "--list-transactions", "0"])
      .success()
      .stdout(predicate::str::contains("Transaction"));
  }

  #[tokio::test]
  async fn test_transaction_create() {
    setup_logger();
    run_command(&["transaction", "--create", "--from", "sender_public_key", "--to", "recipient_public_key"])
      .success()
      .stdout(predicate::str::contains("Transaction response:"));
  }

  #[tokio::test]
  async fn test_wallet_create() {
    setup_logger();
    run_command(&["wallet", "--create"])
      .success()
      .stdout(predicate::str::contains("Wallet created and saved as"));
  }
}
