// tests/rpc_tests.rs

use jsonrpc_core::Params;
use jsonrpc_core::futures::executor;
use serde_json::json;
use ritcoin::rpc::{rpc, block_methods, transaction_methods, wallet_methods, blockchain_methods};
use ritcoin::blockchain::Blockchain;
use std::sync::{Arc, Mutex};
use jsonrpc_http_server::jsonrpc_core;
use crate::jsonrpc_core::IoHandler;

#[test]
fn test_insert_new_block() {
  let blockchain = Arc::new(Mutex::new(Blockchain::new()));
  let mut io = IoHandler::default();
  block_methods(&blockchain, &mut io);

  let request = r#"{"jsonrpc":"2.0","method":"insert_new_block","params":[],"id":1}"#;
  let response = io.handle_request_sync(request);

  assert_eq!(response, Some(r#"{"jsonrpc":"2.0","result":"Block inserted.","id":1}"#.to_string()));
}

#[test]
fn test_get_last_block_hash() {
  let blockchain = Arc::new(Mutex::new(Blockchain::new()));
  let mut io = IoHandler::default();
  block_methods(&blockchain, &mut io);

  blockchain.lock().unwrap().insert_new_block(); // ensure there is at least one block

  let request = r#"{"jsonrpc":"2.0","method":"get_last_block_hash","params":[],"id":1}"#;
  let response = io.handle_request_sync(request);

  let expected_hash = blockchain.lock().unwrap().get_last_block_hash();
  assert_eq!(response, Some(format!(r#"{{"jsonrpc":"2.0","result":"{}","id":1}}"#, expected_hash)));
}

#[test]
fn test_create_wallet() {
  let mut io = IoHandler::default();
  wallet_methods(&mut io);

  let params = json!(["test_wallet.json"]).to_string();
  let request = format!(r#"{{"jsonrpc":"2.0","method":"create_wallet","params":{},"id":1}}"#, params);
  let response = io.handle_request_sync(&request);

  assert!(response.is_some());
  assert!(response.unwrap().contains("Wallet created and saved as test_wallet.json"));
}
