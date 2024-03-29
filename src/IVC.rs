struct PublicParams(u64);
struct Func(fn u64 -> u64);
struct ProveKey(u64);
struct VerifyKey(u64);
struct MerkleTree {
	root: u64
}
struct BlockHash(u64);
struct Block {
	parent_block_hash: BlockHash,
	txn_merkle_root: MerkleTree.root,
	epoch: u64
}
struct BlockAddr(u64);
struct RepScore(u64);
struct Output(BlockHash, BlockHash, BlockAddr, RepScore);
struct Txn {
	sender: BlockAddr,
	receiver: BlockAddr,
	amount: u64
}
struct Proof(u64);
struct TxnWitness {
	txn: Txn,
	path: Proof
}
struct Witness(Vec<Block>, Vec<TxnWitness>);

trait IVC {
	fn generate(u64) -> PublicParams;
	fn encode(PublicParams, Func) -> (ProveKey, VerifyKey);
	fn prove(ProveKey, (u64, Output, Output), Witness, Proof) -> Proof;
	fn verify(VerifyKey, (u64, Output, Output), Proof) -> bool;
}