use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

use csv;
use flate2::read::GzDecoder;
use glob::glob;

use errors::{Result, ResultExt};
use parser;

pub fn process() -> Result<()> {
    let mut args = env::args();
    let folder = args
        .nth(1)
        .chain_err(|| "Specify the directory to work with.")?;
    let destination_file = args
        .next()
        .chain_err(|| "Specify the destination CSV file")?;

    let files: io::Result<Vec<fs::File>> = glob(&format!("{}/**/*.gz", folder))
        .chain_err(|| "Failed to search gziped files")?
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap())
        .map(|path| fs::File::open(path))
        .collect();
    let files = files.chain_err(|| "Failed to open files")?;
    let buffers = files
        .iter()
        .map(|file| GzDecoder::new(file))
        .map(|file| io::BufReader::new(file))
        .collect();

    if destination_file == "-" {
        let output_buffer = io::stdout();
        process_files(buffers, &mut io::BufWriter::new(output_buffer.lock()))?;
    } else {
        let output_buffer = fs::File::create(&destination_file)
            .chain_err(|| format!("Failed to open {}", destination_file))?;
        process_files(buffers, &mut io::BufWriter::new(output_buffer))?;
    }
    Ok(())
}

fn process_files<R, W>(buffers: Vec<R>, output: &mut W) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut csv_writer = csv::Writer::from_writer(output);
    csv_writer
        .write_record(&["date", "verb", "url", "time"])
        .chain_err(|| "Failed to write headers")?;

    for buffer in buffers {
        for line in buffer.lines() {
            let line = line.chain_err(|| "Failed to read a line")?;
            if let Ok((_, (date, verb, url, time))) = parser::parse_log(&line) {
                csv_writer
                    .write_record(&[date, verb, url, time])
                    .chain_err(|| "Failed to write fields")?;
            }
        }
    }
    Ok(())
}
