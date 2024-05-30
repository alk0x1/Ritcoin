extern crate secp256k1;
extern crate rand;
extern crate serde;
use rand::{rngs::OsRng, RngCore};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use std::{collections::HashSet, env, fs::{self, File}, io::Write, path::Path};
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

 pub fn save(&self, filename: &str) {
    let mut dir_path = env::current_dir().unwrap_or_else(|err| {
      eprintln!("Erro ao obter o caminho atual: {:?}", err);
      std::process::exit(1);
    });
    println!("dir_path: {:?}", dir_path);


    dir_path = dir_path.join("wallets");
    fs::create_dir_all(&dir_path).expect("Failed to create wallets directory");
    dir_path = dir_path.join(filename);
    println!("dir_path: {:?}", dir_path);
    
    let public_key_hex = hex::encode(self.public_key.serialize());
    let priv_key_hex = hex::encode(self.secret_key.secret_bytes());

    let wallet_data = serde_json::json!({
      "public_key": public_key_hex,
      "private_key": priv_key_hex,
      "utxos": self.utxos,
    });

    let mut file = File::create(dir_path).expect("Failed to create wallet file");
    writeln!(file, "{}", wallet_data.to_string()).expect("Failed to write wallet data");
  }
}
