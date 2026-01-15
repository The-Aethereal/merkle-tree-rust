use alloy_primitives::{B256, keccak256};

pub fn hash_txn(txn : String) -> B256{
    return keccak256(txn);
}
pub fn main(){
    let txns : Vec<String> = ["txn1", "txn2", "txn3"].iter()
        .map(|str| str.to_string())
        .collect();
    
}