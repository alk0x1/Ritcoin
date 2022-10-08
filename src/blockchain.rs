use crate::block::{Block, Header};
use crate::transactions::Transaction;
use crate::utils::{self, validated_hash};

/*                TODO 
* - create a function to hash a entire block ✔️
* - implement command to print all block hashes in the blockchain ✔️
* - create a method to add transactions in a block
* - improve the types of the structure fields to limit the bytes
*/
pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub transactions_pool: Vec<Transaction>
}

impl Blockchain {
  pub fn new() -> Self {
    Blockchain { 
      blocks: Vec::new(),
      transactions_pool: Vec::new()
    }
  }

  pub fn insert_new_block(&mut self) {
    if self.blocks.len() < 1 {
      let genesis_block = self.create_genesis_block();
      self.blocks.push(genesis_block);
      // change that message to try catch later
      println!("Genesis block created");
    }

    let previous_hash = self.get_last_block_hash();
    // let mut test_transactions_vec: Vec<String> = Vec::new();
    // test_transactions_vec.push(String::from("0"));
    let version = 1;
    let nonce = self.proof_of_work(previous_hash.clone(), version.clone());

    let header = &Header {
      previous_hash: previous_hash.clone(),
      nonce,
      version
    };

    println!("previous_hash.clone(): {} | nonce.to_string(): {}", previous_hash.clone(), nonce.to_string());

    // let new_block = Block::new(header, self.transactions_pool);

    let mut copy_vec: Vec<Transaction> = Vec::new();
    let mut i = 0;
    while i < self.transactions_pool.len() {
      let copy_transaction: Transaction = Transaction {
        input_counter: self.transactions_pool[i].input_counter,
        signature: self.transactions_pool[i].signature.clone(),
        version: self.transactions_pool[i].version
      };
      copy_vec.push(copy_transaction);

      i = i + 1;
    }
    
    let new_block = Block::new(header, copy_vec);
    let new_block_hash = utils::hash_block(header);
    let block_validated = utils::validated_hash(new_block_hash.clone(), 2, String::from("0"));

    if block_validated {
      self.blocks.push(new_block);
    } else {
      println!("Failed to validate block: {}", new_block_hash);
    }
  }

  pub fn get_last_block_hash(&mut self) -> String {
    let nonce = self.blocks[self.blocks.len() - 1].header.nonce.to_string();
    let previous_hash = self.blocks[self.blocks.len() - 1].header.previous_hash.to_string();
    let last_hash = utils::concat_strings(nonce, previous_hash);

    return hex::encode(utils::hash(&last_hash));
  }

	pub fn create_genesis_block(&mut self) -> Block {
		let mut fake_transactions: Vec<Transaction> = Vec::new();
    let new_fake_transaction: Transaction = Transaction {
      version: 1,
      input_counter: 0,
      signature: String::from("")
    };

    fake_transactions.push(new_fake_transaction);

    let genesis_block_header: Header = Header { 
			version: 1,
			previous_hash: String::from(""),
      nonce: 1
			// timestamp: String::from("00"),
			// merkle_root: String::from("00"),
			// difficulty: 1,
		};
		let genesis_block: Block = Block { 
			header: genesis_block_header,
			// size: 0.0,
			transactions_counter: 0,
			transactions: fake_transactions
		};

		return genesis_block;
	}
  
	pub fn proof_of_work(&mut self, previous_hash: String, version: usize) -> i32 {
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
        println!("nonce {} validated: {}", nonce, hashed_with_nonce);
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


  pub fn show_all_transactions(&mut self) {
    for (i, transaction) in self.transactions_pool.iter().enumerate() {
      println!("transaction {}: {}", i, &transaction.signature);
    }
  }

  pub fn show_transaction_info(&mut self, index: usize) {
    if index == 00 {
      println!("Not a valid index {}", index);
    }
    println!("transaction {}: {:?}", index, self.transactions_pool[index]);
  }

}