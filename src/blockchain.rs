use std::collections::HashMap;

use crate::block::{Block, Header};
use crate::transactions::{Transaction, UTXO};
use crate::utils::{self, validated_hash};


pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub transactions_pool: Vec<Transaction>,
  pub utxos: HashMap<String, UTXO>, 
}

impl Blockchain {
  pub fn new() -> Self {
    let mut blockchain = Blockchain {
      blocks: Vec::new(),
      transactions_pool: Vec::new(),
      utxos: HashMap::new(),
    };

    let genesis_block = Blockchain::create_genesis_block();
    blockchain.blocks.push(genesis_block);
    blockchain.update_utxo_set_for_last_block();

    blockchain
  }
 
  pub fn insert_new_block(&mut self) {
    if self.blocks.is_empty() {
      let genesis_block = Blockchain::create_genesis_block();
      self.blocks.push(genesis_block);
      // println!("Genesis block created");
    }

    let version = 1;
    let previous_hash = self.get_last_block_hash();
    let nonce = self.proof_of_work(&previous_hash, version);

    let mut transactions = vec![Blockchain::create_coinbase_transaction(self.blocks.len() as u64)];
    transactions.append(&mut self.transactions_pool);

    let header = Header {
      previous_hash,
      nonce,
      version,
    };

    // println!("previous_hash.clone(): {} | nonce.to_string(): {}", header.previous_hash, header.nonce);

    // let copy_vec: Vec<Transaction> = self.transactions_pool.iter().cloned().collect();
    
    let new_block = Block::new(&header, transactions);

    let new_block_hash = utils::hash_block(&header);
    let block_validated = utils::validated_hash(new_block_hash.clone(), 2, String::from("0"));

    if block_validated {
      self.blocks.push(new_block);
      self.update_utxo_set_for_last_block();
      // println!("Block added with hash: {}", hex::encode(new_block_hash));
    } else {
      // println!("Failed to validate block: {}", hex::encode(new_block_hash));
    }
  }

  pub fn get_last_block_hash(&mut self) -> String {
    let nonce = self.blocks[self.blocks.len() - 1].header.nonce.to_string();
    let previous_hash = self.blocks[self.blocks.len() - 1].header.previous_hash.to_string();
    let last_hash = utils::concat_strings(nonce, previous_hash);

    return hex::encode(utils::hash(&last_hash));
  }

	pub fn create_genesis_block() -> Block {
    let genesis_transaction = Blockchain::create_coinbase_transaction(0);

    let genesis_block_header: Header = Header { 
			version: 1,
			previous_hash: String::from("0"),
      nonce: 0
		};

	  Block { 
			header: genesis_block_header,
			transactions_counter: 0,
			transactions: vec![genesis_transaction]
		}
	}

	fn proof_of_work(&mut self, previous_hash: &String, version: usize) -> i32 {
    let mut nonce: i32 = 0;
    
    loop {
      let prefix = String::from("0");
      let header = &Header { 
   
     version, 
        previous_hash: previous_hash.clone(),
        nonce
      };
      
      let hashed_with_nonce = utils::hash_block(header);

      if validated_hash(hashed_with_nonce.clone(), 3, prefix) {
        // println!("nonce {} validated: {}", nonce, hashed_with_nonce);
        return nonce;
      }
      nonce = nonce + 1;
    }
  }

  pub fn show_all_block_hashes(&mut self) {
    for (i, block) in self.blocks.iter().enumerate() {
      println!("block {}: {}", i, utils::hash_block(&block.header));
    }
  }

  pub fn show_block_info(&mut self, index: usize) {
    if index == 00 {
      println!("Not a valid index {}", index);
    }
    println!("block {}: {:?}", index, self.blocks[index]);
  }

  pub fn get_transactions_in_pool(&mut self) -> Vec<Transaction> {
    self.transactions_pool.clone()
  }

  pub fn show_transaction_info(&mut self, index: usize) {
    if index == 00 {
      println!("Not a valid index {}", index);
    }
    println!("transaction {}: {:?}", index, self.transactions_pool[index]);
  }

  pub fn insert_transaction_in_pool(&mut self, transaction: Transaction) {
    self.transactions_pool.push(transaction);
    println!("transactions_pool: {:?}", self.transactions_pool);
  }

  pub fn update_utxo_set_for_last_block(&mut self) {
  if let Some(last_block) = self.blocks.last() {

    println!("&last_block.transactions {:?}", &last_block.transactions);

    for tx in &last_block.transactions {
      // Process inputs: remove corresponding UTXOs
      for input in &tx.inputs {
        let input_key = format!("{}:{}", input.txid, input.vout);
        self.utxos.remove(&input_key);
        // Debug: Print action
        println!("Removed UTXO for spent input: {}", input_key);
      }

      // Process outputs: add new UTXOs
      for (index, output) in tx.outputs.iter().enumerate() {
        let output_key = format!("{}:{}", tx.txid, index);
        self.utxos.insert(output_key.clone(), output.clone());
        println!("Added UTXO for output: {}", output_key);
      }
    }
  }
}


  pub fn get_utxos(&mut self) -> HashMap<String, UTXO>{
    self.utxos.clone()
  }

  pub fn create_coinbase_transaction(block_height: u64) -> Transaction {
    let reward = 50; 
    let address = "miner_address";
    Transaction::coinbase(block_height, reward, address.to_string())
  }
}
