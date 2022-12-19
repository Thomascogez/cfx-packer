use std::{io::Write, path::Path};

use clap::Parser;
use zip::ZipWriter;
use colored::*;

mod parser;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short, long, default_value = "resource.zip")]
    output: String,

    #[arg(short, long, default_value = "false")]
    dry_run: bool,
}
fn main() {
    let args = Cli::parse();


    let fx_manifest_path = Path::new(&args.path).join("fxmanifest.lua");
    let fx_manifest_file_content = std::fs::read_to_string(&fx_manifest_path).expect("Could not read file");

    let extracted_path = parser::extract_file_paths(fx_manifest_file_content.as_str());

    let mut resolved_paths = parser::resolve_file_paths(extracted_path, &args.path);
    resolved_paths.push(String::from(fx_manifest_path.to_str().unwrap()));
    resolved_paths.dedup();

    print_packed_files_info(&resolved_paths, &args.path);

    if args.dry_run {
        return;
    }

    let zip_file = std::fs::File::create(&args.output).unwrap();
    let mut zip_writer = ZipWriter::new(zip_file);

    for path in resolved_paths {
        zip_writer.start_file(&path, zip::write::FileOptions::default()).unwrap();
        let relative_path = utils::to_relative_path(&path, &args.path);
        let content = std::fs::read(&relative_path).unwrap();
        zip_writer.write(&content).unwrap();
    }

    zip_writer.finish().unwrap();
    println!("Zip file created at {}", args.output.green().bold());

}


fn print_packed_files_info(packed_files: &Vec<String>, base_path: &String) {
    println!("{}", "=========== Packed files ===========".magenta().bold());
    let mut total_size = 0;
    for file in packed_files {
        let relative_path = utils::to_relative_path(file, base_path);
        let file_path = Path::new(&relative_path);
        
        let file_len = file_path.metadata().unwrap().len();
        total_size += file_len;
        
        println!("{} {}", format!("{:.2} KB", file_len as f64 / 1024.0).yellow(), file_path.to_str().unwrap().green());
    }

    println!("{}", "====================================".magenta().bold());
    println!("\n{} {}", format!("{:.2} KB", total_size as f64 / 1024.0).yellow(), "Total unpacked size".green().bold());

}