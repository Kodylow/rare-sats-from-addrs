use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Utxo {
    pub status: Status,
    pub txid: String,
    pub value: i64,
    pub vout: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub block_hash: String,
    pub block_height: i64,
    pub block_time: i64,
    pub confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SatHunterResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SatHunterOutput {
    pub address: String,
    pub txid: String,
    pub vout: i64,
    pub sat_hunter_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialSat {
    pub rarity_tags: Vec<String>,
    pub sat_number: i64,
    pub offset: i64,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub utxo: String,
    pub special_sats: Vec<SpecialSat>,
    pub extraction_psbt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingResponse {
    pub status: String,
}
