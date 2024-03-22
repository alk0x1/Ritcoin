use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::{blockchain::Blockchain, transactions::Transaction, wallets_2::Wallet};

pub fn rpc() {
  let blockchain = Arc::new(Mutex::new(Blockchain::new()));
  let mut io = IoHandler::default();

  blockchain_methods(&blockchain, &mut io);
  block_methods(&blockchain, &mut io);
  transaction_methods(&blockchain, &mut io);
  wallet_methods(&mut io);
  // Placeholder for future wallet methods integration.
  // wallet_methods(&blockchain, &mut io);

  let server = ServerBuilder::new(io)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .expect("Unable to start RPC server");

  server.wait();
}

fn block_methods(blockchain: &Arc<Mutex<Blockchain>>, io: &mut IoHandler) {
  let blockchain_clone = blockchain.clone();
  io.add_method("insert_new_block", move |_params| {
    blockchain_clone.lock().unwrap().insert_new_block();
    Ok(Value::String("Block inserted.".into()))
  });

  let blockchain_clone = blockchain.clone();
  io.add_method("get_last_block_hash", move |_params| {
    let hash = blockchain_clone.lock().unwrap().get_last_block_hash();
    Ok(Value::String(hash))
  });

  let blockchain_clone = blockchain.clone();
  io.add_method("show_all_block_hashes", move |_params| {
    blockchain_clone.lock().unwrap().show_all_block_hashes();
    Ok(Value::String("Block hashes displayed.".into()))
  });

  let blockchain_clone = blockchain.clone();
  io.add_method("show_block_info", move |params: Params| {
    let index: usize = match params.parse() {
      Ok(index) => index,
      Err(_) => return Ok(Value::String("Invalid index.".into())),
    };
    blockchain_clone.lock().unwrap().show_block_info(index);
    Ok(Value::String(format!("Info for block {} displayed.", index)))
  });
}

fn transaction_methods(blockchain: &Arc<Mutex<Blockchain>>, io: &mut IoHandler) {
  let blockchain_clone = blockchain.clone();
  io.add_method("insert_transaction_in_pool", move |params: Params| {
    println!("params: {:?}", params);

    // let tx: Transaction = match params.parse() {
    //   Ok(tx) => {println!("tx: {:?}", tx); tx},
    //   Err(_) => return Ok(Value::String("Invalid transaction data.".into())),
    // };

    let mut blockchain_guard = blockchain_clone.lock().unwrap();

    let from = String::from("025d4949b3fe343039904c0b5ba61686db8af0a40ad548dde0b126adbd13e598b6");
    let to = String::from("035d4949b3fe343039904c0b5ba61686db8af0a40ad548dde0b126adbd13e598b6");
    let utxos = &blockchain_guard.utxos;
    let height = blockchain_guard.blocks.len() as u64;

    println!("Accessed UTXOs: {:?}", utxos);

    match Transaction::new(from, to, 3, utxos, height) {
      Ok(transaction) => {
        blockchain_guard.insert_transaction_in_pool(transaction);
        Ok(Value::String("Transaction inserted into pool.".into()))
      },
      Err(e) => Ok(Value::String(format!("Error creating transaction: {}", e))),
    }
  });

  // Placeholder for future transaction-related methods.
}

// Placeholder for future wallet_methods implementation.
fn wallet_methods(io: &mut IoHandler) {
  io.add_method("create_wallet", move |params: Params| {
    // Extract filename or any other necessary parameters from the received params
    let args: (String,) = match params.parse() {
      Ok(args) => args,
      Err(e) => return Ok(Value::String(format!("Error parsing parameters: {}", e))),
    };

    let filename = args.0;

    // Create a new wallet and save it
    let wallet = Wallet::new();
    wallet.save(&filename);

    Ok(Value::String(format!("Wallet created and saved as {}", filename)))
  });
}

fn blockchain_methods(blockchain: &Arc<Mutex<Blockchain>>, io: &mut IoHandler) {
  let blockchain_clone = blockchain.clone();

  io.add_method("get_blockchain_data", move |_params| {
    let blockchain_guard = blockchain_clone.lock().unwrap();
    
    let transactions_pool_header = "Transactions Pool:";
    let transactions_pool = &blockchain_guard.transactions_pool;
    let transactions_info = if transactions_pool.is_empty() {
        String::from("No transactions in the pool.")
    } else {
      transactions_pool.iter().enumerate().map(|(i, tx)| {
        format!("Transaction {}\n  TXID: {}\n  Inputs: {:?}\n  Outputs: {:?}\n", 
                  i + 1, tx.txid, tx.inputs, tx.outputs)
      }).collect::<Vec<String>>().join("\n")
    };

    let utxos_header = "UTXOs Hashmap {";
    let utxos = &blockchain_guard.utxos;
    let utxos_info = if utxos.is_empty() {
        String::from("No UTXOs available.")
    } else {
        utxos.iter().map(|(key, utxo)| {
            format!("  UTXO Key: {}\n    Details:\n      TXID: {}\n      Index: {}\n      Value: {}\n      Script PubKey: {}\n", 
                    key, utxo.txid, utxo.index, utxo.value, utxo.script_pubkey)
        }).collect::<Vec<String>>().join("\n")
    };

    let data = format!("{}\n{}\n\n{}\n{}\n}}", transactions_pool_header, transactions_info, utxos_header, utxos_info);
    
    Ok(Value::String(data))
  });
}
