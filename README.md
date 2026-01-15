A Merkle tree implementation in Rust using Keccak-256 hashing and Alloy library .

---

```text
                      Root Hash
                   /            \
                  /              \
             Hash(A+B)          Hash(C+D)
             /      \            /      \
            /        \          /        \
         Hash(A)    Hash(B)   Hash(C)  Hash(D)
           |        |          |         |
         Leaf A     Leaf B    Leaf C    Leaf D
        (txn1)     (txn2)     (txn3)    (txn4)
```

---

# functions
```text
hash_txn(txn: &str) -> B256
pair_hashes(hash_1: B256, hash_2: B256) -> B256
calc_root_hash(leaf_nodes: Vec<B256>) -> B256
get_merkle_proof(leaf_nodes: Vec<B256>, index: usize) -> Vec<B256>
verify_hash(root_hash: B256, merkle_proof: &[B256], leaf_hashes: &[B256], index: usize) -> bool
```
---

```text
  Proving Leaf B (index 1):

                    Root
                   /    \
                  /      \
             Hash(A+B)*   Hash(C+D) ← Proof[1]
             /      \
          Hash(A)  Hash(B)
            ↑         ↑
        Proof[0]   Target Leaf

Proof = [Hash(A), Hash(C+D)]
```
