// tests/cli_tests.rs

use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_blockchain_create() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("blockchain")
       .arg("--create");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Block inserted."));
  }

  #[test]
  fn test_blockchain_show_blocks() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("blockchain")
       .arg("--show-blocks");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("block"));
  }

  #[test]
  fn test_blockchain_info() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("blockchain")
       .arg("--info");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Blockchain data:"));
  }

  #[test]
  fn test_block_insert() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("block")
       .arg("--insert");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Block inserted."));
  }

  #[test]
  fn test_block_show_details() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("block")
       .arg("--show-details")
       .arg("0");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Info for block 0 displayed."));
  }

  #[test]
  fn test_block_list_transactions() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("block")
       .arg("--list-transactions")
       .arg("0");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Transaction"));
  }

  #[test]
  fn test_transaction_create() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("transaction")
       .arg("--create")
       .arg("--from")
       .arg("sender_public_key")
       .arg("--to")
       .arg("recipient_public_key");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Transaction response:"));
  }

  #[test]
  fn test_wallet_create() {
    let mut cmd = Command::cargo_bin("ritcoin").unwrap();
    cmd.arg("wallet")
       .arg("--create");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Wallet created and saved as"));
  }
}

