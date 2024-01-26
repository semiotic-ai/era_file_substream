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
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
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
///
/// inner TxData    // Consensus contents of a transaction
/// time  time.Time // Time first seen locally (spam avoidance)
///
/// // caches
/// hash atomic.Value
/// size atomic.Value
/// from atomic.Value
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// consensus
    #[prost(bytes="vec", tag="1")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub nonce: u64,
    /// GasPrice represents the effective price that has been paid for each gas unit of this transaction. Over time, the
    /// Ethereum rules changes regarding GasPrice field here. Before London fork, the GasPrice was always set to the
    /// fixed gas price. After London fork, this value has different meaning depending on the transaction type (see `Type` field).
    ///
    /// In cases where `TransactionTrace.Type == TRX_TYPE_LEGACY || TRX_TYPE_ACCESS_LIST`, then GasPrice has the same meaning
    /// as before the London fork.
    ///
    /// In cases where `TransactionTrace.Type == TRX_TYPE_DYNAMIC_FEE`, then GasPrice is the effective gas price paid
    /// for the transaction which is equals to `BlockHeader.BaseFeePerGas + TransactionTrace.`
    #[prost(message, optional, tag="3")]
    pub gas_price: ::core::option::Option<BigInt>,
    /// GasLimit is the maximum of gas unit the sender of the transaction is willing to consume when perform the EVM
    /// execution of the whole transaction
    #[prost(uint64, tag="4")]
    pub gas_limit: u64,
    /// Value is the amount of Ether transferred as part of this transaction.
    #[prost(message, optional, tag="5")]
    pub value: ::core::option::Option<BigInt>,
    /// Input data the transaction will receive for execution of EVM.
    #[prost(bytes="vec", tag="6")]
    pub input: ::prost::alloc::vec::Vec<u8>,
    /// V is the recovery ID value for the signature Y point.
    #[prost(bytes="vec", tag="7")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// R is the signature's X point on the elliptic curve (32 bytes).
    #[prost(bytes="vec", tag="8")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// S is the signature's Y point on the elliptic curve (32 bytes).
    #[prost(bytes="vec", tag="9")]
    pub s: ::prost::alloc::vec::Vec<u8>,
    // GasUsed is the total amount of gas unit used for the whole execution of the transaction.
    //   uint64 gas_used = 10;

    /// Type represents the Ethereum transaction type, available only since EIP-2718 & EIP-2930 activation which happened on Berlin fork.
    /// The value is always set even for transaction before Berlin fork because those before the fork are still legacy transactions.
    #[prost(enumeration="transaction::Type", tag="12")]
    pub r#type: i32,
    /// AcccessList represents the storage access this transaction has agreed to do in which case those storage
    /// access cost less gas unit per access.
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_ACCESS_LIST || TRX_TYPE_DYNAMIC_FEE` which
    /// is possible only if Berlin (TRX_TYPE_ACCESS_LIST) nor London (TRX_TYPE_DYNAMIC_FEE) fork are active on the chain.
    #[prost(message, repeated, tag="14")]
    pub access_list: ::prost::alloc::vec::Vec<AccessTuple>,
    //   // MaxFeePerGas is the maximum fee per gas the user is willing to pay for the transaction gas used.
    //   //
    //   // This will is populated only if `TransactionTrace.Type == TRX_TYPE_DYNAMIC_FEE` which is possible only
    //   // if Londong fork is active on the chain.
    //   //
    //   // Only available in DetailLevel: EXTENDED
    //   BigInt max_fee_per_gas = 11;

    //   // MaxPriorityFeePerGas is priority fee per gas the user to pay in extra to the miner on top of the block's
    //   // base fee.
    //   //
    //   // This will is populated only if `TransactionTrace.Type == TRX_TYPE_DYNAMIC_FEE` which is possible only
    //   // if London fork is active on the chain.
    //   //
    //   // Only available in DetailLevel: EXTENDED
    //   BigInt max_priority_fee_per_gas = 13;

    // meta
    //   uint32 index = 20;
    //   bytes hash = 21;
    //   bytes from = 22;
    // Only available in DetailLevel: EXTENDED
    //   bytes return_data = 23;
    // Only available in DetailLevel: EXTENDED
    //   bytes public_key = 24;
    //   uint64 begin_ordinal = 25;
    //   uint64 end_ordinal = 26;

    // TransactionTraceStatus is the status of the transaction execution and will let you know if the transaction
    // was successful or not.
    //
    // A successful transaction has been recorded to the blockchain's state for calls in it that were successful.
    // This means it's possible only a subset of the calls were properly recorded, refer to \[calls[].state_reverted\] field
    // to determine which calls were reverted.
    //
    // A quirks of the Ethereum protocol is that a transaction `FAILED` or `REVERTED` still affects the blockchain's
    // state for **some** of the state changes. Indeed, in those cases, the transactions fees are still paid to the miner
    // which means there is a balance change for the transaction's emitter (e.g. `from`) to pay the gas fees, an optional
    // balance change for gas refunded to the transaction's emitter (e.g. `from`) and a balance change for the miner who
    // received the transaction fees. There is also a nonce change for the transaction's emitter (e.g. `from`).
    //
    // This means that to properly record the state changes for a transaction, you need to conditionally procees the
    // transaction's status.
    //
    // For a `SUCCEEDED` transaction, you iterate over the `calls` array and record the state changes for each call for
    // which `state_reverted == false` (if a transaction succeeded, the call at #0 will always `state_reverted == false`
    // because it aligns with the transaction).
    //
    // For a `FAILED` or `REVERTED` transaction, you iterate over the root call (e.g. at #0, will always exist) for
    // balance changes you process those where `reason` is either `REASON_GAS_BUY`, `REASON_GAS_REFUND` or
    // `REASON_REWARD_TRANSACTION_FEE` and for nonce change, still on the root call, you pick the nonce change which the
    // smallest ordinal (if more than one).
    //   TransactionTraceStatus status = 30;

    #[prost(message, optional, tag="31")]
    pub receipt: ::core::option::Option<TransactionReceipt>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// All transactions that ever existed prior Berlin fork before EIP-2718 was implemented.
        TrxTypeLegacy = 0,
        /// Transaction that specicy an access list of contract/storage_keys that is going to be used
        /// in this transaction.
        ///
        /// Added in Berlin fork (EIP-2930).
        TrxTypeAccessList = 1,
        /// Transaction that specifis an access list just like TRX_TYPE_ACCESS_LIST but in addition defines the
        /// max base gas gee and max priority gas fee to pay for this transaction. Transaction's of those type are
        /// executed against EIP-1559 rules which dictates a dynamic gas cost based on the congestion of the network.
        TrxTypeDynamicFee = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::TrxTypeLegacy => "TRX_TYPE_LEGACY",
                Type::TrxTypeAccessList => "TRX_TYPE_ACCESS_LIST",
                Type::TrxTypeDynamicFee => "TRX_TYPE_DYNAMIC_FEE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRX_TYPE_LEGACY" => Some(Self::TrxTypeLegacy),
                "TRX_TYPE_ACCESS_LIST" => Some(Self::TrxTypeAccessList),
                "TRX_TYPE_DYNAMIC_FEE" => Some(Self::TrxTypeDynamicFee),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionTraceStatus {
    Unknown = 0,
    Succeeded = 1,
    Failed = 2,
    Reverted = 3,
}
impl TransactionTraceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionTraceStatus::Unknown => "UNKNOWN",
            TransactionTraceStatus::Succeeded => "SUCCEEDED",
            TransactionTraceStatus::Failed => "FAILED",
            TransactionTraceStatus::Reverted => "REVERTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SUCCEEDED" => Some(Self::Succeeded),
            "FAILED" => Some(Self::Failed),
            "REVERTED" => Some(Self::Reverted),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
