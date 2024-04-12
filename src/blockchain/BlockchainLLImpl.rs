use crate::blockchain::Blockchain::Block;
use crate::blockchain::Blockchain::BlockPointer;
use crate::blockchain::Blockchain::Blockchain;

use std::mem;

type Hash = ();
type Epoch = ();

pub struct BlockData { 
	hash: Hash,
    epoch: Epoch,
}

pub struct BlockchainLLImpl {
    head: BlockPointer<BlockData>,
}

impl Blockchain for BlockchainLLImpl {
    type BlockData = BlockData;

    fn new() -> Self {
        return BlockchainLLImpl {
            head: BlockPointer::Empty,
        };
    }

    fn add_block(&mut self, data: Self::BlockData) {
        let new_block = Box::new(Block {
            data: data,
            next: mem::replace(&mut self.head, BlockPointer::Empty),
        });
        self.head = BlockPointer::Cons(new_block);
    }

    fn get_block_at_index(_: Self, index: u64) -> BlockPointer<Self::BlockData> {
        return BlockPointer::Empty;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut blockchain = BlockchainLLImpl::new();
    }

    #[test]
    fn test_add_and_get_block() {
        let mut blockchain = BlockchainLLImpl::new();
        blockchain.add_block(BlockData {
            hash: (),
            epoch: ()
        });
        let added_block = BlockchainLLImpl::get_block_at_index(blockchain, 0);
    }
}