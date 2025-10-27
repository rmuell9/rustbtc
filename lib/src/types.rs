use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::U256;
use crate::error::{BtcError, Result};

// Type Definitions
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<Hash, TransactionOutput>,
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn new() -> Self {
        Blockchain { 
            utxos: HashMap::new(),
            blocks: vec![],
        }
    }
    
    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for transactions in &block.transactions {
                for input in &transaction.inputs {
                    self.utxos.remove(
                        &input.prev_transaction_output_hash,
                    );
                }
                for output in transaction.outputs.iter() {
                    self.utxos.insert(
                        transaction.hash(),
                        output.clone()
                    );
                }
            }
        }
    }



    pub fn add_block(&mut self, 
        block: Block) -> Result<()> {
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero()
            {
                println!("zero hash");
                return Err(BtcError::InvalidBlock);
            }
        } else {
            let last_block = self.blocks.last().unwrap();
            if block.header.prev_block_hash != last_block.hash()
            {
                println!("prev hash is wrong");
                return Err(BtcError::InvalidBlock);
            }

            if !block
                .header
                .hash()
                .matches_target(block.header.target)
            {
                println!("doesn't match target");
                return Err(BtcError::InvalidBlock);
            }

            let calculated_merkle_root =
                MerkleRoot::calculate(&block.transactions);
            if calculated_merkle_root != block.header.merkle_root
            {
                println!("invalid merkle root");
                return Err(BtcError::InvalidMerkleRoot);
            }

            if block.header.timestamp
                <= last_block.header.timestamp
            {
                return Err(BtcError::InvalidBlock);
            }

            block.verify_transactions( // will add method in future
                self.block_height(),
                &self.utxos,
            )?;
        }
        self.blocks.push(block);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
impl Block {
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>,
    ) -> Self {
        Block {
            header,
            transactions,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    pub target: U256,
}
impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}
impl TransactionOutput {
        pub fn hash(&self) -> Hash {
            Hash::hash(self)
        }
    }

impl Transaction {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>
    ) -> Self {
        Transaction {
            inputs,
            outputs,
        }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

