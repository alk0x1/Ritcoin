use crate::transactions;
use crate::utils;
extern crate hex;

#[derive(Debug)]
pub struct Header {
	pub version: i32,
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
	pub transactions: Vec<String>
}
// Modify that later to have only a new() method and 
// create the genesis block in Blockchain class
impl Block {
	pub fn new(transactions_counter: usize, transactions: Vec<String>, previous_hash: String, nonce: i32) -> Self {
		let mut header = Header {
			version: 1,
			previous_hash,
			nonce
		};

		Block {
			header,
			transactions_counter,
			transactions
		}
	}
}
