use rand::Rng;
extern crate hex;
use crate::utils;

#[derive(Debug, Clone)]
pub struct Transaction {
  pub txid: String,
  pub inputs: Vec<Input>,
  pub outputs: Vec<UTXO>
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UTXO {
  pub txid: String,
  pub index: i32,
  pub value: u64,
  pub script_pubkey: String
}

#[derive(Debug, Clone)]
pub struct Input {
  pub txid: String,
  // UTXO index
  pub vout: u32,

  // Script that pove the propertie of UTXO being spended
  pub script_sig: String,
}

impl Transaction {
  pub fn new(txid: String, inputs: Vec<Input>, outputs: Vec<UTXO>) -> Self {
    Transaction { txid, inputs, outputs }
  }

  pub fn calculate_txid(&self) -> String {
    let serialized_data = self.serialize();  // You need to implement this serialize method
    hex::encode(utils::hash_Vec_u8(&serialized_data))
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

  pub fn serialize(&self) -> Vec<u8> {
    let mut serialized_data = Vec::new();
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
    let outputs = vec![
      UTXO {
        txid: txid.clone(),
        index: 0,
        value,
        script_pubkey
      }
    ];

    Transaction {
      txid,
      inputs: Vec::new(), // No inputs for coinbase
      outputs,
    }
  }

}

impl Input {
  pub fn serialize(&self) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    serialized_data.extend(self.txid.as_bytes());
    serialized_data.extend(&self.vout.to_le_bytes());
    serialized_data.extend(self.script_sig.as_bytes());
    serialized_data
  }
}

impl UTXO {
  pub fn serialize(&self) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    serialized_data.extend(self.txid.as_bytes());
    serialized_data.extend(&self.index.to_le_bytes());
    serialized_data.extend(&self.value.to_le_bytes());
    serialized_data.extend(self.script_pubkey.as_bytes());

    serialized_data
  }
}