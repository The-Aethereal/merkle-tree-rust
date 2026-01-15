use alloy_primitives::{B256, keccak256};

pub fn hash_txn(txn: &String) -> B256 {
    return keccak256(txn);
}



pub fn pair_hashes(hash_1: B256, hash_2: B256) -> B256 {
   /* let hash_combination: String = hash_1.to_string() + &hash_2.to_string(); // for concatenation move first and borrow second  */
    let mut buffer = [0u8; 64];
    buffer[..32].copy_from_slice(hash_1.as_slice());      // Merkle trees concatenate bytes, not hex strings.
    buffer[32..].copy_from_slice(hash_2.as_slice());                                                
    return keccak256(buffer);
}

pub fn next_level(prev_level: &Vec<B256>) -> Vec<B256>{
    let mut next_lev= Vec::new();
    let mut j=0;
    for i in 0..prev_level.len(){
        next_lev[j]=pair_hashes(prev_level[i], prev_level[i+1]);
        j=j+1;
    }
    return next_lev;
}

pub fn calc_root_hash(leaf_hashes: Vec<B256>) -> B256 {
    let mut root_vec =leaf_hashes;
    while(root_vec.len()>1){
        root_vec = next_level(&root_vec);
    }
    return root_vec[0];
}

pub fn main() {
    let txns: Vec<String> = ["txn1", "txn2", "txn3"]
        .iter()
        .map(|str| str.to_string())
        .collect();
    let leaf_hashes: Vec<B256> = txns.iter().map(|txn| hash_txn(&txn)).collect();

}
