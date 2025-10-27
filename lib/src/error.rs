use thiserror::Error;
#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Invalid block")]
    InvalidBlock,
    #[error("Invalid block header")]
    InvalidBlockHeader,
    #[error("Invalid transactio input")]
    InvalidTransactionInput,
    #[error("Invalid transaction output")]
    InvalidTransactionOutput,
    #[error("Invalid Merkle Root")]
    InvalidMerkleRoot,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invaid public key")]
    InvalidPublicKey,
    #[error("Invalid transaction")]
    InvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, BtcError>;
