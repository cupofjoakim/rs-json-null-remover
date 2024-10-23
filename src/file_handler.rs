use serde_json::Value;
use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

pub fn read_from_file(file_path: PathBuf) -> Result<Value, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let v = serde_json::from_reader(reader)?;

    Ok(v)
}

pub fn write_to_file(
    file_path: PathBuf,
    json_data: &mut Value,
    should_pretty_print: bool,
) -> Result<(), io::Error> {
    let file = File::create(file_path)?;

    log::debug!("Should pretty print: {}", should_pretty_print);

    if should_pretty_print {
        serde_json::to_writer_pretty(file, json_data)?;
    } else {
        serde_json::to_writer(file, json_data)?;
    }

    Ok(())
}
