use anyhow::{bail, Context, Result};
use clap::Parser;
use lz4_flex::decompress_size_prepended;
use std::fs::{write, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;

/// Mozilla's magic header
const MAGIC_HEADER: &[u8] = b"mozLz40\0";

#[derive(Parser)]
#[command(name = "Lizard")]
#[command(version, about, long_about = None)]
struct Cli {
    /// LZ4 file to decompress.
    input_file: PathBuf,

    /// Recipient of the decompressed data.
    output_file: PathBuf,
}

/// Decompresses the input file into the output file assuming it is a Mozilla-flavoured LZ4 file.
fn decompress_command(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    let input_file = File::open(&input_path)?;
    let mut input_reader = BufReader::new(input_file);

    let header = consume_header(&mut input_reader)?;

    check_header(&header)?;

    let input_data = consume_body(&mut input_reader)?;

    let output_data = inflate(&input_data)?;
    write(output_path, &output_data)?;

    Ok(())
}

/// Consumes the header.
fn consume_header<R: Read>(reader: &mut BufReader<R>) -> Result<[u8; 8]> {
    let mut header = [0; 8];
    reader
        .read_exact(&mut header)
        .context("Failed to read the header.")?;

    Ok(header)
}

/// Fully consumes the body.
fn consume_body<R: Read>(reader: &mut BufReader<R>) -> Result<Vec<u8>> {
    let mut body: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut body)
        .context("Failed to read the body.")?;

    Ok(body)
}

/// Takes a LZ4 buffer with size prepended and returns the decompressed data.
fn inflate(input: &[u8]) -> Result<Vec<u8>> {
    let output =
        decompress_size_prepended(input).context("Failed to decompress the input file.")?;

    Ok(output)
}

fn check_header(header: &[u8]) -> Result<()> {
    if header != MAGIC_HEADER {
        bail!("Wrong magic header");
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match decompress_command(cli.input_file, cli.output_file) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}
