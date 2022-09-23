use crate::block::Block;

pub struct Blockchain {
  pub blocks: Vec<Block>
}

impl Blockchain {
  pub fn new() -> Self {
    Blockchain { 
      blocks: vec![],
    }
    
  }
  pub fn insert_new_block(&mut self, block: Block) {
    self.blocks.push(block);
  }
}