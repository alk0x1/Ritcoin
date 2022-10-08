use crate::transactions::Transaction;
// use crate::utils;
extern crate hex;

#[derive(Debug)]
pub struct Header {
	pub version: usize,
	pub previous_hash: String,
	pub nonce: i32
	// timestamp: String,
	// merkle_root: String,
	// difficulty: i32
}
#[derive(Debug)]
pub struct Block {
	pub header: Header,
	// size: f32,
	pub transactions_counter: usize,
	pub transactions: Vec<Transaction>
}
// Modify that later to have only a new() method and 
// create the genesis block in Blockchain class
impl Block {
	pub fn new(header: &Header, transactions: Vec<Transaction>) -> Self {
		let header = Header {
			version: header.version,
			previous_hash: header.previous_hash.clone(),
			nonce: header.nonce
		};

		Block {
			header,
			transactions_counter: 0,
			transactions
		}
	}

}