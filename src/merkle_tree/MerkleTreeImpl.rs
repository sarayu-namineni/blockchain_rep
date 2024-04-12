use crate::merkle_tree::MerkleTree::MerkleTree;

struct MerkleTreeImpl;

impl MerkleTree for MerkleTreeImpl {
    type Hash = ();
    type Leaf = ();
    type Proof = ();

    fn insert(rt : Self::Hash, e: Self::Leaf) -> Self::Hash {
        return ();
    }

    fn verify(rt: Self::Hash, e: Self::Leaf, pi: Self::Proof) -> bool {
        return true;
    }
}