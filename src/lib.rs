mod pb;
mod utils;
mod header_accumulator;

use substreams::pb::substreams::store_delta::Operation;
use substreams::store::{StoreSet, StoreSetProto, StoreNew, StoreDelete, Deltas, DeltaProto};
use pb::acme::verifiable_block::v1::VerifiableBlock;
use substreams_ethereum::pb::eth;
use crate::header_accumulator::{get_value_for_block};
use crate::pb::acme::verifiable_block::v1::Era;

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<VerifiableBlock, substreams::errors::Error> {
    Ok(VerifiableBlock::try_from(block)?)
}

#[substreams::handlers::store]
pub fn store_era(blk: VerifiableBlock, output: StoreSetProto<VerifiableBlock>) {
    let era_id = blk.number / 8192;
    let relative_block_number = blk.number % 8192;

    if blk.number % 8192 == 0 && era_id > 0 {
        // Delete the previous era
        output.delete_prefix(0, &"Block:".to_string());
    }

    output.set(
        era_id,
        format!("Block:{}", relative_block_number),
        &blk,
    );
}

#[substreams::handlers::map]
pub fn output_era(deltas: Deltas<DeltaProto<VerifiableBlock>>) -> Result<Option<Era>, substreams::errors::Error> {
    if deltas.deltas.len() == 1 {
        return Ok(None);
    }

    let blocks: Vec<VerifiableBlock> = deltas.iter()
        .filter(|delta| delta.operation == Operation::Delete)
        .map(|delta| delta.old_value.clone())
        .collect();

    if blocks.len() != 8192 {
        return Err(substreams::errors::Error::msg(format!("Invalid number of blocks in era: {}", blocks.len())));
    }

    let header_accumulator_value = get_value_for_block(blocks[0].number);

    Ok(Some(Era {
        blocks,
        header_accumulator_value,
    }))
}
