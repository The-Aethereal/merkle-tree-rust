use alloy_primitives::{B256, keccak256};

pub fn hash_txn(txn: &String) -> B256 {
    return keccak256(txn);
}

pub fn pair_hashes(hash_1: B256, hash_2: B256) -> B256 {
    let hash_combination: String = hash_1.to_string() + &hash_2.to_string(); // for concatenation move first and borrow second
    return keccak256(hash_combination);
}

pub fn calc_root_hash(leaf_hash: Vec<B256>) {}

pub fn main() {
    let txns: Vec<String> = ["txn1", "txn2", "txn3"]
        .iter()
        .map(|str| str.to_string())
        .collect();
    let leaf_hash: Vec<B256> = txns.iter().map(|txn| txn.hash_txn(&txn)).collect();
}
