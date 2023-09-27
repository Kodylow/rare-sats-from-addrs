use dotenv::dotenv;
use log::{error, info};
use serde_json;
use std::path::Path;
use std::{env, fs};
use std::{fs::File, io::BufWriter};

mod file_io;
mod http;
mod models;
mod utils;

use crate::file_io::{read_addresses_from_file, read_ids_from_file};
use crate::http::{get_scan_result, get_utxos, post_to_sat_hunter};
use crate::models::SatHunterOutput;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenv().ok();
    let api_key = env::var("DEEZY_API_KEY").expect("DEEZY_API_KEY must be set");
    info!("Starting application");

    let ids_file_exists = Path::new("sat-hunter-ids.txt").exists();

    if !ids_file_exists {
        process_addresses()?;
        process_sat_hunter(&api_key)?;
    }

    check_sat_hunter(&api_key)?;

    Ok(())
}

fn process_addresses() -> anyhow::Result<()> {
    let addresses: Vec<String> =
        read_addresses_from_file("addresses.txt").expect("Unable to read addresses");

    let mut output = Vec::new();

    for addr in addresses {
        info!("Processing address: {}", addr);
        match get_utxos(&addr) {
            Ok(utxos) => {
                for utxo in utxos {
                    output.push(SatHunterOutput {
                        address: addr.clone(),
                        txid: utxo.txid,
                        vout: utxo.vout,
                        sat_hunter_id: String::new(), // Initialize with empty string
                    });
                }
            }
            Err(e) => {
                error!("Error getting UTXOs for address {}: {}", addr, e);
            }
        }
    }

    info!("Finished processing all addresses");
    let file = File::create("sat-hunting.json").expect("Unable to create file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &output).expect("Unable to write data");
    info!("Data written to sat-hunting.json");

    Ok(())
}

use crate::file_io::{write_ids_to_file, write_to_json_file};

fn process_sat_hunter(api_key: &String) -> anyhow::Result<()> {
    let mut output: Vec<SatHunterOutput> =
        serde_json::from_str(&fs::read_to_string("sat-hunting.json").unwrap()).unwrap();

    for item in &mut output {
        let outpoint = utils::parse_outpoint(&item);
        match post_to_sat_hunter(outpoint, &api_key) {
            Ok(id) => {
                item.sat_hunter_id = id;
            }
            Err(e) => {
                error!("Error posting UTXO to sat hunter: {}", e);
            }
        }
    }

    write_to_json_file("sat-hunting.json", &output)?;
    info!("Data written to sat-hunting.json");

    write_ids_to_file("sat-hunter-ids.txt", &output)?;
    info!("Data written to sat-hunter-ids.txt");

    Ok(())
}

fn check_sat_hunter(api_key: &String) -> anyhow::Result<()> {
    let ids: Vec<String> = read_ids_from_file("sat-hunter-ids.txt").expect("Unable to read IDs");

    let mut results = Vec::new();

    for id in ids {
        match get_scan_result(&id, &api_key) {
            Ok(result) => {
                results.push(result);
                info!("Successfully fetched result for ID {}", id);
            }
            Err(e) => {
                error!("Error fetching result for ID {}: {}", id, e);
            }
        }
    }

    let file = File::create("scan-results.json").expect("Unable to create file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &results).expect("Unable to write data");
    info!("Data written to scan-results.json");

    Ok(())
}
