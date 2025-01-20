use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{self, BufWriter, Write};
use std::net::Ipv4Addr;
use std::path::PathBuf;

const IPV4_RANGE: u64 = 4_294_967_296;
const MAX_FILE_SIZE: usize = 20 * 1024 * 1024;

fn u32_to_ipv4_string(ip: u64) -> String {
    let ipv4 = Ipv4Addr::from(ip as u32);
    ipv4.to_string()
}

fn get_output_dir() -> io::Result<PathBuf> {
    println!("
    
  ___   ___  __   __  _ _       ___   ___   _  _ 
 |_ _| | _ \\ \\ \\ / / | | |     / __| | __| | \\| |
  | |  |  _/  \\ V /  |_  _|   | (_ | | _|  | .` |
 |___| |_|     \\_/     |_|     \\___| |___| |_|\\_|
                                                 
    OpenSource Project KodakSec
    ");
    println!("Hello! Where do you want to save the files?");
    println!("Enter the folder path (e.g., C:/test):");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(PathBuf::from(input.trim()))
}

fn generate_ipv4_files(output_dir: &PathBuf) -> io::Result<()> {
    create_dir_all(output_dir)?;

    let mut current_file_number = 1;
    let mut current_file_size = 0;
    
    let mut file = create_new_file(output_dir, current_file_number)?;
    let mut writer = BufWriter::new(file);

    for ip in 0..IPV4_RANGE {
        let ip_string = u32_to_ipv4_string(ip);

        writer.write_all(ip_string.as_bytes())?;
        writer.write_all(b"\n")?;

        current_file_size += ip_string.len() + 1;

        if current_file_size >= MAX_FILE_SIZE {
            writer.flush()?;
            current_file_number += 1;
            current_file_size = 0;
            file = create_new_file(output_dir, current_file_number)?;
            writer = BufWriter::new(file);
        }
    }

    writer.flush()?;

    Ok(())
}

fn create_new_file(output_dir: &PathBuf, file_number: u32) -> io::Result<File> {
    let filename = output_dir.join(format!("part{}.txt", file_number));
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename)
}

fn main() {
    match get_output_dir() {
        Ok(output_dir) => {
            println!("Generating files in: {}", output_dir.display());
            if let Err(e) = generate_ipv4_files(&output_dir) {
                eprintln!("Error: {}", e);
            } else {
                println!("Finished generating files in: {}", output_dir.display());
            }
        }
        Err(e) => eprintln!("Error reading input: {}", e),
    }
}
