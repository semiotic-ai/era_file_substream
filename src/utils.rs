use prost_types::Timestamp;
use substreams_ethereum::pb::eth;
use substreams_ethereum::pb::eth::v2::{CallType, TransactionTrace};
use crate::pb::acme::verifiable_block::v1::{AccessTuple, BigInt, BlockHeader, Log, Transaction, TransactionReceipt, Uint64Array, Uint64NestedArray, VerifiableBlock};

impl TryFrom<eth::v2::Block> for VerifiableBlock {
    type Error = substreams::errors::Error;

    fn try_from(block: eth::v2::Block) -> Result<Self, Self::Error>{
        let transactions = block.transaction_traces.into_iter().map(Transaction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            number: block.number,
            size: block.size,
            header: block.header.map(BlockHeader::from),
            uncles: block.uncles.into_iter().map(BlockHeader::from).collect(),
            transactions,
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

impl TryFrom<TransactionTrace> for Transaction {
    type Error = substreams::errors::Error;

    fn try_from(value: TransactionTrace) -> Result<Self, Self::Error> {
        let transaction = Transaction {
            to: map_tx_to(&value)?, // TODO: PROBLEM IS HERE
            nonce: value.nonce,
            gas_price: value.gas_price.map(BigInt::from),
            gas_limit: value.gas_limit,
            value: value.value.map(BigInt::from),
            input: value.input,
            v: value.v,
            r: value.r,
            s: value.s,
            r#type: value.r#type,
            access_list: value.access_list.into_iter().map(|access| AccessTuple {
                address: access.address,
                storage_keys: access.storage_keys,
            }).collect(),
            receipt: value.receipt.map(TransactionReceipt::from),
            status: value.status,
            max_fee_per_gas: value.max_fee_per_gas.map(BigInt::from),
            max_priority_fee_per_gas: value.max_priority_fee_per_gas.map(BigInt::from),
            hash: value.hash,
        };

        Ok(transaction)
    }
}


fn map_tx_to(trace: &TransactionTrace) -> Result<Vec<u8>, substreams::errors::Error> {
    let first_call = trace.calls.first().ok_or(substreams::errors::Error::msg("No calls in tx"))?;

    let call_type = first_call.call_type();

    if call_type == CallType::Create {
        Ok(vec![])
    } else {
        Ok(trace.to.clone())
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