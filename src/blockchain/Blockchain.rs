pub struct Block<T> {
    pub data: T,
    pub next: BlockPointer<T>,
}

pub enum BlockPointer<T> {
    Empty,
    Cons(Box<Block<T>>), 
}

pub trait Blockchain {
    type BlockData;

    fn new() -> Self;
    fn add_block(&mut self, data: Self::BlockData);
    fn get_block_at_index(_: Self, index: u64) -> BlockPointer<Self::BlockData>;
}