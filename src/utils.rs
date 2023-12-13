use prost_types::Timestamp;
use substreams_ethereum::pb::eth;
use substreams_ethereum::pb::eth::v2::TransactionTrace;
use crate::pb::acme::verifiable_block::v1::{BigInt, BlockHeader, Log, TransactionReceipt, Uint64Array, Uint64NestedArray, VerifiableBlock};

impl TryFrom<eth::v2::Block> for VerifiableBlock {
    type Error = substreams::errors::Error;

    fn try_from(block: eth::v2::Block) -> Result<Self, Self::Error>{
        let transaction_receipts = block.transaction_traces.into_iter().map(TransactionReceipt::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            number: block.number,
            size: block.size,
            header: block.header.map(BlockHeader::from),
            uncles: block.uncles.into_iter().map(BlockHeader::from).collect(),
            transaction_receipts,
            balance_changes: vec![],
            code_changes: vec![],
            hash: block.hash,
        })
    }
}

impl From<eth::v2::BlockHeader> for BlockHeader {
    fn from(header: eth::v2::BlockHeader) -> Self {
        Self {
            parent_hash: header.parent_hash,
            uncle_hash: header.uncle_hash,
            coinbase: header.coinbase,
            state_root: header.state_root,
            transactions_root: header.transactions_root,
            receipt_root: header.receipt_root,
            logs_bloom: header.logs_bloom,
            difficulty: header.difficulty.map(BigInt::from),
            timestamp: header.timestamp.map(|t| Timestamp {
                seconds: t.seconds,
                nanos: t.nanos,
            }),
            extra_data: header.extra_data,
            mix_hash: header.mix_hash,
            nonce: header.nonce,
            hash: header.hash,
            base_fee_per_gas: header.base_fee_per_gas.map(BigInt::from),
            withdrawals_root: header.withdrawals_root,
            number: header.number,
            gas_limit: header.gas_limit,
            total_difficulty: header.total_difficulty.map(BigInt::from),
            gas_used: header.gas_used,
            tx_dependency: header.tx_dependency.map(Uint64NestedArray::from),
        }
    }
}

impl From<eth::v2::BigInt> for BigInt {
    fn from(big_int: eth::v2::BigInt) -> Self {
        Self {
            bytes: big_int.bytes,
        }
    }
}


impl From<eth::v2::Uint64NestedArray> for Uint64NestedArray {
    fn from(nested_array: eth::v2::Uint64NestedArray) -> Self {
        Self {
            val: nested_array.val.into_iter().map(Uint64Array::from).collect(),
        }
    }
}

impl From<eth::v2::Uint64Array> for Uint64Array {
    fn from(array: eth::v2::Uint64Array) -> Self {
        Self {
            val: array.val,
        }
    }
}

impl TryFrom<TransactionTrace> for TransactionReceipt {
    type Error = substreams::errors::Error;

    fn try_from(value: TransactionTrace) -> Result<Self, Self::Error> {
        let receipt = value.receipt
            .ok_or_else(|| substreams::errors::Error::msg("Missing tx receipt"))?;
        Ok(Self::from(receipt))
    }
}

impl From<eth::v2::TransactionReceipt> for TransactionReceipt {
    fn from(receipt: eth::v2::TransactionReceipt) -> Self {
        Self {
            state_root: receipt.state_root,
            cumulative_gas_used: receipt.cumulative_gas_used,
            logs_bloom: receipt.logs_bloom,
            logs: receipt.logs.into_iter().map(Log::from).collect(),
        }
    }
}

impl From<eth::v2::Log> for Log {
    fn from(log: eth::v2::Log) -> Self {
        Self {
            address: log.address,
            topics: log.topics,
            data: log.data,
            index: log.index,
            block_index: log.block_index,
            ordinal: log.ordinal,
        }
    }
}