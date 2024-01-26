pub mod pb;
pub mod utils;

use pb::acme::verifiable_block::v1::VerifiableBlock;
use substreams_ethereum::pb::eth;

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<VerifiableBlock, substreams::errors::Error> {
    Ok(VerifiableBlock::try_from(block)?)
}

