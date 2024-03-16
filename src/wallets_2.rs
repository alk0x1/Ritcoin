extern crate secp256k1;
extern crate rand;
extern crate serde_json;

use rand::{rngs::OsRng, RngCore};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use std::collections::HashSet;

use crate::transactions::{Input, Transaction, UTXO};

pub struct Wallet {
  secret_key: SecretKey,
  public_key: PublicKey,
  utxos: HashSet<UTXO>,
}

impl Wallet {
  pub fn new() -> Self {
    let secp = Secp256k1::new();  // Creating a new secp256k1 context; used for signing and verification

    let mut rng = OsRng;
    let mut key_data = [0u8; 32];
    rng.fill_bytes(&mut key_data);

    let secret_key = match SecretKey::from_slice(&key_data) {
      Ok(key) => key,
      Err(_) => panic!("Invalid key data"),
    };

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    println!("Secret Key: {:?}", secret_key);
    Wallet {
      secret_key,
      public_key,
      utxos: HashSet::new(),
    }
  }
  
  pub fn add_utxo(&mut self, utxo: UTXO) {
    self.utxos.insert(utxo);
  }

  pub fn remove_utxo(&mut self, utxo: &UTXO) {
    self.utxos.remove(utxo);
  }

  pub fn create_transaction(&self, recipient_script_pubkey: String, amount: u64) -> Transaction {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut total_amount = 0_u64;

    for utxo in &self.utxos {
      if total_amount >= amount {
        break;
      }
      let input = Input {
        txid: utxo.txid.clone(),
        vout: utxo.index as u32,
        script_sig: "signature_placeholder".to_string(),
      };
      inputs.push(input);
      total_amount += utxo.value;
    }

    if total_amount < amount {
      panic!("Not enough funds to create transaction.");
    }

    let recipient_output = UTXO {
      txid: Transaction::new_pseudo_hash(),  // Generating a pseudo txid for illustration
      index: 0,
      value: amount,
      script_pubkey: recipient_script_pubkey.clone(),
    };
    outputs.push(recipient_output);

    // Include a change output if necessary
    if total_amount > amount {
      let change_output = UTXO {
        txid: Transaction::new_pseudo_hash(),  // Generating a pseudo txid for illustration
        index: 1,
        value: total_amount - amount,
        script_pubkey: "change_address_script_placeholder".to_string(),
      };
      outputs.push(change_output);
    }

    Transaction::new(Transaction::new_pseudo_hash(), inputs, outputs)
  }

  // Add more functionalities like adding UTXOs, creating transactions, etc., here
}
