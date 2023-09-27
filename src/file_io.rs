use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use anyhow::Context;

use crate::models::SatHunterOutput;

pub fn read_addresses_from_file<P>(filename: P) -> anyhow::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).context("Failed to open file")?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .collect::<Result<_, _>>()
        .context("Failed to read lines from file")
}

pub fn read_ids_from_file<P>(filename: P) -> anyhow::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).context("Failed to open file")?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .collect::<Result<_, _>>()
        .context("Failed to read lines from file")
}

pub fn write_to_json_file<T: serde::Serialize>(filename: &str, data: &T) -> anyhow::Result<()> {
    let file = File::create(filename).context("Unable to create file")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).context("Unable to write data")
}

pub fn write_ids_to_file(filename: &str, data: &[SatHunterOutput]) -> anyhow::Result<()> {
    let ids_file = File::create(filename).context("Unable to create file")?;
    let mut ids_writer = BufWriter::new(ids_file);
    for item in data {
        writeln!(ids_writer, "{}", item.sat_hunter_id).context("Unable to write data")?;
    }
    Ok(())
}
