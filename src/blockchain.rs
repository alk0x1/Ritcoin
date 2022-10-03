use crate::block::{Block, Header};
use crate::utils::{self, validated_hash, hash};

pub struct Blockchain {
  pub blocks: Vec<Block>
}

// create a function to hash a entire block
impl Blockchain {
  pub fn new() -> Self {
    Blockchain { 
      blocks: vec![],
    }
    
  }
  pub fn insert_new_block(&mut self) {
    if self.blocks.len() < 1 {
      let genesis_block = self.create_genesis_block();
      self.blocks.push(genesis_block);
      // change that message to try catch later
      println!("Genesis block created");
    }

    let previous_hash = self.get_last_hash();
    let mut test_transactions_vec: Vec<String> = Vec::new();
    test_transactions_vec.push(String::from("0"));
    let nonce = self.proof_of_work(previous_hash.clone());

    let to_be_hashed = utils::concat_strings(previous_hash.clone(), String::from(nonce.to_string()));
    println!("previous_hash.clone(): {} | nonce.to_string(): {}", previous_hash.clone(), nonce.to_string());

    let new_block = Block::new(0, test_transactions_vec, previous_hash, nonce);
    let new_block_hash = hex::encode(utils::hash(&to_be_hashed));
    let block_validated = utils::validated_hash(new_block_hash.clone(), 3, String::from("0"));

    if block_validated {
      self.blocks.push(new_block);
    } else {
      println!("Failed to validate block: {}", new_block_hash);
    }
  }

  pub fn get_last_hash(&mut self) -> String {
    let nonce = self.blocks[self.blocks.len() - 1].header.nonce.to_string();
    let previous_hash = self.blocks[self.blocks.len() - 1].header.previous_hash.to_string();
    let last_hash = utils::concat_strings(nonce, previous_hash);

    return hex::encode(utils::hash(&last_hash));
  }

	pub fn create_genesis_block(&mut self) -> Block {
		let mut fake_transactions: Vec<String> = Vec::new();
    fake_transactions.push(String::from("a"));

    let genesis_block_header: Header = Header { 
			version: 1,
			previous_hash: String::from("00"),
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
  
	pub fn proof_of_work(&mut self, previous_hash: String) -> i32 {
		let mut nonce: i32 = 0;

    loop {
      let prefix = String::from("0");
      let nonce_string = String::from(nonce.to_string());

      let to_be_hashed_with_nonce = utils::concat_strings(previous_hash.clone(), nonce_string);

      let hashed_with_nonce = hex::encode(utils::hash(&to_be_hashed_with_nonce));
     
      if validated_hash(hashed_with_nonce.clone(), 3, prefix) {
        println!("nonce {} validated: {}", nonce, hashed_with_nonce);
        return nonce;
      }
      nonce = nonce + 1;
    }
  }

}