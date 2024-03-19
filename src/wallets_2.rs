extern crate secp256k1;
extern crate rand;
extern crate serde;
use rand::{rngs::OsRng, RngCore};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use std::{collections::HashSet, fs::{self, File}, io::Write, path::Path};
use crate::transactions::{Input, Transaction, UTXO};


#[derive(Debug)]
pub struct Wallet {
  pub secret_key: SecretKey,
  pub public_key: PublicKey,
  pub utxos: HashSet<UTXO>,
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

 pub fn save(&self, filename: &str) {
    let dir_path = Path::new("./wallets");
    fs::create_dir_all(dir_path).expect("Failed to create wallets directory");
    let file_path = dir_path.join(filename);
    let public_key_hex = hex::encode(self.public_key.serialize());
    let priv_key_hex = hex::encode(self.secret_key.secret_bytes());

    let wallet_data = serde_json::json!({
      "public_key": public_key_hex,
      "private_key": priv_key_hex,
      "utxos": self.utxos,
    });

    let mut file = File::create(file_path).expect("Failed to create wallet file");
    writeln!(file, "{}", wallet_data.to_string()).expect("Failed to write wallet data");
  }
}
