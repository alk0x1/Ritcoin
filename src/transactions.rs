use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::utils;
use std::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
  pub txid: String,
  pub inputs: Vec<Input>,
  pub outputs: Vec<UTXO>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct UTXO {
  pub txid: String,
  pub index: i32,
  pub value: u64,
  pub script_pubkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
  pub txid: String,
  pub vout: u32,
  pub script_sig: String,
}

impl Transaction {
  pub fn new(txid: String, inputs: Vec<Input>, outputs: Vec<UTXO>) -> Self {
    Transaction { txid, inputs, outputs }
  }

  pub fn calculate_txid(&self) -> String {
    let serialized_data = self.serialize();
    hex::encode(utils::hash_Vec_u8(&serialized_data))
  }

  pub fn new_pseudo_hash() -> Result<String, &'static str> {
    let random_number = rand::thread_rng().gen::<[u8; 1]>();
    let random_number_converted = std::str::from_utf8(&random_number);

    match random_number_converted {
      Ok(n) => Ok(hex::encode(utils::hash(n))),
      Err(_) => Err("Failed to generate pseudo hash."),
    }
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut serialized_data = Vec::with_capacity(
      self.txid.len() +
      self.inputs.iter().map(|i| i.serialize().len()).sum::<usize>() +
      self.outputs.iter().map(|o| o.serialize().len()).sum::<usize>()
    );

    serialized_data.extend(self.txid.as_bytes());
    serialized_data.extend(&(self.inputs.len() as u32).to_le_bytes());

    for input in &self.inputs {
      serialized_data.extend(input.serialize());
    }

    serialized_data.extend(&(self.outputs.len() as u32).to_le_bytes());

    for output in &self.outputs {
      serialized_data.extend(output.serialize());
    }

    serialized_data
  }

  pub fn coinbase(txid: String, value: u64, script_pubkey: String) -> Self {
    Transaction {
      txid,
      inputs: Vec::new(),
      outputs: vec![UTXO { txid, index: 0, value, script_pubkey }],
    }
  }
}

impl Input {
  pub fn serialize(&self) -> Vec<u8> {
    [self.txid.as_bytes(), &self.vout.to_le_bytes(), self.script_sig.as_bytes()].concat()
  }
}

impl UTXO {
  pub fn serialize(&self) -> Vec<u8> {
    [self.txid.as_bytes(), &self.index.to_le_bytes(), &self.value.to_le_bytes(), self.script_pubkey.as_bytes()].concat()
  }
}
