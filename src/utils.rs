use log::trace;

use crate::models::SatHunterOutput;

pub fn parse_outpoint(utxo: &SatHunterOutput) -> String {
    let tx_id = &utxo.txid;
    let index = utxo.vout;
    trace!("Parsing outpoint for tx_id: {}, index: {}", tx_id, index);
    format!("{}:{}", tx_id, index)
}
