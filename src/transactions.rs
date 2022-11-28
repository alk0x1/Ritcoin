use rand::Rng;
use sha2::digest::{generic_array::GenericArray, typenum::{UInt, UTerm, bit::{B1, B0}}};
extern crate hex;
use crate::utils;


#[derive(Debug)]
pub struct Transaction {
  pub version: usize,
  pub input_counter: usize,
  pub signature: String,
  // inputs: Vec<Inputs>,
  // outputs: Vec<Outputs>,
  // locktime: Date
}

// how to implement locking_script
// basically the script is writted using opcodes.
// but for this I can just use Rust

pub struct Output {
  amount: usize,                  // Bitcoin value in satoshis (10-8 bitcoin)
  locking_script_size: usize,     // Locking-Script length in bytes, to follow
  // locking_script: any          // A script defining the conditions needed to spend the output
}

// The unlocking script is usually a signature proving ownership of the bitcoin address that is in the locking script.
pub struct Input {
  transaction_hash: String,        // Pointer to the transaction containing the UTXO to be spent
  output_index: usize,             // The index number of the UTXO to be spent; first one is 0
  // unlocking_script_size: usize, // Unlocking-Script length in bytes, to follow
  // unlocking_script: any,        // A script that fulfills the conditions of the UTXO locking script.
  sequence_number: usize           // Currently disabled Tx-replacement feature, set to 0xFFFFFFFF
}

impl Transaction {
  pub fn new(version: usize, input_counter: usize, signature: String) -> Self {
    Transaction {
      version,
      input_counter,
      signature
    }
  }
  
  pub fn new_pseudo_hash() -> String {
    let random_number = &[rand::thread_rng().gen()];
		let random_number_converted= std::str::from_utf8(random_number);
		
		let random_number_result = match random_number_converted {
			Ok(random_number_converted) => random_number_converted,
			Err(e) => "error"
		};
		
    if random_number_result != "error" {
      let pseudo_hash = utils::hash(random_number_result);
      let hex_pseudo_hash= hex::encode(pseudo_hash);

      return hex_pseudo_hash;
		} else {
      return Self::new_pseudo_hash();
    }
  }

  pub fn new_transaction_input() {

  } 

  pub fn new_transaction_output() {
    
  }

}