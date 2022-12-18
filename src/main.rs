use std::{io::Write, path::Path};

use clap::Parser;
use zip::ZipWriter;
use colored::*;

mod parser;

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
    let file_content = std::fs::read_to_string(&fx_manifest_path).expect("Could not read file");

    let extracted_path = parser::extract_file_paths(file_content.as_str());

    let mut resolved_paths = parser::resolve_file_paths(extracted_path);
    resolved_paths.push(String::from(fx_manifest_path.to_str().unwrap()));
    resolved_paths.dedup();

    println!("{}", "=========== Packed files ===========".magenta().bold());
    print_packed_files_info(&resolved_paths);
    println!("{}", "====================================".magenta().bold());

    let zip_file = std::fs::File::create(args.output).unwrap();
    let mut zip_writer = ZipWriter::new(zip_file);

    for path in resolved_paths {
        let relative_path = Path::new(&args.path).join(&path);
        zip_writer.start_file(path, zip::write::FileOptions::default()).unwrap();
        let content = std::fs::read(&relative_path).unwrap();
        zip_writer.write(&content).unwrap();
    }

    zip_writer.finish().unwrap();
    println!("Finished writing zip file");

}


fn print_packed_files_info(packed_files: &Vec<String>) {
    let mut total_size = 0;
    for file in packed_files {
        let file_path = Path::new(&file);
        
        let file_len = file_path.metadata().unwrap().len();
        total_size += file_len;
        
        println!("{} {}", format!("{:.2} KB", file_len as f64 / 1024.0).yellow(), file_path.to_str().unwrap().green());
    }

    println!("\n{} {}", format!("{:.2} KB", total_size as f64 / 1024.0).yellow(), "Total size".green().bold().on_green());
}