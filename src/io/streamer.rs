/*
Implementation of the wordlist reader, this were thought
to use intern buffer for velocity.
*/

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;



// Function that clean up spaces and avoid empty lines and comments
pub fn read_wordlist<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> {
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut entries: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        let entry: &str = line.trim();

        if entry.is_empty() || entry.starts_with('#') {
            continue;
        }

        entries.push(entry.to_string());

    }
    Ok(entries)
}

// Tester for streamer.rs
#[cfg(test)]
mod test {
    
}