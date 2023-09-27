use anyhow::Context;
use log::trace;

use crate::models::{PendingResponse, SatHunterResponse, ScanResult, Utxo};

pub fn get_utxos(address: &str) -> anyhow::Result<Vec<Utxo>> {
    let url = format!("https://mempool.space/api/address/{}/utxo", address);
    trace!("Sending GET request to {}", url);
    let response = reqwest::blocking::get(&url).context("Failed to send GET request")?;
    trace!("Received response: {:?}", response);
    let utxos: Vec<Utxo> = response
        .json()
        .context("Failed to parse response into UTXOs")?;
    trace!("Parsed UTXOs: {:?}", utxos);
    Ok(utxos)
}

pub fn post_to_sat_hunter(outpoint: String, api_key: &String) -> anyhow::Result<String> {
    let url = "https://api.deezy.io/v1/sat-hunting/scan";
    let body = serde_json::json!({
        "utxo_to_scan": outpoint,
    });
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("x-api-token", api_key)
        .json(&body)
        .send()
        .context("Failed to send POST request")?;
    let sat_hunter_response: SatHunterResponse = response
        .json()
        .context("Failed to parse response into SatHunterResponse")?;
    Ok(sat_hunter_response.id)
}

pub fn get_scan_result(id: &str, api_key: &String) -> anyhow::Result<Option<ScanResult>> {
    let url = format!("https://api.deezy.io/v1/sat-hunting/scan/{}", id);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("x-api-token", api_key)
        .send()
        .context("Failed to send request")?;
    let body = response.text().context("Failed to read response body")?; // Convert the response body to a String
    println!("Response body: {}", body);

    // Try to parse the response body into a PendingResponse
    let pending_result: Result<PendingResponse, _> = serde_json::from_str(&body);
    if let Ok(pending) = pending_result {
        if pending.status == "PENDING" {
            // If the status is "PENDING", return None
            return Ok(None);
        }
    }

    // If the status was not "PENDING", try to parse the response body into a ScanResult
    let scan_result: Result<ScanResult, _> = serde_json::from_str(&body);
    match scan_result {
        Ok(res) => Ok(Some(res)),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Err(anyhow::Error::new(e))
        }
    }
}
