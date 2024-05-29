use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::{blockchain::Blockchain, transactions::Transaction, utils, wallets_2::Wallet};

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

pub fn block_methods(blockchain: &Arc<Mutex<Blockchain>>, io: &mut IoHandler) {
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
  io.add_method("show_block_info", move |params: Params| {
    let index: usize = match params {
      Params::Array(arr) if arr.len() == 1 => {
        if let Value::String(index_str) = &arr[0] {
          index_str.parse::<usize>().unwrap_or_else(|_| usize::MAX)
        } else {
          usize::MAX
        }
      }
      _ => usize::MAX,
    };
    if index == usize::MAX {
      return Ok(Value::String("Invalid index.".into()));
    }

    blockchain_clone.lock().unwrap().show_block_info(index);
    Ok(Value::String(format!("Info for block {} displayed.", index)))
  });


  let blockchain_clone = blockchain.clone();
   io.add_method("show_transactions_in_a_block", move |params: Params| {
    let index: usize = match params {
      Params::Array(ref arr) if arr.len() == 1 => {
        if let Value::String(ref index_str) = arr[0] {
          index_str.parse::<usize>().unwrap_or_else(|_| usize::MAX)
        } else {
          usize::MAX
        }
      }
      _ => usize::MAX,
    };
    if index == usize::MAX {
      return Ok(Value::String("Invalid block index provided.".into()));
    }

    let blockchain_guard = blockchain_clone.lock().unwrap();
    if index < blockchain_guard.blocks.len() {
      let block = &blockchain_guard.blocks[index];
      // Construct the information string for transactions within the block.
      let transactions_info: String = block.transactions.iter().enumerate().map(|(tx_index, tx)| {
        format!("Transaction {} in Block {}:\n  TXID: {}\n  Inputs: {:?}\n  Outputs: {:?}",
                tx_index + 1, index, tx.txid, tx.inputs, tx.outputs)
      }).collect::<Vec<String>>().join("\n\n");

      Ok(Value::String(transactions_info))
    } else {
      Ok(Value::String(format!("Block with index {} not found.", index)))
    }
  });
}

pub fn transaction_methods(blockchain: &Arc<Mutex<Blockchain>>, io: &mut IoHandler) {
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
pub fn wallet_methods(io: &mut IoHandler) {
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

pub fn blockchain_methods(blockchain: &Arc<Mutex<Blockchain>>, io: &mut IoHandler) {
  io.add_method("get_blockchain_data", {
    let blockchain_clone = blockchain.clone();
    move |_params| {
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
  }});

  io.add_method("show_all_block_hashes", {
    let blockchain_clone = blockchain.clone(); // Clone again for this closure
    move |_params| {
      let blockchain_guard = blockchain_clone.lock().unwrap();
        let blocks_info: String = blockchain_guard.blocks.iter().enumerate().map(|(index, block)| {
          let block_hash = utils::hash_block(&block.header);
          format!("block {}: {}", index, block_hash)
        }).collect::<Vec<String>>().join("\n");
        Ok(Value::String(blocks_info))
    }
  });

   io.add_method("show_transactions_pool", {
    let blockchain_clone = blockchain.clone();
    move |_params: Params| {
      let transactions = blockchain_clone.lock().unwrap().get_transactions_in_pool();
      let transactions_json: Vec<Value> = transactions.iter().map(|tx| serde_json::to_value(tx).unwrap()).collect();
      Ok(Value::Array(transactions_json))
    }
  });

}
