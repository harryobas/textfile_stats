use magnus::{define_module, function, prelude::*, Error, exception};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn word_count(file_path: String) -> Result<u32, Error> {

    let file = File::open(file_path)
    .map_err(
        |e| Error::new(exception::io_error(), e.to_string())
    )?;

    let reader = BufReader::new(file);
    let mut word_count = 0u32;

    for line in reader.lines() {
        if let Ok(line) = line {
            word_count += line.split_whitespace().count() as u32;
        }
    }

    Ok(word_count)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("TextfileStats")?;
    module.define_singleton_method("word_count", function!(word_count, 1))?;
    Ok(())
}
