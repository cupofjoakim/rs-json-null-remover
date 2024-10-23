use clap::Parser;
use serde_json::Value;
use std::{
    fs::File,
    io::{self, BufReader, Error, ErrorKind},
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

    println!("Successfully processed file.");
}

fn read_from_file(file_path: PathBuf) -> Result<Value, io::Error> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let reader = BufReader::new(file);
    let v = serde_json::from_reader(reader)?;

    Ok(v)
}

fn write_to_file(
    file_path: PathBuf,
    json_data: &mut Value,
    should_pretty_print: bool,
) -> Result<(), io::Error> {
    let file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

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
        Value::Object(_) => remove_null_values_from_object(json_data),
        Value::Array(_) => remove_null_values_from_array(json_data),
        _ => Ok(()),
    }
}

fn remove_null_values_from_object(json_data: &mut Value) -> Result<(), io::Error> {
    let object_map = json_data
        .as_object_mut()
        .ok_or(Error::new(ErrorKind::InvalidInput, "Failed to read object"))?;

    // Collect keys to remove
    let keys_to_remove: Vec<String> = object_map
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
        object_map.remove(&key);
    }

    // Go a lever deeper w recursion
    for value in object_map.values_mut() {
        remove_null_values(value)?;
    }

    Ok(())
}

fn remove_null_values_from_array(json_data: &mut Value) -> Result<(), io::Error> {
    let array = json_data
        .as_array_mut()
        .ok_or(Error::new(ErrorKind::InvalidInput, "Failed to read array"))?;

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
