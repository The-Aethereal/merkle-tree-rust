use alloy_primitives::{B256, keccak256};

pub fn hash_txn(txn: &str) -> B256 {
    return keccak256(txn.to_le_bytes());
}



pub fn pair_hashes(hash_1: B256, hash_2: B256) -> B256 {
   /* let hash_combination: String = hash_1.to_string() + &hash_2.to_string(); // for concatenation move first and borrow second  */
    let mut buffer = [0u8; 64];
    buffer[..32].copy_from_slice(hash_1.as_slice());      // Merkle trees concatenate bytes, not hex strings.
    buffer[32..].copy_from_slice(hash_2.as_slice());                                                
    return keccak256(buffer);
}

pub fn next_level(prev_level: &Vec<B256>) -> Vec<B256>{
    let mut next_lev = Vec::with_capacity((prev_level.len() + 1) / 2);

    let mut i:usize = 0;
    while i < prev_level.len() {
        let left = prev_level[i];
        let right = 
        if i + 1 < prev_level.len() {
            prev_level[i + 1]
        } else {prev_level[i] };  // duplicating last element if odd prev_level.len()
            
        

        next_lev.push(pair_hashes(left, right));
        i += 2;
    }

    return next_lev;
}

pub fn calc_root_hash(leaf_nodes: Vec<B256>) -> B256 {
    let mut hashes_current_level =leaf_nodes;
    let mut n = hashes_current_level.len();

    while n > 1 {
        let mut write = 0;let mut read = 0;
        

        while read < n {
            // other form of hashes_current_level[i/2]= pair_hashes(hashes_current_level[i],hashes_current_level[min(i+1,N-1)] )

            let left = hashes_current_level[read];
            let right = if read + 1 < n {
                hashes_current_level[read + 1]
            } else {
                hashes_current_level[read]
            };
            hashes_current_level[write] = pair_hashes(left, right); 

            write += 1; read += 2;
        }

        n = write;
    }

    return hashes_current_level[0];
}

pub fn get_merkle_proof(leaf_nodes:Vec<B256>, index:usize)-> Vec<B256>{
    
    let mut hashes_current_level =leaf_nodes;

    let mut merkle_proof: Vec<B256>= Vec::new();

    let mut n = hashes_current_level.len();

    while n > 1 {
        let mut write = 0; let mut read = 0;


        while read < n {
            if index==read {merkle_proof.push(
                if read + 1 < n {
                hashes_current_level[read + 1]
                } else {
                    hashes_current_level[read]
                }
            )}
            else if index==read+1 {merkle_proof.push(hashes_current_level[read]);}
            let left = hashes_current_level[read];
            let right = if read + 1 < n {
                hashes_current_level[read + 1]
            } else {
                hashes_current_level[read]
            };
            
            hashes_current_level[write] = pair_hashes(left, right); 

            write += 1; read += 2;

        }

        n = write;
    }

    return merkle_proof;
}

fn verify_hash(root_hash: B256, merkle_proof: &[B256], leaf_hashes: &[B256], mut index: usize) -> bool {

    
    false
}
pub fn main() {
    let txns: Vec<String> = ["txn1", "txn2", "txn3"]
        .iter()
        .map(|str| str.to_string())
        .collect();
    let leaf_hashes: Vec<B256> = txns.iter().map(|txn| hash_txn(&txn)).collect();

}
