use clap::Parser;
use serde_json::{Result, Value};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, action)]
    pretty: bool,
}

fn main() {
    let args = Args::parse();

    println!(
        "Hello! I'm going to try to read a file from {}, clean it up, and output it to {}",
        args.file, args.output
    );
    let v = &mut read_from_file(args.file).expect("Failed to parse the file contents!");
    remove_null_values(v);
    write_to_file(args.output, v, args.pretty);
}

fn read_from_file(file_path: String) -> Result<Value> {
    let file = File::open(file_path).expect("Could not open file!");
    let reader = BufReader::new(file);
    let v = serde_json::from_reader(reader)?;

    Ok(v)
}

fn write_to_file(path: String, v: &mut Value, should_pretty_print: bool) {
    let file = &mut File::create(path).expect("Could not create file!");

    println!("Should pretty print: {}", should_pretty_print);

    if should_pretty_print {
        serde_json::to_writer_pretty(file, v).expect("Could not write to file");
    } else {
        serde_json::to_writer(file, v).expect("Could not write to file");
    }
}

fn remove_null_values(v: &mut Value) {
    if v.is_object() {
        let object_map = v.as_object_mut().expect("Could not parse object from json");

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
            println!("Removing key {}", key);
            object_map.remove(&key);
        }

        // Go a lever deeper w recursion
        for value in object_map.values_mut() {
            remove_null_values(value);
        }
    } else if v.is_array() {
        let array = v.as_array_mut().expect("Could not parse array from json");

        // Collect indices to remove
        let indices_to_remove: Vec<usize> = array
            .iter()
            .enumerate()
            .filter_map(|(index, value)| if value.is_null() { Some(index) } else { None })
            .collect();

        for index in indices_to_remove.iter().rev() {
            println!("Removing item with index {}", index);
            array.remove(*index);
        }

        // Go a lever deeper w recursion
        for value in array.iter_mut() {
            remove_null_values(value);
        }
    }
}
