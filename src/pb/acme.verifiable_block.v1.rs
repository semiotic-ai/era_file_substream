// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Era {
    #[prost(bytes="vec", tag="1")]
    pub header_accumulator_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub blocks: ::prost::alloc::vec::Vec<VerifiableBlock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifiableBlock {
    /// Hash is the block's hash.
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Number is the block's height at which this block was mined.
    #[prost(uint64, tag="3")]
    pub number: u64,
    /// Size is the size in bytes of the RLP encoding of the block according to Ethereum
    /// rules.
    #[prost(uint64, tag="4")]
    pub size: u64,
    /// Header contain's the block's header information like its parent hash, the merkel root hash
    /// and all other information the form a block.
    #[prost(message, optional, tag="5")]
    pub header: ::core::option::Option<BlockHeader>,
    /// Uncles represents block produced with a valid solution but were not actually choosen
    /// as the canonical block for the given height so they are mostly "forked" blocks.
    ///
    /// If the Block has been produced using the Proof of Stake consensus algorithm, this
    /// field will actually be always empty.
    #[prost(message, repeated, tag="6")]
    pub uncles: ::prost::alloc::vec::Vec<BlockHeader>,
    #[prost(message, repeated, tag="10")]
    pub transaction_receipts: ::prost::alloc::vec::Vec<TransactionReceipt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(bytes="vec", tag="1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    /// Uncle hash of the block, some reference it as `sha3Uncles`, but `sha3`` is badly worded, so we prefer `uncle_hash`, also
    /// referred as `ommers` in EIP specification.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to `0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347`.
    #[prost(bytes="vec", tag="2")]
    pub uncle_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub coinbase: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub transactions_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub receipt_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    /// Difficulty is the difficulty of the Proof of Work algorithm that was required to compute a solution.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to `0x00`.
    #[prost(message, optional, tag="8")]
    pub difficulty: ::core::option::Option<BigInt>,
    /// TotalDifficulty is the sum of all previous blocks difficulty including this block difficulty.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to the terminal total difficulty
    /// that was required to transition to Proof of Stake algorithm, which varies per network. It is set to
    /// 58 750 000 000 000 000 000 000 on Ethereum Mainnet and to 10 790 000 on Ethereum Testnet Goerli.
    #[prost(message, optional, tag="17")]
    pub total_difficulty: ::core::option::Option<BigInt>,
    #[prost(uint64, tag="9")]
    pub number: u64,
    #[prost(uint64, tag="10")]
    pub gas_limit: u64,
    #[prost(uint64, tag="11")]
    pub gas_used: u64,
    #[prost(message, optional, tag="12")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// ExtraData is free-form bytes included in the block by the "miner". While on Yellow paper of
    /// Ethereum this value is maxed to 32 bytes, other consensus algorithm like Clique and some other
    /// forks are using bigger values to carry special consensus data.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field is strictly enforced to be <= 32 bytes.
    #[prost(bytes="vec", tag="13")]
    pub extra_data: ::prost::alloc::vec::Vec<u8>,
    /// MixHash is used to prove, when combined with the `nonce` that sufficient amount of computation has been
    /// achieved and that the solution found is valid.
    #[prost(bytes="vec", tag="14")]
    pub mix_hash: ::prost::alloc::vec::Vec<u8>,
    /// Nonce is used to prove, when combined with the `mix_hash` that sufficient amount of computation has been
    /// achieved and that the solution found is valid.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to `0`.
    #[prost(uint64, tag="15")]
    pub nonce: u64,
    /// Hash is the hash of the block which is actually the computation:
    ///
    ///   Keccak256(rlp([
    ///     parent_hash,
    ///     uncle_hash,
    ///     coinbase,
    ///     state_root,
    ///     transactions_root,
    ///     receipt_root,
    ///     logs_bloom,
    ///     difficulty,
    ///     number,
    ///     gas_limit,
    ///     gas_used,
    ///     timestamp,
    ///     extra_data,
    ///     mix_hash,
    ///     nonce,
    ///     base_fee_per_gas (to be included, only if London Fork is active)
    ///     withdrawals_root (to be included, only if Shangai Fork is active)
    ///   ]))
    ///
    #[prost(bytes="vec", tag="16")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Base fee per gas according to EIP-1559 (e.g. London Fork) rules, only set if London is present/active on the chain.
    #[prost(message, optional, tag="18")]
    pub base_fee_per_gas: ::core::option::Option<BigInt>,
    /// Withdrawals root hash according to EIP-4895 (e.g. Shangai Fork) rules, only set if Shangai is present/active on the chain.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(bytes="vec", tag="19")]
    pub withdrawals_root: ::prost::alloc::vec::Vec<u8>,
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, optional, tag="20")]
    pub tx_dependency: ::core::option::Option<Uint64NestedArray>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint64NestedArray {
    #[prost(message, repeated, tag="1")]
    pub val: ::prost::alloc::vec::Vec<Uint64Array>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint64Array {
    #[prost(uint64, repeated, tag="1")]
    pub val: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(bytes="vec", tag="1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionReceipt {
    /// State root is an intermediate state_root hash, computed in-between transactions to make
    /// **sure** you could build a proof and point to state in the middle of a block. Geth client
    /// uses `PostState + root + PostStateOrStatus`` while Parity used `status_code, root...`` this piles
    /// hardforks, see (read the EIPs first):
    /// - <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-658.md>
    ///
    /// Moreover, the notion of `Outcome`` in parity, which segregates the two concepts, which are
    /// stored in the same field `status_code`` can be computed based on such a hack of the `state_root`
    /// field, following `EIP-658`.
    ///
    /// Before Byzantinium hard fork, this field is always empty.
    #[prost(bytes="vec", tag="1")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub cumulative_gas_used: u64,
    #[prost(bytes="vec", tag="3")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Index is the index of the log relative to the transaction. This index
    /// is always populated regardless of the state revertion of the the call
    /// that emitted this log.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(uint32, tag="4")]
    pub index: u32,
    /// BlockIndex represents the index of the log relative to the Block.
    ///
    /// An **important** notice is that this field will be 0 when the call
    /// that emitted the log has been reverted by the chain.
    ///
    /// Currently, there is two locations where a Log can be obtained:
    /// - block.transaction_traces\[].receipt.logs[\]
    /// - block.transaction_traces\[].calls[].logs[\]
    ///
    /// In the `receipt` case, the logs will be populated only when the call
    /// that emitted them has not been reverted by the chain and when in this
    /// position, the `blockIndex` is always populated correctly.
    ///
    /// In the case of `calls` case, for `call` where `stateReverted == true`,
    /// the `blockIndex` value will always be 0.
    #[prost(uint32, tag="6")]
    pub block_index: u32,
    #[prost(uint64, tag="7")]
    pub ordinal: u64,
}
// @@protoc_insertion_point(module)
