pub trait MerkleTree {
    type Hash;
    type Leaf;
    type Proof;

    fn insert(rt : Self::Hash, e: Self::Leaf) -> Self::Hash;
    fn verify(rt: Self::Hash, e: Self::Leaf, pi: Self::Proof) -> bool;
}