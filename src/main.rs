use clap::Parser;
use serde_json::{Map, Value};
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, action)]
    pretty: bool,
}

fn main() {
    let now = Instant::now();

    env_logger::init();
    let args = Args::parse();

    log::debug!(
        "Attempting read from {}, and will try write to {}",
        args.file.display(),
        args.output.display()
    );

    let json_data = &mut read_from_file(args.file).expect("Failed to parse the file contents!");

    remove_null_values(json_data).expect("Failed to remove null values, panicking...");

    write_to_file(args.output, json_data, args.pretty)
        .expect("Failed to write cleaned json to file, panicking...");

    println!(
        "Success! Processed file in {:.4}s",
        now.elapsed().as_secs_f32()
    );
}

fn read_from_file(file_path: PathBuf) -> Result<Value, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let v = serde_json::from_reader(reader)?;

    Ok(v)
}

fn write_to_file(
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

fn remove_null_values(json_data: &mut Value) -> Result<(), io::Error> {
    match json_data {
        Value::Object(object) => remove_null_values_from_object(object),
        Value::Array(array) => remove_null_values_from_array(array),
        _ => Ok(()),
    }
}

fn remove_null_values_from_object(object: &mut Map<String, Value>) -> Result<(), io::Error> {
    // Collect keys to remove
    let keys_to_remove: Vec<String> = object
        .iter()
        .filter_map(|(key, value)| {
            if value.is_null() {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect();

    for key in keys_to_remove {
        log::debug!("Removing key {}", key);
        object.remove(&key);
    }

    // Go a lever deeper w recursion
    for value in object.values_mut() {
        remove_null_values(value)?;
    }

    Ok(())
}

fn remove_null_values_from_array(array: &mut Vec<Value>) -> Result<(), io::Error> {
    // Collect indices to remove
    let indices_to_remove: Vec<usize> = array
        .iter()
        .enumerate()
        .filter_map(|(index, value)| if value.is_null() { Some(index) } else { None })
        .collect();

    for index in indices_to_remove.iter().rev() {
        log::debug!("Removing item with index {}", index);
        array.remove(*index);
    }

    // Go a lever deeper w recursion
    for value in array.iter_mut() {
        remove_null_values(value)?;
    }

    Ok(())
}
