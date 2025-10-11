use crate::block::Block;

pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Chain {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev = self.blocks.last().unwrap();
        let block = Block::new(prev.height + 1, data, prev.hash.clone());
        self.blocks.push(block);
    }
}
