use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::{blockchain::Blockchain, transactions::Transaction};

// Assuming Blockchain, Transaction, and other necessary structs are defined
// and that utils module provides necessary utility functions.

pub fn rpc() {
  let blockchain = Arc::new(Mutex::new(Blockchain::new()));
  let mut io = IoHandler::default();

  {
    let blockchain = blockchain.clone();
    io.add_method("insert_new_block", move |_params| {
      blockchain.lock().unwrap().insert_new_block();
      Ok(Value::String("Block inserted".into()))
    });
  }

  {
    let blockchain = blockchain.clone();
    io.add_method("get_last_block_hash", move |_params| {
      let hash = blockchain.lock().unwrap().get_last_block_hash();
      Ok(Value::String(hash))
    });
  }

  {
    let blockchain = blockchain.clone();
    io.add_method("show_all_block_hashes", move |_params| {
      blockchain.lock().unwrap().show_all_block_hashes();
      Ok(Value::String("Block hashes displayed".into()))
    });
  }

  {
    let blockchain = blockchain.clone();
    io.add_method("show_block_info", move |params: Params| {
      let index: usize = match params.parse() {
        Ok(index) => index,
        Err(_) => return Ok(Value::String("Invalid index".into())),
      };
      blockchain.lock().unwrap().show_block_info(index);
      Ok(Value::String(format!("Info for block {} displayed", index)))
    });
  }

  {
    let blockchain = blockchain.clone();
    io.add_method("insert_transaction_in_pool", move |params: Params| {
      let tx: Transaction = match params.parse() {
        Ok(tx) => tx,
        Err(_) => return Ok(Value::String("Invalid transaction data".into())),
      };
      blockchain.lock().unwrap().insert_transaction_in_pool(tx);
      Ok(Value::String("Transaction inserted into pool".into()))
    });
  }

  let server = ServerBuilder::new(io)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .expect("Unable to start RPC server");

  server.wait();
}
