use assembler::get_without_null_values;
use clap::Parser;
use cleaner::remove_null_values;
use file_handler::{read_from_file, write_to_file};
use std::path::PathBuf;
use std::time::Instant;

mod assembler;
mod cleaner;
mod file_handler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, action)]
    pretty: bool,

    // Slower, rebuilds the json rather than mutating it in memory
    #[arg(short, long, action)]
    assemble: bool,
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

    if args.assemble {
        let data = &mut get_without_null_values(json_data.to_owned())
            .expect("Failed to remove null values, panicking...");

        write_to_file(args.output, data, args.pretty)
            .expect("Failed to write cleaned json to file, panicking...");
    } else {
        remove_null_values(json_data).expect("Failed to remove null values, panicking...");

        write_to_file(args.output, json_data, args.pretty)
            .expect("Failed to write cleaned json to file, panicking...");
    }

    println!(
        "Success! Processed file in {:.4}s",
        now.elapsed().as_secs_f32()
    );
}
