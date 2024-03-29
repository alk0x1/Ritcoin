use rsa::rand_core::block;
use serde::{Deserialize, Serialize};
use crate::utils;
use std::{collections::HashMap, result::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
  pub txid: String,
  pub inputs: Vec<Input>,
  pub outputs: Vec<UTXO>,
  pub block_height: u64,

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
  pub vout: i32,
  pub script_sig: String,
}

impl Transaction {
  pub fn new(from: String, to: String, value: u64, utxos: &HashMap<String, UTXO>, block_height: u64) -> Result<Self, &'static str> {
    let (total_value, inputs) = utxos.iter()
      .filter(|(_, utxo)| utxo.script_pubkey == from)
      .fold((0_u64, Vec::new()), |(acc_value, mut acc_inputs), (_, utxo)| {
        if acc_value < value {
          acc_inputs.push(Input::new(utxo.txid.clone(), utxo.index, "signature_placeholder".to_string()));
        }
        (acc_value + utxo.value, acc_inputs)
      });

    if total_value < value {
      return Err("Not enough balance");
    }

    let outputs = if total_value > value {
      vec![UTXO::new(
        utils::double_sha256(b"temporary placeholder"),
        0,
        value,
        to.clone(),
      )]
    } else {
      Vec::new()
    };

    let tx_data = [from, to].join("");
    let txid = utils::double_sha256(tx_data.as_bytes());

    Ok(Transaction {
      txid,
      inputs,
      outputs,
      block_height,
    })
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

  pub fn coinbase(block_height: u64, value: u64, script_pubkey: String) -> Self {
    let outputs = vec![UTXO {
        txid: String::new(), // This will be updated after the txid is calculated.
        index: 0,
        value,
        script_pubkey,
    }];

    let mut transaction = Transaction {
        block_height,
        txid: String::new(), // Placeholder, to be calculated.
        inputs: Vec::new(), // No inputs for coinbase.
        outputs,
    };

    transaction.txid = transaction.calculate_txid(block_height);

    println!("transaction.txid: {}", transaction.txid);

    for output in transaction.outputs.iter_mut() {
      output.txid = transaction.txid.clone();
    }

    println!("transaction-return: {:?}", transaction);
    transaction
  }

  pub fn calculate_txid(&self, block_height: u64) -> String {
    let mut serialized = serde_json::to_string(&self).expect("Transaction serialization failed");
    
    let height = block_height.to_string();
    serialized.push_str(&height);

    println!("utils::double_sha256(serialized.as_bytes()): {:?}", utils::double_sha256(serialized.as_bytes()));
    utils::double_sha256(serialized.as_bytes())
  }
}

impl Input {
  pub fn new (txid: String, vout: i32, script_sig: String) -> Self {
    Input {
      txid,
      vout,
      script_sig,
    }
  }

  pub fn serialize(&self) -> Vec<u8> {
    [self.txid.as_bytes(), &self.vout.to_le_bytes(), self.script_sig.as_bytes()].concat()
  }
}

impl UTXO {
  pub fn new (txid: String, index: i32, value: u64, script_pubkey: String) -> Self {
    UTXO {
      txid,
      index,
      value,
      script_pubkey
    }
  }
  pub fn serialize(&self) -> Vec<u8> {
    [self.txid.as_bytes(), &self.index.to_le_bytes(), &self.value.to_le_bytes(), self.script_pubkey.as_bytes()].concat()
  }
}
